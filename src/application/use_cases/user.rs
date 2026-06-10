use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};

use crate::application::app_error::AppResult;

struct User {
    username: String,
    email: String,
    password_hash: String,
}

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()>;
}

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
}

#[derive(Clone)]
pub struct UserUseCases {
    persistence: Arc<dyn UserPersistence>,
}

impl UserUseCases {
    pub fn new(persistence: Arc<dyn UserPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn add(&self, username: &str, email: &str, password: &SecretString) -> AppResult<()> {
        info!("Adding user...");

        self.persistence
            .create_user(username, email, password.expose_secret())
            .await?;

        info!("Adding user finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockUserPersistence;

    #[async_trait]
    impl UserPersistence for MockUserPersistence {
        async fn create_user(
            &self,
            username: &str,
            email: &str,
            _password_hash: &str,
        ) -> AppResult<()> {
            assert_eq!(username, "testuser");
            assert_eq!(email, "testuser@gmail.com");

            Ok(())
        }
    }

    // struct MockUserCredentialsHasher;

    // impl UserCredentialsHasher for MockUserCredentialsHasher {
    //     fn hash_password(&self, password: &str) -> AppResult<String> {
    //         Ok(format!("{}_hash", password))
    //     }
    // }

    #[tokio::test]
    async fn add_user_works() {
        let user_use_cases = UserUseCases::new(Arc::new(MockUserPersistence));

        let result = user_use_cases
            .add("testuser", "testuser@gmail.com", &"testuser_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
