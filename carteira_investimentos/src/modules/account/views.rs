use askama::Template;

use crate::error::AppError;

// Página de login
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {
    pub error_msg: String
}

impl LoginPage {
    pub fn new(error_msg: String) -> Self {
        Self {
            error_msg
        }        
    }

    pub fn html(self) -> Result<String, AppError> {
        Ok(self.render()?)
    }
}

// Página de criação de conta
#[derive(Template)]
#[template(path = "new_account.html")]
pub struct NewAccountPage {
    pub error_msg: String
}

impl NewAccountPage {
    pub fn new(error_msg: String) -> Self {
        Self {
            error_msg
        }
    }

    pub fn html(self) -> Result<String, AppError> {
        Ok(self.render()?)
    }
}
