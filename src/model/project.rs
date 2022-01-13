use anyhow::{Context as _, Result};
use async_graphql::{ComplexObject, Context, SimpleObject};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::model::Task;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn create(id: Uuid, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

#[ComplexObject]
impl Project {
    async fn tasks<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Task>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_as!(
            Task,
            r#"SELECT 
                id as "id!: Uuid", 
                name, 
                description, 
                project_id as "project_id!: Uuid" 
            FROM Tasks
            WHERE 
                project_id = ?"#,
            self.id
        )
        .fetch_all(pool)
        .await
        .context("")
    }
}
