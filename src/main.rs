use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder};
use medai::handlers;
use std::env;
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
        } else {
            println!("Connected to the database");
        }
    });

    Ok(client)
}

// async fn init_schema(client: Arc<Client>) -> Result<(), PostgresError> {
//     let schema_content = fs::read_to_string("schema.sql").expect("Failed to read schema file");
//
//     client.batch_execute(&schema_content).await?;
//
//     println!("Schema initialized successfully");
//
//     Ok(())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_db()
        .await
        .expect("Failed to connect to the database");
    let _client_data = web::Data::new(client);

    let app = move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(hello)
            .service(
                actix_files::Files::new("/static/avatar", "./static/avatar").show_files_listing(),
            )
            .service(
                actix_files::Files::new("/static/carosel", "./static/carosel").show_files_listing(),
            )
            .configure(handlers::team_member::team_routes)
            .app_data(web::Data::clone(&_client_data))
    };

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
