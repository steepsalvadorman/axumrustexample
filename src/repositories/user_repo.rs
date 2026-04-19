use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::models::entities::user::{Entity as UserEntity, Column as UserColumn, Model as User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepoTrait{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sea_orm::DbErr>;
}


#[derive(Clone)]
pub struct PostgresUserRepository {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserRepoTrait for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sea_orm::DbErr> {
        let user = UserEntity::find()
            .filter(UserColumn::Email.eq(email))
            .one(&self.db)
            .await?;

        Ok(user)
    }
}