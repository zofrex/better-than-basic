extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate params;
extern crate persistent;
extern crate cookie;

use iron::Iron;
use iron::Request;
use iron::IronResult;
use iron::Response;
use iron::Set;
use iron::Chain;
use iron::Plugin;
use iron::modifiers::RedirectRaw;
use iron::headers::SetCookie;
use iron::modifiers::Header;

use router::Router;

use handlebars_iron::HandlebarsEngine;
use handlebars_iron::DirectorySource;
use handlebars_iron::Template;

use params::{Params, Value};

use persistent::Read;

mod users;
use users::Users;
use users::LoginResult;

use std::collections::BTreeMap;

use cookie::Cookie;

fn login_page(request: &mut Request) -> IronResult<Response> {
    let mut data = BTreeMap::new();

    if let Some(&ref cookie_header) = request.headers.get::<iron::headers::Cookie>() {
        match cookie_header.iter()
            .filter_map(|cookie| Cookie::parse(cookie.clone()).ok())
            .find(|cookie| cookie.name() == "session-id") {
            Some(ref session_cookie) => {
                data.insert(String::from("session"), session_cookie.value().to_string());
            }
            None => (),
        }
    }
    let params = request.get_ref::<Params>().unwrap();
    let error = params.find(&["error"]);
    let error = non_empty_string(error);

    if let Some(error) = error {
        data.insert(String::from("error"), error.to_string());
    }

    let mut response = Response::new();
    response.set_mut(Template::new("login", data)).set_mut(iron::status::Ok);
    Ok(response)
}

fn redirect(path: &str) -> Response {
    Response::with((iron::status::Found, RedirectRaw(String::from(path))))
}

fn non_empty_string<'a>(value: Option<&'a Value>) -> Option<&'a String> {
    match value {
        Some(&Value::String(ref contents)) if !contents.is_empty() => Some(contents),
        _ => None,
    }
}

fn process_login(request: &mut Request) -> IronResult<Response> {
    let arc = request.get::<Read<Users>>().unwrap();
    let params = request.get_ref::<Params>().unwrap();

    let username = params.find(&["username"]);
    let password = params.find(&["password"]);

    let username = non_empty_string(username);
    let password = non_empty_string(password);

    match (username, password) {
        (None, _) => {
            return Ok(redirect("/?error=no_username"));
        }
        (Some(_), None) => {
            return Ok(redirect("/?error=no_password"));
        }
        (Some(username), Some(password)) => {
            let users = arc.as_ref();

            match users.login(username, password) {
                LoginResult::UserNotFound => return Ok(redirect("/?error=wrong_username")),
                LoginResult::WrongPassword => return Ok(redirect("/?wrong_password")),
                LoginResult::Correct => {
                    let cookie =
                        Cookie::build("session-id", "hello").http_only(true).finish().to_string();
                    return Ok(Response::with((iron::status::Found,
                                              RedirectRaw(String::from("/?correct")),
                                              Header(SetCookie(vec![cookie])))));
                }
            }
        }
    };
}

fn main() {
    let users = Users::hardcoded();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("templates", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", login_page, "login_page");
    router.post("/login", process_login, "login_submit");

    let mut chain = Chain::new(router);

    chain.link_before(Read::<Users>::one(users));
    chain.link_after(hbse);

    Iron::new(chain).http("localhost:3000").unwrap();
}
