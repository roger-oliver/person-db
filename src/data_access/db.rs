use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Person {
    pub id: Uuid,
    pub full_name: String,
    pub nickname: String,
    pub birth: NaiveDate,
    pub skills: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddPerson {
    pub full_name: String,
    pub nickname: String,
    pub birth: NaiveDate,
    pub skills: Option<Vec<String>>,
}

pub async fn db_add_person(
    pg_pool: &PgPool,
    person_to_add: &AddPerson,
) -> Result<Person, sqlx::Error> {
    let id = Uuid::new_v4();

    let person = sqlx::query_as::<_, Person>(
        r#"INSERT INTO people
                       (id, full_name, nickname, birth, skills)
                 VALUES($1, $2, $3, $4, $5)
                 RETURNING id, full_name, nickname, birth, skills;"#,
    )
    .bind(id)
    .bind(&person_to_add.full_name)
    .bind(&person_to_add.nickname)
    .bind(person_to_add.birth)
    .bind(&person_to_add.skills)
    .fetch_one(pg_pool)
    .await?;

    Ok(person)
}

pub async fn db_get_people_by_param(
    pg_pool: &PgPool,
    search: String,
) -> Result<Vec<Person>, sqlx::Error> {
    let people = sqlx::query_as::<_, Person>(
        r#"SELECT id, full_name, nickname, birth, skills FROM PEOPLE WHERE search_trgm ILIKE $1;"#,
    )
    .bind(format!("%{}%", search))
    .fetch_all(pg_pool)
    .await?;

    Ok(people)
}

pub async fn db_get_person_by_id(pg_pool: &PgPool, id: Uuid) -> Result<Person, sqlx::Error> {
    let person = sqlx::query_as::<_, Person>(
        r#"SELECT id, full_name, nickname, birth, skills FROM PEOPLE WHERE id = $1;"#,
    )
    .bind(id)
    .fetch_one(pg_pool)
    .await?;

    Ok(person)
}

pub async fn db_bulk_insert(pg_pool: &PgPool, people: Vec<Person>) -> Result<usize, sqlx::Error> {
    if people.is_empty() {
        return Ok(0);
    }

    let data_as_csv = parse_people_to_csv(&people);

    let mut conn = pg_pool.acquire().await?;

    let mut copy_in = conn
        .copy_in_raw("COPY people (id, full_name, nickname, birth, skills) FROM STDIN")
        .await?;

    copy_in.send(data_as_csv.as_bytes()).await?;

    copy_in.finish().await?;

    Ok(people.len())
}

fn parse_people_to_csv(people: &[Person]) -> String {
    let mut csv_data = String::new();

    for person in people {
        csv_data.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            person.id,
            person.full_name,
            person.nickname,
            person.birth,
            parse_vec_to_pg_vec(&person.skills)
        ));
    }

    csv_data
}

fn parse_vec_to_pg_vec(string_vec: &Option<Vec<String>>) -> String {
    match string_vec {
        Some(sv) if !sv.is_empty() => {
            let elements: Vec<String> = sv.iter().map(|el| el.replace("'", "''")).collect();
            format!("{{{}}}", elements.join(","))
        }
        _ => "{}".to_string(),
    }
}
