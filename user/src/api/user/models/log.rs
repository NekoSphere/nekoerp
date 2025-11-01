pub use super::prelude::*;

#[derive(Debug, Builder, NekoPrint, Clone)]
#[transporter(async fn trans() { transporter(message).await; })]
pub struct Log {
    pub uri: Uri,
    pub method: Method,
    pub status: StatusCode,
}
