use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;
use uuid::Uuid;

use std::collections::HashMap;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<String>;
    fn delete_user(&mut self, user_uuid: String);
}

#[derive(Clone)]
pub struct User {
    user_uuid: String,
    username: String,
    password: String,
}

#[derive(Default)]
pub struct UsersImpl {
    uuid_to_user: HashMap<String, User>,
    username_to_user: HashMap<String, User>,
}

impl Users for UsersImpl {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String> {
        // TODO: Check if username already exist. If so return an error.
        if self.username_to_user.get(&username).is_some() {
            return Err("Username already exists.".to_string());
        }

        println!("Salting password");
        let salt = SaltString::generate(&mut OsRng);

        println!("Hashing password");
        let hashed_password = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password.\n{e:?}"))?
            .to_string();

        // Create new user with unique uuid and hashed password.
        println!("Creating new user");
        let user: User = User {
            user_uuid: Uuid::new_v4().to_string(),
            username: username.clone(),
            password: hashed_password,
        };

        // TODO: Add user to `username_to_user` and `uuid_to_user`.
        println!("inserting into HashMap");
        self.uuid_to_user
            .insert(user.user_uuid.clone(), user.clone());
        self.username_to_user.insert(username.clone(), user.clone());
        println!("done");
        Ok(())
    }

    fn get_user_uuid(&self, username: String, password: String) -> Option<String> {
        // Retrieve `User` or return `None` is user can't be found.
        let user: &User = self.username_to_user.get(&username)?;

        // Get user's password as `PasswordHash` instance.
        let hashed_password = user.password.clone();
        let parsed_hash = PasswordHash::new(&hashed_password).ok()?;

        // Verify passed in password matches user's password.
        let result = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash);

        // TODO: If the username and password passed in matches the user's username and password return the user's uuid.
        if result.ok().is_some() {
            return Some(user.user_uuid.clone());
        }
        None
    }

    fn delete_user(&mut self, user_uuid: String) {
        // TODO: Remove user from `username_to_user` and `uuid_to_user`.
        let user = self.uuid_to_user.get(&user_uuid);
        println!("Deleting user - found user: {}", user.is_some());
        if user.is_some() {
            let username = user.unwrap();
            let r = self.username_to_user.remove(&username.username);
            println!("Removing user - found username: {}", r.is_some());
            let r = self.uuid_to_user.remove(&user_uuid);
            println!("Removing uuid: {}", r.is_some());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert_eq!(user_service.uuid_to_user.len(), 1);
        assert_eq!(user_service.username_to_user.len(), 1);
    }

    #[test]
    fn should_fail_creating_user_with_existing_username() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let result = user_service.create_user("username".to_owned(), "password".to_owned());

        assert!(result.is_err());
    }

    #[test]
    fn should_retrieve_user_uuid() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(user_service
            .get_user_uuid("username".to_owned(), "password".to_owned())
            .is_some());
    }

    #[test]
    fn should_fail_to_retrieve_user_uuid_with_incorrect_password() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(user_service
            .get_user_uuid("username".to_owned(), "incorrect password".to_owned())
            .is_none());
    }

    #[test]
    fn should_delete_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        println!(
            "uuid_to_user {}  username_to_user {}",
            user_service.uuid_to_user.len(),
            user_service.username_to_user.len()
        );

        let user_uuid = user_service
            .get_user_uuid("username".to_owned(), "password".to_owned())
            .unwrap();
        println!("user_uuid {}", user_uuid);

        user_service.delete_user(user_uuid);

        println!(
            "uuid_to_user {}  username_to_user {}",
            user_service.uuid_to_user.len(),
            user_service.username_to_user.len()
        );

        assert_eq!(user_service.uuid_to_user.len(), 0);
        assert_eq!(user_service.username_to_user.len(), 0);
    }
}
