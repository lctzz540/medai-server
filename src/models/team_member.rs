use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{Client, Error as PostgresError};

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "team")]
pub struct TeamMember {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub units: String,
    pub description: String,
    pub avatar: String,
    pub github: Option<String>,
    pub facebook: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub envelope: Option<String>,
}

pub async fn add(client: &Client, team_member: &TeamMember) -> Result<(), PostgresError> {
    client
        .execute(
            "INSERT INTO team (name, role, units, description, avatar, github, facebook, linkedin, twitter, envelope)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            &[&team_member.name, &team_member.role, &team_member.units, &team_member.description,
              &team_member.avatar, &team_member.github, &team_member.facebook, &team_member.linkedin,
              &team_member.twitter, &team_member.envelope]
        )
        .await?;

    Ok(())
}

pub async fn get(client: &Client) -> Result<Vec<TeamMember>, PostgresError> {
    let rows = client.query("SELECT * FROM team", &[]).await?;

    let mut team_members = Vec::new();

    for row in rows {
        let team_member = TeamMember {
            id: row.get("id"),
            name: row.get("name"),
            role: row.get("role"),
            units: row.get("units"),
            description: row.get("description"),
            avatar: row.get("avatar"),
            github: row.get("github"),
            facebook: row.get("facebook"),
            linkedin: row.get("linkedin"),
            twitter: row.get("twitter"),
            envelope: row.get("envelope"),
        };

        team_members.push(team_member);
    }

    Ok(team_members)
}
