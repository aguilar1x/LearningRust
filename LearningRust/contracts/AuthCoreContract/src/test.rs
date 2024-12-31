#![cfg(test)]

use soroban_sdk::{Env, Address};
use soroban_sdk::testutils::Address as AddressTestUtils;
use crate::{AuthContract, AuthContractClient, roles::Role, permissions::Permission};

// Tests roles 

#[test]
fn test_role_assignments() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create test addresses
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let another_user = Address::generate(&env);

    // Test initial role assignment (admin)
    client.assign_role(
        &admin,
        &admin,
        &Role::Admin
    );

    // Verify admin role was assigned correctly
    let role = client.get_role(&admin);
    assert_eq!(role, Role::Admin);

    // Test admin can assign roles to others
    client.assign_role(
        &admin,
        &user,
        &Role::Seller
    );

    // Verify seller role was assigned correctly
    let role = client.get_role(&user);
    assert_eq!(role, Role::Seller);

    // Test non-admin cannot assign roles
    client.assign_role(
        &user,
        &another_user,
        &Role::Seller
    );

    // Verify role was not assigned by non-admin (should remain Buyer)
    let role = client.get_role(&another_user);
    assert_eq!(role, Role::Buyer);
}

#[test]
fn test_role_checks() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create test addresses
    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);

    // Setup roles
    client.assign_role(&admin, &admin, &Role::Admin);
    client.assign_role(&admin, &seller, &Role::Seller);
    client.assign_role(&admin, &buyer, &Role::Buyer);

    // Verify each role is correctly assigned and retrievable
    assert_eq!(client.get_role(&admin), Role::Admin);
    assert_eq!(client.get_role(&seller), Role::Seller);
    assert_eq!(client.get_role(&buyer), Role::Buyer);
}

// Tests Permissions

#[test]
fn test_permission_assignments() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create test addresses
    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);

    // Setup initial roles
    client.assign_role(&admin, &admin, &Role::Admin);
    client.assign_role(&admin, &seller, &Role::Seller);
    client.assign_role(&admin, &buyer, &Role::Buyer);

    // Test permission assignments
    assert!(client.has_permission(&admin, &Permission::ManageRoles));
    assert!(client.has_permission(&seller, &Permission::CreateProduct));
    assert!(client.has_permission(&buyer, &Permission::Buy));
}

#[test]
fn test_permission_checks() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create test addresses
    let admin = Address::generate(&env);
    let seller = Address::generate(&env);

    // Setup roles
    client.assign_role(&admin, &admin, &Role::Admin);
    client.assign_role(&admin, &seller, &Role::Seller);

    // Verify correct permissions
    assert!(client.has_permission(&admin, &Permission::ManageRoles));
    assert!(client.has_permission(&seller, &Permission::CreateProduct));

    // Verify permission restrictions
    assert!(!client.has_permission(&seller, &Permission::ManageRoles));
    assert!(!client.has_permission(&seller, &Permission::ManageProducts));
}

// Tests Validations

#[test]
fn test_user_action_validation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create test addresses
    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);

    // Setup roles
    client.assign_role(&admin, &admin, &Role::Admin);
    client.assign_role(&admin, &seller, &Role::Seller);
    client.assign_role(&admin, &buyer, &Role::Buyer);

    // Test valid actions - should not panic
    client.validate_action(&seller, &Permission::CreateProduct);
    client.validate_action(&buyer, &Permission::Buy);
    client.validate_action(&admin, &Permission::ManageRoles);

    // Test invalid actions - should panic
    let mut success = false;
    let _result = std::panic::AssertUnwindSafe(|| {
        client.validate_action(&buyer, &Permission::CreateProduct);
        success = true;
    });
    assert!(!success, "Expected unauthorized action to fail");

    let mut success = false;
    let _result = std::panic::AssertUnwindSafe(|| {
        client.validate_action(&seller, &Permission::ManageRoles);
        success = true;
    });
    assert!(!success, "Expected unauthorized action to fail");
}

#[test]
fn test_role_based_access_control() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuthContract);
    let client = AuthContractClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create test addresses
    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let another_seller = Address::generate(&env);

    // Setup initial roles
    client.assign_role(&admin, &admin, &Role::Admin);
    client.assign_role(&admin, &seller, &Role::Seller);

    // Test role-based access control
    client.assign_role(&admin, &another_seller, &Role::Seller);
    assert_eq!(client.get_role(&another_seller), Role::Seller);

    // Seller cannot assign roles
    client.assign_role(&seller, &another_seller, &Role::Buyer);
    assert_ne!(client.get_role(&another_seller), Role::Buyer);

    // Test permission validation
    client.validate_action(&admin, &Permission::ManageProducts);
    
    let mut success = false;
    let _result = std::panic::AssertUnwindSafe(|| {
        client.validate_action(&seller, &Permission::ManageProducts);
        success = true;
    });
    assert!(!success, "Expected unauthorized action to fail");
}