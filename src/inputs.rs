use async_graphql::InputObject;
use uuid::Uuid;

use crate::model::Role;

#[derive(InputObject)]
pub(crate) struct InputWorker {
    pub name: String,
    pub last_name: String,
    pub department: Uuid,
    pub role: Role
}

#[derive(InputObject)]
pub(crate) struct InputTask {
    pub name: String,
    pub description: String,
    pub workers: Vec<Uuid>,
    pub project: Uuid
}