pub use super::prelude::*;

#[async_trait]
pub trait Repository {
    async fn create(&self, user: &User) -> Result<(), ApplicationError>;
    async fn delete(&self);
    async fn update(&self);
    async fn find_by_token(&self);
    async fn find_by_id(&self, uuid: String) -> Result<Option<User>, ApplicationError>;
    async fn find_by_serial_id(&self);
    async fn find_by_email<T: AsRef<str> + Send + Sync>(
        &self,
        email: T,
    ) -> Result<Option<User>, ApplicationError>;
    async fn search_by_name(&self);
    async fn search_by_email(&self);
    async fn search_by_policies(&self);
    async fn search_by_birthday(&self);
}
