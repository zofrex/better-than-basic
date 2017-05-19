extern crate iron;
use iron::typemap::Key;

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
    pub fn hardcoded() -> Users {
        Users {
            users: vec![User {
                            username: String::from("zoe"),
                            password: String::from("password"),
                        }],
        }
    }

    pub fn login(&self, username: &str, password: &str) -> LoginResult {
        match self.users.iter().find(|user| user.username == username) {
            None => LoginResult::UserNotFound,
            Some(user) if user.password == password => LoginResult::Correct,
            Some(_) => LoginResult::WrongPassword,
        }
    }
}
