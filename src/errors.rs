use std::fmt;

#[derive(Ord,Eq,PartialEq,PartialOrd)]
pub enum LoginError {
    UsernameMissing,
    UsernameNotFound,
    PasswordMissing,
    PasswordIncorrect,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   LoginError::UsernameMissing => "username_missing",
                   LoginError::UsernameNotFound => "username_not_found",
                   LoginError::PasswordMissing => "password_missing",
                   LoginError::PasswordIncorrect => "password_incorrect",
               })
    }
}

impl LoginError {
    pub fn from_strings(errors: &Vec<String>) -> Vec<LoginError> {
        errors.iter().filter_map(|e| LoginError::from_string(e)).collect()
    }

    pub fn to_query(errors: Vec<LoginError>) -> String {
        errors.into_iter().map(|e| format!("error={}", e)).collect::<Vec<String>>().join("&")
    }

    fn from_string(error: &str) -> Option<LoginError> {
        match error {
            "username_missing" => Some(LoginError::UsernameMissing),
            "username_not_found" => Some(LoginError::UsernameNotFound),
            "password_missing" => Some(LoginError::PasswordMissing),
            "password_incorrect" => Some(LoginError::PasswordIncorrect),
            _ => None,
        }
    }

    pub fn get_field(&self) -> &'static str {
        match *self {
            LoginError::UsernameMissing |
            LoginError::UsernameNotFound => "username",
            LoginError::PasswordMissing |
            LoginError::PasswordIncorrect => "password",
        }
    }
}
