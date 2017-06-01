extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate urlencoded;
extern crate persistent;
extern crate cookie;
extern crate staticfile;
extern crate mount;
extern crate url;

#[macro_use]
extern crate serde_derive;

use iron::Iron;
use iron::Request;
use iron::IronResult;
use iron::Response;
use iron::Set;
use iron::Chain;
use iron::Plugin;
use iron::modifiers::Redirect;
use iron::modifiers::RedirectRaw;
use iron::headers::SetCookie;
use iron::modifiers::Header;

use router::Router;

use handlebars_iron::HandlebarsEngine;
use handlebars_iron::DirectorySource;
use handlebars_iron::Template;

use urlencoded::{UrlEncodedQuery, UrlEncodedBody};

use persistent::Read;
use persistent::Write;

mod users;
mod sessions;
mod config;
mod i18n;
mod errors;
mod listener;
mod page_data;

use users::Users;
use users::LoginResult;

use cookie::Cookie;

use sessions::Sessions;
use config::Config;

use staticfile::Static;
use mount::Mount;

use iron::Url;
use url::Url as RawUrl;

use errors::LoginError;

use listener::Listener;

use std::collections::HashMap;
use std::collections::BTreeMap;

fn check_auth(request: &mut Request) -> IronResult<Response> {
    let sessions_mutex = request.get::<Write<Sessions>>().unwrap();

    let authorised = match request.headers.get::<iron::headers::Cookie>() {
        Some(&ref cookie_header) => {
            match cookie_header.iter()
                .filter_map(|cookie| Cookie::parse(cookie.clone()).ok())
                .find(|cookie| cookie.name() == "session-id") {
                Some(ref session_cookie) => {
                    sessions_mutex.lock().unwrap().check_session(session_cookie.value())
                }
                None => false,
            }
        }
        None => false,
    };

    if authorised {
        Ok(Response::with(iron::status::Ok))
    } else {
        Ok(Response::with(iron::status::Forbidden))
    }
}

fn login_page(request: &mut Request) -> IronResult<Response> {
    let empty_map = HashMap::new();
    let params = request.get_ref::<UrlEncodedQuery>().unwrap_or(&empty_map);

    let errors = params.get("error").map_or(vec![], LoginError::from_strings);

    let mut form_values: BTreeMap<&str, String> = BTreeMap::new();
    if let Some(redirect) = params.get("return").and_then(one_or_none).and_then(non_empty_string) {
        form_values.insert("return", redirect);
    }

    let i18n = i18n::I18n::new("en");
    let data = i18n.get_catalog(errors, form_values);

    let mut response = Response::new();
    response.set_mut(Template::new("login", data)).set_mut(iron::status::Ok);
    Ok(response)
}

fn success_page(_: &mut Request) -> IronResult<Response> {
    let i18n = i18n::I18n::new("en");
    let data = i18n.get_catalog(vec![], BTreeMap::new());

    let mut response = Response::new();
    response.set_mut(Template::new("success", data)).set_mut(iron::status::Ok);
    Ok(response)
}

fn absolute_from_relative(base: &Url, path: &str) -> Url {
    let base: RawUrl = base.clone().into();
    Url::from_generic_url(base.join(path).unwrap()).unwrap()
}

fn redirect(context: &Request, path: &str) -> Response {
    Response::with((iron::status::Found, Redirect(absolute_from_relative(&context.url, path))))
}

fn non_empty_string(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

fn one_or_none(value: &Vec<String>) -> Option<String> {
    match value.len() {
        1 => Some(value.first().unwrap().clone()),
        _ => None,
    }
}

fn redirect_with_errors(request: &mut Request,
                        errors: Vec<LoginError>,
                        return_url: Option<String>)
                        -> IronResult<Response> {
    let mut path = format!("/?{}", LoginError::to_query(errors));
    if let Some(return_url) = return_url {
        path = format!("{}&return={}", path, return_url);
    }
    Ok(redirect(request, &path))
}

fn process_login(request: &mut Request) -> IronResult<Response> {
    let params = request.get::<UrlEncodedBody>().unwrap();
    let arc = request.get::<Read<Users>>().unwrap();

    let username = params.get("username").and_then(one_or_none).and_then(non_empty_string);
    let password = params.get("password").and_then(one_or_none).and_then(non_empty_string);
    let redirect = params.get("return").and_then(one_or_none).and_then(non_empty_string);

    match (username, password) {
        (None, None) => {
            return redirect_with_errors(request,
                                        vec![LoginError::UsernameMissing,
                                             LoginError::PasswordMissing],
                                        redirect)
        }
        (None, _) => {
            return redirect_with_errors(request, vec![LoginError::UsernameMissing], redirect)
        }
        (Some(_), None) => {
            return redirect_with_errors(request, vec![LoginError::PasswordMissing], redirect)
        }
        (Some(username), Some(password)) => {
            let users = arc.as_ref();

            match users.login(&username, &password) {
                LoginResult::UserNotFound => {
                    return redirect_with_errors(request,
                                                vec![LoginError::UsernameNotFound],
                                                redirect)
                }
                LoginResult::WrongPassword => {
                    return redirect_with_errors(request,
                                                vec![LoginError::PasswordIncorrect],
                                                redirect)
                }
                LoginResult::Correct => {
                    let sessions_mutex = request.get::<Write<Sessions>>().unwrap();
                    let session_id = sessions_mutex.lock().unwrap().create_session();
                    let cookie = Cookie::build("session-id", session_id)
                        .http_only(true)
                        .path("/")
                        .finish()
                        .to_string();
                    return match redirect {
                        Some(redirect) => {
                            Ok(Response::with((iron::status::Found,
                                               RedirectRaw(redirect),
                                               Header(SetCookie(vec![cookie])))))
                        }
                        None => {
                            Ok(Response::with((iron::status::Found,
                                               Redirect(absolute_from_relative(&request.url,
                                                                               "/success")),
                                               Header(SetCookie(vec![cookie])))))
                        }
                    };
                }
            }
        }
    };
}

fn main() {
    let config = Config::from_file("/etc/better-than-basic/config.toml");
    let listener = Listener::setup(config);
    let users = Users::from_file("/etc/better-than-basic/users.toml");
    let sessions = Sessions::new().unwrap();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("/usr/share/better-than-basic/templates", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", login_page, "login_page");
    router.post("/login", process_login, "login_submit");
    router.get("/success", success_page, "success_page");
    router.get("/check", check_auth, "check_endpoint");

    let mut mount = Mount::new();
    mount.mount("/usr/share/better-than-basic/static/", Static::new("static"));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);

    chain.link_before(Read::<Users>::one(users));
    chain.link_before(Write::<Sessions>::one(sessions));
    chain.link_after(hbse);

    let iron = Iron::new(chain);
    let iron = listener.listen_for(iron);
    iron.unwrap();
}
