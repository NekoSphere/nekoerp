use super::prelude::*;

#[derive(Debug, Default)]
pub struct UseCaseRepository<R: RepoBounds>(pub R);

#[derive(Debug, Default)]
pub struct UseCaseService<S: ServiceBounds>(pub S);
#[derive(Debug, Default)]
pub struct UseCaseCache<R: RepoBounds>(pub R);

#[derive(Debug, Default)]
pub struct UseCase<R: RepoBounds, S: ServiceBounds> {
    //pub aes:
    pub cache: R,
    pub repository: R,
    pub service: S,
}

impl<R: RepoBounds, S: ServiceBounds> UseCase<R, S> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cache(mut self, new: UseCaseCache<R>) -> Self {
        self.cache = new.0;
        self
    }
    pub fn repository(mut self, new: UseCaseRepository<R>) -> Self {
        self.repository = new.0;
        self
    }
    pub fn service(mut self, new: UseCaseService<S>) -> Self {
        self.service = new.0;
        self
    }
    pub fn state(self) -> Arc<Self> {
        Arc::new(self)
    }
}
