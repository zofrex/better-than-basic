use std::collections::BTreeMap;

use errors::LoginError;
use page_data::PageData;

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

                strings.insert("success_title", "Success!");
                strings.insert("success_message", "You are now logged in.");

                errors.insert(LoginError::UsernameMissing, "You must enter a username");
                errors.insert(LoginError::UsernameNotFound,
                              "Could not find a user with that username");
                errors.insert(LoginError::PasswordMissing, "You must enter a password");
                errors.insert(LoginError::PasswordIncorrect, "Incorrect password");
            }
            "fr" => {
                strings.insert("locale", "fr");

                strings.insert("login_title", "Identification");
                strings.insert("login_subtitle", "Vous devez s'identifier pour utiliser cette page:");
                strings.insert("username_label", "Nom d'utilisateur:");
                strings.insert("username_placeholder", "nom d'utilisateur");
                strings.insert("password_label", "Mot de passe:");
                strings.insert("password_placeholder", "mot de passe");
                strings.insert("login_button", "Identifier");

                strings.insert("success_title", "Succès!");
                strings.insert("success_message", "Votre compte a été validé.");

                errors.insert(LoginError::UsernameMissing, "Vous devez entrer un nom d'utilisateur");
                errors.insert(LoginError::UsernameNotFound,
                              "Nom d'utilisateur ou mot de passe invalide.");
                errors.insert(LoginError::PasswordMissing, "Vous devez entrer un mot de passe");
                errors.insert(LoginError::PasswordIncorrect, "Nom d'utilisateur ou mot de passe invalide.");
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

    pub fn get_catalog(&self,
                       errors: Vec<LoginError>,
                       form_data: BTreeMap<&'static str, String>)
                       -> PageData {
        let error_messages = errors.into_iter()
            .map(|e| (e.get_field(), *self.errors.get(&e).unwrap()))
            .collect();

        PageData {
            i18n: self.strings.clone(),
            errors: error_messages,
            form_data: form_data,
        }
    }
}
