use crate::{
    inputs::{InputTask, InputWorker},
    model::{Department, Role, Task, Worker, Project},
};
use anyhow::{Context as _, Result};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

pub(crate) struct Query;

#[Object]
impl Query {
    async fn search_workers_by_name<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query: String,
    ) -> Result<Vec<Worker>> {
        let db = ctx.data_unchecked::<SqlitePool>();
        sqlx::query_file_as!(Worker, "sql/selectWorker.sql", query)
            .fetch_all(db)
            .await
            .context("")
    }
}

pub(crate) struct Mutation;

#[Object]
impl Mutation {
    async fn create_worker<'ctx>(&self, ctx: &Context<'ctx>, input: InputWorker) -> Result<Worker> {
        let db = ctx.data_unchecked::<SqlitePool>();
        let worker = Worker::new(input.name, input.last_name, input.department, input.role);
        let role = worker.role as i64;
        sqlx::query_file!(
            "sql/insertWorker.sql",
            worker.id,
            worker.name,
            worker.last_name,
            worker.department_id,
            worker.created_time,
            role
        )
        .execute(db)
        .await?;
        Ok(worker)
    }

    async fn create_department<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
    ) -> Result<Department> {
        let db = ctx.data_unchecked::<SqlitePool>();
        let n_department = Department::create(name);
        sqlx::query!(
            r#"INSERT INTO Departments (
            id, name
        ) VALUES (
            ?, ?
        )"#,
            n_department.id,
            n_department.name,
        )
        .execute(db)
        .await?;
        Ok(n_department)
    }

    async fn create_project<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
        desc: String
    ) -> Result<Project> {
        let db = ctx.data_unchecked::<SqlitePool>();
        let n_department = Project::create(Uuid::new_v4(), name, desc);
        sqlx::query!(
            r#"INSERT INTO Project (
            id, name, description
        ) VALUES (
            ?, ?, ?
        )"#,
            n_department.id,
            n_department.name,
            n_department.description
        )
        .execute(db)
        .await?;
        Ok(n_department)
    }

    async fn create_task<'ctx>(&self, ctx: &Context<'ctx>, input: InputTask) -> Result<Task> {
        let pool = ctx.data_unchecked::<SqlitePool>();

        let task = Task::create(input.name, input.description, input.project);

        sqlx::query!(
            r#"INSERT INTO Tasks (
            id, 
            name, 
            description, 
            project_id, 
            progress
        ) VALUES (
            ?, ?, ?, ?, 0
        )"#,
            task.id,
            task.name,
            task.description,
            task.project_id
        )
        .execute(pool)
        .await?;

        for worker in input.workers {
            sqlx::query!(
                r#"INSERT INTO Assignments (
                worker_id,
                task_id
            ) VALUES (
                ?, ?
            )"#,
                worker,
                task.id,
            )
            .execute(pool)
            .await?;
        }

        Ok(task)
    }
}

pub(crate) type GraphQLPracticeSchema = Schema<Query, Mutation, EmptySubscription>;

#[cfg(test)]
mod test {
    use async_graphql::EmptySubscription;

    use crate::schema::{Mutation, GraphQLPracticeSchema, Query};

    #[test]
    #[ignore]
    fn generate_schema() {
        let schema = GraphQLPracticeSchema::build(Query, Mutation, EmptySubscription).finish();
        println!("{}", schema.sdl());
    }
}
