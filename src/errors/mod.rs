use serde::{Serialize, Deserialize};
use crate::types::User;

pub type SingUpResult = Result<User, SingUpError>;

#[derive(Serialize, Deserialize)]
pub enum SingUpError{
    NoUser,
    WrongPassword
}

pub type RegisterResult = Result<User, Vec<SingUpError>>;

#[derive(Serialize, Deserialize)]
pub enum RegistrationError{
    InvalidSchoolNumber,
    InvalidClass,
    InvalidUsername,
    InvalidPassword,
    InvalidEmail,
    InvalidLocation,
    UserExists
}
