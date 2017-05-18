extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate params;
extern crate persistent;

use iron::Iron;
use iron::Request;
use iron::IronResult;
use iron::Response;
use iron::Set;
use iron::Chain;
use iron::Plugin;
use iron::modifiers::RedirectRaw;
use iron::typemap::Key;

use router::Router;

use handlebars_iron::HandlebarsEngine;
use handlebars_iron::DirectorySource;
use handlebars_iron::Template;

use params::{Params, Value};

use persistent::Read;

pub struct Password {
    username: String,
    password: String,
}

#[derive(Copy, Clone)]
pub struct PasswordsKey;
impl Key for PasswordsKey {
    type Value = Vec<Password>;
}

fn login_page(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(Template::new("login", false)).set_mut(iron::status::Ok);
    Ok(response)
}

fn process_login(request: &mut Request) -> IronResult<Response> {
    let arc = request.get::<Read<PasswordsKey>>().unwrap();
    let params = request.get_ref::<Params>().unwrap();

    let username = params.find(&["username"]);
    let password = params.find(&["password"]);

    let username = match username {
        Some(&Value::String(ref username)) => Some(username),
        _ => None,
    };

    let password = match password {
        Some(&Value::String(ref password)) => Some(password),
        _ => None,
    };

    match (username, password) {
        (None, _) => {
            return Ok(Response::with((iron::status::Found,
                                      RedirectRaw(String::from("/?error=no_username")))));
        }
        (Some(_), None) => {
            return Ok(Response::with((iron::status::Found,
                                      RedirectRaw(String::from("/?error=no_password")))));
        }
        (Some(username), Some(password)) => {
            let passwords = arc.as_ref();

            match passwords.iter().find(|p| username == &p.username) {
                None => {
                    return Ok(Response::with((iron::status::Found,
                                              RedirectRaw(String::from("/?error=wrong_username")))))
                }
                Some(login) => {
                    if &login.password == password {
                        return Ok(Response::with((iron::status::Found,
                                                  RedirectRaw(String::from("/?correct")))));
                    } else {
                        return Ok(Response::with((iron::status::Found,
                                                  RedirectRaw(String::from("/?wrong_password")))));
                    }
                }
            }
        }
    };
}

fn main() {
    let passwords = vec![Password {
                             username: String::from("zoe"),
                             password: String::from("password"),
                         }];

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("templates", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", login_page, "login_page");
    router.post("/login", process_login, "login_submit");

    let mut chain = Chain::new(router);

    chain.link_before(Read::<PasswordsKey>::one(passwords));
    chain.link_after(hbse);

    Iron::new(chain).http("localhost:3000").unwrap();
}
