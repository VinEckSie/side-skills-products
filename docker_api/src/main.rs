use std::env;
use axum::{response::Html, routing::get, Router};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    //Load envs
    dotenvy::dotenv().ok();
    
    // for (key, value) in std::env::vars() {
    //     println!("{key} = {value}");
    // }
    
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("DB connect failed");

    let row: (i32,) = sqlx::query_as("SELECT 42")
        .fetch_one(&pool)
        .await
        .expect("Query failed");

    println!("✅ DB Connected — query result: {}", row.0);
    
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("Hello, World UPDATED!")
}

//Docker for development
// # 1. Build your development image (only needed if not using docker-compose)
// docker build -f Dockerfile.dev -t my-app-dev .
// 
// # 2. Start the app + DB + other services
// docker-compose up               # or
// docker-compose up --build      # if Dockerfile.dev changed
// 
// # 3. Enter a container (optional)
// docker exec -it <container-name> bash
// 
// # 4. Stop all services
// docker-compose down


//Docker for production
// # 1. Build your production image
// docker build -f Dockerfile -t my-app-prod .
// 
// # 2. Run the app manually (without compose)
// docker run -p 80:80 my-app-prod
// 
// # OR use a production docker-compose file
// docker-compose -f docker-compose.prod.yml up --build






