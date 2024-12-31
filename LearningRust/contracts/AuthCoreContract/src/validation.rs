use soroban_sdk::{Address, Env};
use crate::types::Error;
use crate::roles::Role;
use crate::permissions::{Permission, PermissionManager};

pub struct ValidationManager;

impl ValidationManager {
    // Validate user exists and is active
    pub fn validate_user(env: &Env, user: &Address) -> Result<(), Error> {
        match env.storage().instance().get::<Address, Role>(user) {
            Some(_) => Ok(()),
            None => Err(Error::UserNotFound),
        }
    }

    // Validate user has required role
    pub fn validate_role(env: &Env, user: &Address, required_role: &Role) -> Result<(), Error> {
        Self::validate_user(env, user)?;
        
        match env.storage().instance().get::<Address, Role>(user) {
            Some(user_role) => {
                if user_role == *required_role {
                    Ok(())
                } else {
                    Err(Error::InvalidRole)
                }
            },
            None => Err(Error::UserNotFound),
        }
    }

    // Validate user has required permissions
    pub fn validate_action(
        env: &Env,
        user: &Address,
        required_permissions: &[Permission],
    ) -> Result<(), Error> {
        Self::validate_user(env, user)?;
        PermissionManager::validate_permissions(env, user, required_permissions)
    }

    // Validate admin action
    pub fn validate_admin_action(env: &Env, admin: &Address) -> Result<(), Error> {
        Self::validate_role(env, admin, &Role::Admin)
    }

    // Validate seller action
    pub fn validate_seller_action(
        env: &Env,
        seller: &Address,
        permission: Permission,
    ) -> Result<(), Error> {
        Self::validate_role(env, seller, &Role::Seller)?;
        PermissionManager::has_permission(env, seller, &permission)
            .and_then(|has_permission| {
                if has_permission {
                    Ok(())
                } else {
                    Err(Error::NotAuthorized)
                }
            })
    }

    // Validate buyer action
    pub fn validate_buyer_action(
        env: &Env,
        buyer: &Address,
        permission: Permission,
    ) -> Result<(), Error> {
        Self::validate_role(env, buyer, &Role::Buyer)?;
        PermissionManager::has_permission(env, buyer, &permission)
            .and_then(|has_permission| {
                if has_permission {
                    Ok(())
                } else {
                    Err(Error::NotAuthorized)
                }
            })
    }

    // Validate complex action requiring multiple permissions
    pub fn validate_complex_action(
        env: &Env,
        user: &Address,
        required_role: &Role,
        required_permissions: &[Permission],
    ) -> Result<(), Error> {
        Self::validate_role(env, user, required_role)?;
        Self::validate_action(env, user, required_permissions)
    }
}
