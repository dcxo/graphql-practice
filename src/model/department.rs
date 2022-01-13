use anyhow::{Context as _, Result};
use async_graphql::{ComplexObject, Context, SimpleObject};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, DateTime};

use super::Role;
use super::Worker;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Department {
    pub id: uuid::Uuid,
    pub name: String,
}

impl Department {
    pub fn create(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
        }
    }
}

#[ComplexObject]
impl Department {
    async fn workers<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Worker>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_as!(
            Worker,
            r#"SELECT 
                id as "id!: Uuid", 
                name, 
                last_name, 
                department_id as "department_id!: Uuid", 
                created_time as "created_time!: DateTime<Utc>", 
                role as "role!: Role" 
            FROM Workers WHERE department_id = ?"#,
            self.id
        )
        .fetch_all(pool)
        .await
        .context("")
    }
    async fn managers(&self) -> Result<Vec<&Worker>> {
        todo!()
    }
}
