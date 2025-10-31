use super::prelude::*;

#[derive(Builder, Debug)]
pub struct InMemoryReposiroty {
    pub data: RwLock<Vec<User>>,
}

#[async_trait]
impl Repository for InMemoryReposiroty {
    async fn create(&self, user: &User) -> Result<(), ApplicationError> {
        let mut data = self.data.write().await;
        data.push(user.clone());
        Ok(())
    }

    async fn find_by_email<S>(&self, email: S) -> Result<Option<User>, ApplicationError>
    where
        S: Into<String> + Send,
    {
        let data = self.data.read().await;
        let email = email.into();
        Ok(data.iter().find(|u| u.email == email).cloned())
    }

    async fn count(&self) -> Result<usize, ApplicationError> {
        let data = self.data.read().await;
        Ok(data.len())
    }
    async fn find_by_id(&self, uuid: String) -> Result<Option<User>, ApplicationError> {
        let data = self.data.read().await;
        Ok(data.iter().find(|u| u.uuid.to_string() == uuid).cloned())
    }
    async fn delete(&self) {}
    async fn update(&self) {}
    async fn find_by_token(&self) {}
    async fn find_by_serial_id(&self) {}
    async fn search_by_name(&self) {}
    async fn search_by_email(&self) {}
    async fn search_by_policies(&self) {}
    async fn search_by_birthday(&self) {}
}
