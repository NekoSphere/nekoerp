use super::prelude::*;

#[derive(NekoPrint, Builder, Debug, Clone)]
pub struct InMemoryReposiroty {
    pub data: Vec<User>,
}

#[async_trait]
impl Repository for InMemoryReposiroty {
    async fn create(&self, user: &User) -> Result<(), ApplicationError> {
        Ok(())
    }
    async fn delete(&self) {}
    async fn update(&self) {}
    async fn find_by_email<T: AsRef<str> + Send + Sync>(
        &self,
        email: T,
    ) -> Result<Option<User>, ApplicationError> {
        let user = self
            .data
            .iter()
            .find(|u| u.email == email.as_ref())
            .cloned();
        Ok(user)
    }
    async fn find_by_token(&self) {}
    async fn find_by_id(&self, uuid: String) -> Result<Option<User>, ApplicationError> {
        let user = self
            .data
            .iter()
            .find(|u| u.uuid.to_string() == uuid)
            .cloned();
        Ok(user)
    }
    async fn find_by_serial_id(&self) {}
    async fn search_by_name(&self) {}
    async fn search_by_email(&self) {}
    async fn search_by_policies(&self) {}
    async fn search_by_birthday(&self) {}
}
