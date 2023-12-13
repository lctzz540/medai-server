use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use medai::handlers::team_member::team_routes;
use std::{env, fs, sync::Arc};
use tokio_postgres::{Client, Error as PostgresError, NoTls};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn connect_db() -> Result<Client, PostgresError> {
    dotenv::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in the environment");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

async fn init_schema(client: Arc<Client>) -> Result<(), PostgresError> {
    let schema_content =
        fs::read_to_string("./sql/schema.sql").expect("Failed to read schema file");

    client.batch_execute(&schema_content).await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_db().await.expect("Failed to connect to database");
    let client_arc = Arc::new(client);

    let cloned_client = Arc::clone(&client_arc);

    tokio::spawn(async move {
        if let Err(e) = init_schema(cloned_client).await {
            eprintln!("Failed to initialize schema: {}", e);
        } else {
            println!("Schema initialized successfully");
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&client_arc)))
            .service(hello)
            .configure(team_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
