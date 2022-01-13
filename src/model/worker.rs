use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{Decode, Encode, SqlitePool};
use uuid::Uuid;
use anyhow::{Result, Context as _};

use super::{Department, Task};

#[derive(Enum, Clone, Copy, PartialEq, Eq, Encode, Decode, Debug)]
#[repr(i64)]
pub enum Role {
    Worker,
    Manager,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Worker {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub created_time: DateTime<Utc>,
    pub role: Role,

    #[graphql(skip)]
    pub department_id: Uuid,
}

impl Worker {
    pub fn new(name: String, last_name: String, department_id: Uuid, role: Role) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            last_name,
            department_id,
            created_time: Utc::now(),
            role,
        }
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Worker {}

#[ComplexObject]
impl Worker {
    async fn formal_name(&self) -> String {
        format!("{}, {}", self.last_name, self.name)
    }
    async fn full_name(&self) -> String {
        format!("{} {}", self.name, self.last_name)
    }
    async fn dependents(&self) -> Option<Vec<&Worker>> {
        match self.role {
            Role::Manager => Some(
                // self.department
                //     .workers
                //     .iter()
                //     .filter(|w| w.role == Role::Worker && *w != self)
                //     .collect(),
                vec![],
            ),
            Role::Worker => None,
        }
    }
    async fn department<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Department> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_as!(
            Department,
            r#"SELECT id as "id!: Uuid", name FROM Departments WHERE id = ?"#,
            self.department_id
        )
        .fetch_one(pool)
        .await.context("")
    }
    async fn tasks<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Task>> {
        let pool = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_as!(
            Task,
            r#"SELECT 
                t.id as "id!: Uuid", 
                t.name, 
                t.description, 
                t.project_id as "project_id!: Uuid" 
            FROM Assignments a 
            JOIN Tasks t 
                ON a.task_id = t.id 
            WHERE 
                a.worker_id = ?"#,
            self.id
        )
        .fetch_all(pool)
        .await.context("")
    }
}
