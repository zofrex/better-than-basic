extern crate iron;
extern crate router;
extern crate handlebars_iron;
extern crate params;
extern crate persistent;
extern crate cookie;
extern crate hyperlocal;
extern crate hyper;
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
use iron::headers::SetCookie;
use iron::modifiers::Header;

use router::Router;

use handlebars_iron::HandlebarsEngine;
use handlebars_iron::DirectorySource;
use handlebars_iron::Template;

use params::{Params, Value};

use persistent::Read;
use persistent::Write;

mod users;
mod sessions;
mod config;
mod i18n;
mod errors;

use users::Users;
use users::LoginResult;

use cookie::Cookie;

use sessions::Sessions;
use hyperlocal::UnixSocketListener;
use config::Config;
use std::net::TcpListener;
use hyper::net::HttpListener;
use iron::Protocol;

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

use staticfile::Static;
use mount::Mount;

use iron::Url;
use url::Url as RawUrl;

use errors::LoginError;

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
    let params = request.get_ref::<Params>().unwrap();
    let error = params.find(&["error"]);
    let error = non_empty_string(error);

    let mut errors: Vec<LoginError> = vec![];
    if let Some(error) = error {
        errors = LoginError::from_strings(vec![error]);
    }

    let i18n = i18n::I18n::new("en");
    let data = i18n.get_catalog(errors);

    let mut response = Response::new();
    response.set_mut(Template::new("login", data)).set_mut(iron::status::Ok);
    Ok(response)
}

fn absolute_from_relative(base: &Url, path: &str) -> Url {
    let base: RawUrl = base.clone().into();
    Url::from_generic_url(base.join(path).unwrap()).unwrap()
}

fn redirect(context: &Request, path: &str) -> Response {
    Response::with((iron::status::Found, Redirect(absolute_from_relative(&context.url, path))))
}

fn non_empty_string<'a>(value: Option<&'a Value>) -> Option<&'a String> {
    match value {
        Some(&Value::String(ref contents)) if !contents.is_empty() => Some(contents),
        _ => None,
    }
}

fn redirect_with_errors(request: &mut Request, errors: Vec<LoginError>) -> IronResult<Response> {
    Ok(redirect(request, &format!("/?{}", LoginError::to_query(errors))))
}

fn process_login(request: &mut Request) -> IronResult<Response> {
    let arc = request.get::<Read<Users>>().unwrap();
    let params = request.get::<Params>().unwrap();

    let username = params.find(&["username"]);
    let password = params.find(&["password"]);

    let username = non_empty_string(username);
    let password = non_empty_string(password);

    match (username, password) {
        (None, None) => {
            return redirect_with_errors(request,
                                        vec![LoginError::UsernameMissing,
                                             LoginError::PasswordMissing])
        }
        (None, _) => return redirect_with_errors(request, vec![LoginError::UsernameMissing]),
        (Some(_), None) => return redirect_with_errors(request, vec![LoginError::PasswordMissing]),
        (Some(username), Some(password)) => {
            let users = arc.as_ref();

            match users.login(username, password) {
                LoginResult::UserNotFound => {
                    return redirect_with_errors(request, vec![LoginError::UsernameNotFound])
                }
                LoginResult::WrongPassword => {
                    return redirect_with_errors(request, vec![LoginError::PasswordIncorrect])
                }
                LoginResult::Correct => {
                    let sessions_mutex = request.get::<Write<Sessions>>().unwrap();
                    let session_id = sessions_mutex.lock().unwrap().create_session();
                    let cookie = Cookie::build("session-id", session_id)
                        .http_only(true)
                        .path("/")
                        .finish()
                        .to_string();
                    return Ok(Response::with((iron::status::Found,
                                              Redirect(absolute_from_relative(&request.url,
                                                                              "/?correct")),
                                              Header(SetCookie(vec![cookie])))));
                }
            }
        }
    };
}

enum Listener {
    UnixSocket(UnixSocketListener),
    Tcp(TcpListener),
}

fn setup_listener(config: Config) -> Listener {
    let path = &config.listen;

    if path.starts_with("/") {
        if let Err(e) = std::fs::remove_file(path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                panic!("Error unlinking Unix socket {}: {}", path, e);
            }
        }

        let l = Listener::UnixSocket(UnixSocketListener::new(path).unwrap());

        if let Some(socket_mode) = config.socket_mode {
            let permissions = Permissions::from_mode(socket_mode);
            std::fs::set_permissions(path, permissions).unwrap();
            println!("Listening on Unix socket {}", path);
        }
        l
    } else {
        let l = Listener::Tcp(TcpListener::bind(path).unwrap());
        println!("Listening on {}", path);
        l
    }
}

fn main() {
    let config = Config::from_file("config.toml");
    let listener = setup_listener(config);
    let users = Users::from_file("users.toml");
    let sessions = Sessions::new().unwrap();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("templates", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", login_page, "login_page");
    router.post("/login", process_login, "login_submit");
    router.get("/check", check_auth, "check_endpoint");

    let mut mount = Mount::new();
    mount.mount("/static/", Static::new("static"));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);

    chain.link_before(Read::<Users>::one(users));
    chain.link_before(Write::<Sessions>::one(sessions));
    chain.link_after(hbse);

    let iron = Iron::new(chain);
    let iron = match listener {
        Listener::UnixSocket(listener) => iron.listen(listener, Protocol::http()),
        Listener::Tcp(listener) => iron.listen(HttpListener::from(listener), Protocol::http()),
    };
    iron.unwrap();
}
