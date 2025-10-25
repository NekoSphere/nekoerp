use super::prelude::*;

pub async fn email_exists<C, R>(cache: &C, repo: &R, email: &str) -> Result<bool, ApplicationError>
where
    C: Repository + Sync,
    R: Repository + Sync,
{
    if cache.find_by_email(email).await?.is_some() {
        return Ok(true);
    }
    if repo.find_by_email(email).await?.is_some() {
        return Ok(true);
    }
    Ok(false)
}
