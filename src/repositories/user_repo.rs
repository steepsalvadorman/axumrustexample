use sqlx::PgPool;
use crate::models::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepoTrait{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
}


pub struct PostgresUserRepository {
    pub pool: PgPool,
}

#[async_trait]
impl UserRepoTrait for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password FROM usuarios WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}