extern crate hyperlocal;
extern crate hyper;

use iron;
use self::hyperlocal::UnixSocketListener;
use self::hyper::net::HttpListener;
use std::net::TcpListener;

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;

use config::Config;

pub enum Listener {
    UnixSocket(UnixSocketListener),
    Tcp(TcpListener),
}

impl Listener {
    pub fn setup(config: Config) -> Listener {
        let path = &config.listen;

        if path.starts_with("/") {
            if let Err(e) = fs::remove_file(path) {
                if e.kind() != io::ErrorKind::NotFound {
                    panic!("Error unlinking Unix socket {}: {}", path, e);
                }
            }

            let l = Listener::UnixSocket(UnixSocketListener::new(path).unwrap());

            if let Some(socket_mode) = config.socket_mode {
                let permissions = fs::Permissions::from_mode(socket_mode);
                fs::set_permissions(path, permissions).unwrap();
                println!("Listening on Unix socket {}", path);
            }
            l
        } else {
            let l = Listener::Tcp(TcpListener::bind(path).unwrap());
            println!("Listening on {}", path);
            l
        }
    }

    pub fn listen_for<H: iron::Handler>(self,
                                        iron: iron::Iron<H>)
                                        -> iron::error::HttpResult<iron::Listening> {
        match self {
            Listener::UnixSocket(listener) => iron.listen(listener, iron::Protocol::http()),
            Listener::Tcp(listener) => {
                iron.listen(HttpListener::from(listener), iron::Protocol::http())
            }
        }
    }
}
