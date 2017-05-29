extern crate iron;
extern crate toml;
extern crate bcrypt;

use iron::typemap::Key;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use self::toml::value::Value;

use self::bcrypt::verify;

struct User {
    username: String,
    password: String,
}

impl Key for Users {
    type Value = Users;
}

pub struct Users {
    users: Vec<User>,
}

pub enum LoginResult {
    UserNotFound,
    WrongPassword,
    Correct,
}

impl Users {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Users {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let users_table = contents.parse::<Value>().unwrap();
        let users = users_table.as_table()
            .unwrap()
            .iter()
            .map(|(username, password)| {
                User {
                    username: username.clone(),
                    password: String::from(password.as_str().unwrap()),
                }
            })
            .collect::<Vec<User>>();
        Users { users: users }
    }

    pub fn login(&self, username: &str, password: &str) -> LoginResult {
        match self.users.iter().find(|user| user.username == username) {
            None => LoginResult::UserNotFound,
            Some(user) => match verify(password, &user.password) {
                Ok(valid) if valid => LoginResult::Correct,
                _ => LoginResult::WrongPassword,
            }
        }
    }
}
