use soroban_sdk::{contracttype, Address, Env, symbol_short, Vec};
use crate::roles::Role;
use crate::types::Error;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Permission {
    ManageRoles,
    ManageProducts,
    CreateProduct,
    ModifyProduct,
    DeleteProduct,
    CreatePurchase,
    ManageMarket,
    ViewAnalytics,
    Buy,
}

#[allow(dead_code)]
pub struct PermissionManager;

impl PermissionManager {
    // Check if a user has a specific permission
    pub fn has_permission(env: &Env, user: &Address, permission: &Permission) -> Result<bool, Error> {
        match env.storage().instance().get::<Address, Role>(user) {
            Some(role) => {
                Ok(match (role, permission) {
                    (Role::Admin, _) => true,  // Admin has all permissions
                    (Role::Seller, Permission::CreateProduct) => true,
                    (Role::Seller, Permission::ModifyProduct) => true,
                    (Role::Seller, Permission::DeleteProduct) => true,
                    (Role::Seller, Permission::ViewAnalytics) => true,
                    (Role::Buyer, Permission::CreatePurchase) => true,
                    (Role::Buyer, Permission::ViewAnalytics) => true,
                    _ => false,
                })
            },
            None => Err(Error::UserNotFound),
        }
    }

    // Validate multiple permissions for a user
    pub fn validate_permissions(
        env: &Env,
        user: &Address,
        permissions: &[Permission],
    ) -> Result<(), Error> {
        for permission in permissions {
            if !Self::has_permission(env, user, permission)? {
                return Err(Error::NotAuthorized);
            }
        }
        Ok(())
    }

    // Grant specific permission to a role
    pub fn grant_permission(
        env: &Env,
        admin: &Address,
        role: &Role,
        permission: Permission
    ) -> Result<(), Error> {
        if !Self::has_permission(env, admin, &Permission::ManageRoles)? {
            return Err(Error::NotAuthorized);
        }

        let storage_key = symbol_short!("perm");
        let mut role_permissions = env.storage()
            .instance()
            .get::<_, Vec<Permission>>(&storage_key)
            .unwrap_or(Vec::new(env));

        role_permissions.push_back(permission.clone());
        env.storage().instance().set(&storage_key, &role_permissions);

        // Emit permission granted event con role clonado
        env.events().publish(
            (symbol_short!("perm_add"), permission),
            role.clone()  // Clonamos el role aquí
        );
        
        Ok(())
    }
}