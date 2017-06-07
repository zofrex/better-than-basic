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
            "nl" => {
            	strings.insert("locale", "nl");
                
                strings.insert("login_title", "Aanmelden");
                strings.insert("login_subtitle", "U dient zich eerst aan te melden:");
                strings.insert("username_label", "Gebruikersnaam:");
                strings.insert("username_placeholder", "gebruikersnaam");
                strings.insert("password_label", "Wachtwoord:");
                strings.insert("password_placeholder", "wachtwoord");
                strings.insert("login_button", "Aanmelden");

                strings.insert("success_title", "Success!");
                strings.insert("success_message", "U bent nu aangemeld.");

                errors.insert(LoginError::UsernameMissing, "U dient een gebruikersnaam in te geven.");
                errors.insert(LoginError::UsernameNotFound,
                              "Geen gebruiker gevonden met deze gebruikersnaam");
                errors.insert(LoginError::PasswordMissing, "U dient een wachtwoord in te geven");
                errors.insert(LoginError::PasswordIncorrect, "Verkeerd wachtwoord");
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
