use actix_web::{get, post, web, HttpResponse, Responder};
use tokio_postgres::Client;

use crate::models::team_member::{add, get, TeamMember};

#[post("/add_team_member")]
pub async fn add_team_member(
    client: web::Data<Client>,
    team_member: web::Json<TeamMember>,
) -> impl Responder {
    println!("oke");
    match add(&client, &team_member).await {
        Ok(_) => HttpResponse::Ok().json("Team member added successfully"),
        Err(e) => {
            eprintln!("Error adding team member: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get_team_members")]
pub async fn get_team_members(client: web::Data<Client>) -> impl Responder {
    println!("oke");
    match get(&client).await {
        Ok(team_members) => HttpResponse::Ok().json(team_members),
        Err(e) => {
            eprintln!("Error getting team members: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn team_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_team_member);
    cfg.service(get_team_members);
}
