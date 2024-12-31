use soroban_sdk::{contracterror, contracttype};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NotAuthorized = 1,
    UserNotFound = 2,
    InvalidRole = 3,
    UnauthorizedAccess = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserData {
    pub role: crate::roles::Role,
    pub active: bool,
}
