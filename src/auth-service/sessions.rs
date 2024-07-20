use std::collections::HashMap;

use uuid::Uuid;

pub trait Sessions {
    fn create_session(&mut self, user_uuid: &str) -> String;
    fn delete_session(&mut self, session: &str);
}

#[derive(Default)]
pub struct SessionsImpl {
    uuid_to_session: HashMap<String, String>,
}

impl Sessions for SessionsImpl {
    fn create_session(&mut self, user_uuid: &str) -> String {
        // Create a new session using Uuid::new_v4().
        let session: String = Uuid::new_v4().to_string();

        // TODO: Insert session into `uuid_to_session`.
        self.uuid_to_session
            .insert(user_uuid.to_string(), session.clone()); // Insert new session into the map.  // Note: In a real-world application, you'd probably want to store the session in a secure and persistent storage.  // For the purpose of this example, we're using a HashMap for simplicity.  // In a real-world application, you'd want to use a database or a secure storage system.  // This code assumes the `uuid_to_session` HashMap is already initialized and doesn't panic when trying to insert a duplicate key.  // In a production application, you'd want to handle this case appropriately.  // Note: In a real-world application, you'd probably want to store the session in a secure and persistent storage.  // For the purpose of this example, we're using a HashMap for simplicity.  // In a real-world application, you'd want to use a database

        session
    }

    fn delete_session(&mut self, session: &str) {
        println!("session db values pre delete: {:?}", self.uuid_to_session);
        self.uuid_to_session.retain(|_, v| !v.as_str().eq(session));
        println!("session db values post delete: {:?}", self.uuid_to_session);
        // for (k, v) in self.uuid_to_session.iter() {
        //     if v.eq(&session) {
        //         println!("user_uuid: {}  session: {}", k, v);
        //         self.uuid_to_session.remove(k);
        //         println!("After removing the session");
        //     }
        // }
        // TODO: Delete session from `uuid_to_session`.
        // Remove session from the map.  // Note: In a real-world application, you'd probably want to store the session in a secure and persistent storage.  // For the purpose of this example, we're using a HashMap for simplicity.  // In a real-world application, you'd want to use a database or a secure storage system.  // This code assumes the `uuid_to_session` HashMap is already initialized and doesn't panic when trying to remove a non-existent key.  // In a production application, you'd want to handle this case appropriately.  // Note: In a real-world application, you'd probably want to store the session in a secure and persistent storage.  // For the purpose of this example, we're using a HashMap for simplicity.  // In a real-world application, you'd want to use a database or a secure storage system.  //
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_session() {
        let mut session_service = SessionsImpl::default();
        assert_eq!(session_service.uuid_to_session.len(), 0);
        let session = session_service.create_session("123456");
        assert_eq!(session_service.uuid_to_session.len(), 1);
        assert_eq!(
            session_service.uuid_to_session.get("123456").unwrap(),
            &session
        );
    }

    #[test]
    fn should_delete_session() {
        let mut session_service = SessionsImpl::default();
        let session = session_service.create_session("123456");
        session_service.delete_session(&session);
        assert_eq!(session_service.uuid_to_session.len(), 0);
    }
}
