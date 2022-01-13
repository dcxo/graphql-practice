use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, routes, Build, Rocket, State};
use schema::{Mutation, GraphQLPracticeSchema, Query};

pub mod inputs;
pub mod model;
mod schema;

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<GraphQLPracticeSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<GraphQLPracticeSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

#[rocket::launch]
async fn rocket_launch() -> Rocket<Build> {
    let pool = sqlx::SqlitePool::connect("sqlite:./local.db")
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let schema = GraphQLPracticeSchema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();

    Rocket::build().manage(schema).mount(
        "/",
        routes![graphql_playground, graphql_query, graphql_request],
    )
}
