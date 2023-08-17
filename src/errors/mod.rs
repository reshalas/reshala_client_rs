use serde::{Serialize, Deserialize};
use crate::types::User;

pub type SingUpResult = Result<User, SingUpError>;

#[derive(Serialize, Deserialize, Debug)]
pub enum SingUpError{
    NoUser,
    WrongPassword,
    NoUsernameHeader,
    NoPasswordHeader,
    NoHeaders

}
pub type RegisterResult = Result<User, Vec<RegistrationError>>;

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
