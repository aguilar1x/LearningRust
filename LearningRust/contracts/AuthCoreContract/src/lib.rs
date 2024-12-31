use soroban_sdk::{contract, contractimpl, Address, Env};

pub mod roles;
pub mod permissions;
pub mod validation;
pub mod types;
pub mod test;

pub use roles::Role;
pub use permissions::Permission;
pub use types::{Error, UserData};

const INITIALIZED_KEY: &str = "initialized";
const ADMIN_EXISTS_KEY: &str = "admin_exists";

#[contract]
pub struct AuthContract;

#[contractimpl]
impl AuthContract {
    pub fn initialize(env: Env) {
        if !env.storage().instance().has(&INITIALIZED_KEY) {
            env.storage().instance().set(&INITIALIZED_KEY, &true);
        }
    }

    pub fn assign_role(env: Env, admin: Address, user: Address, role: Role) {
        let has_any_admin = env.storage().instance()
            .get(&ADMIN_EXISTS_KEY)
            .unwrap_or(false);

        let can_assign = if !has_any_admin && role == Role::Admin {
            env.storage().instance().set(&ADMIN_EXISTS_KEY, &true);
            true
        } else {
            env.storage().instance()
                .get::<Address, UserData>(&admin)
                .map(|data| data.role == Role::Admin)
                .unwrap_or(false)
        };

        if can_assign {
            let user_data = UserData {
                role,
                active: true,
            };
            env.storage().instance().set(&user, &user_data);
        }
    }

    pub fn get_role(env: Env, user: Address) -> Role {
        env.storage().instance()
            .get::<Address, UserData>(&user)
            .map(|data| data.role)
            .unwrap_or(Role::Buyer)
    }

    pub fn has_permission(env: Env, user: Address, permission: Permission) -> bool {
        let role = Self::get_role(env.clone(), user);
        match role {
            Role::Admin => true,
            Role::Seller => match permission {
                Permission::CreateProduct | Permission::Buy => true,
                _ => false,
            },
            Role::Buyer => matches!(permission, Permission::Buy),
        }
    }

    pub fn validate_action(env: Env, user: Address, permission: Permission) -> Result<(), Error> {
        if Self::has_permission(env.clone(), user, permission) {
            Ok(())
        } else {
            Err(Error::UnauthorizedAccess)
        }
    }
}
