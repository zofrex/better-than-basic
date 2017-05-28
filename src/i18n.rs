use std::collections::BTreeMap;

use errors::LoginError;

pub struct I18n {
    strings: BTreeMap<&'static str, &'static str>,
    errors: BTreeMap<LoginError, &'static str>,
}

impl I18n {
    pub fn new(locale: &str) -> I18n {
        let mut strings = BTreeMap::new();
        let mut errors = BTreeMap::new();

        match locale {
            "en" => {
                strings.insert("locale", "en");
                strings.insert("login_title", "Login");
                strings.insert("login_subtitle", "You need to login to access this page:");
                strings.insert("username_label", "Username:");
                strings.insert("username_placeholder", "username");
                strings.insert("password_label", "Password:");
                strings.insert("password_placeholder", "password");
                strings.insert("login_button", "Login");

                errors.insert(LoginError::UsernameMissing, "You must enter a username");
                errors.insert(LoginError::UsernameNotFound,
                              "Could not find a user with that username");
                errors.insert(LoginError::PasswordMissing, "You must enter a password");
                errors.insert(LoginError::PasswordIncorrect, "Incorrect password");
            }
            _ => {
                panic!("Invalid locale");
            }
        }

        I18n {
            strings: strings,
            errors: errors,
        }
    }

    pub fn get_catalog(&self, errors: Vec<LoginError>) -> BTreeMap<&str, BTreeMap<&str, &str>> {
        let mut data = BTreeMap::new();
        data.insert("i18n", self.strings.clone());

        let error_messages = errors.into_iter()
            .map(|e| (e.get_field(), *self.errors.get(&e).unwrap()))
            .collect();

        data.insert("errors", error_messages);
        data
    }
}
