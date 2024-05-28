extern crate anyhow;
use std::env;

use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptySubscription, Schema,
};
use async_graphql_rocket::{ GraphQLQuery, GraphQLRequest, GraphQLResponse };
use dotenv::dotenv;
use rocket::{response::content, routes, State};

mod db;
mod models;

use db::{Pool, PoolManager};

use models::{QueryRoot, MutationRoot};

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[rocket::get("/hello")]
async fn hello(_schema: &State<MySchema>) -> String {
  "🚀 says hello!".to_string()
}

#[rocket::get("/")]
fn graphql_playground() -> content::RawHtml<String> {
  content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<MySchema>, query: GraphQLQuery) -> GraphQLResponse {
  query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<MySchema>, request: GraphQLRequest) -> GraphQLResponse {
  request.execute(schema.inner()).await
}

#[rocket::main]
async fn main() {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let mgr = PoolManager { url: database_url };
  let db_pool = Pool::new(mgr, 16);
  let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
    .data(db_pool)
    .finish();
  rocket::build()
    .manage(schema)
    .mount(
      "/",
      routes![
        graphql_query,
        graphql_request,
        graphql_playground,
        hello
      ],
    )
    .launch()
    .await
    .unwrap();
}
