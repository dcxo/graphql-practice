use anyhow::{Context as _, Result};
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::{Role, Worker, project::Project};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: String,

    #[graphql(skip)]
    pub project_id: Uuid,
}

impl Task {
    pub fn create(name: String, description: String, project_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            project_id,
        }
    }
}

#[ComplexObject]
impl Task {
    async fn workers<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Worker>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_as!(
            Worker,
            r#"SELECT 
                w.id as "id!: Uuid",
                w.name,
                w.last_name,
                w.department_id as "department_id!: Uuid",
                w.created_time as "created_time!: DateTime<Utc>",
                w.role as "role!: Role"
            FROM Assignments a JOIN Workers w ON a.worker_id = w.id WHERE a.task_id = ?"#,
            self.id
        )
        .fetch_all(pool)
        .await
        .context("")
    }
    async fn project<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Project> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_as!(
            Project,
            r#"SELECT 
                id as "id!: Uuid",
                name,
                description
            FROM Project WHERE id = ?"#,
            self.project_id
        )
        .fetch_one(pool)
        .await
        .context("")
    }
}
