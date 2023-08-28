use super::GetColFromStr;
use crate::error::{Error, Result};
use crate::models::RowCount;
use crate::query_params::{Filters, StdQueryParams, StdQueryParamsPreSerialize};
use axum::extract::Path;
use axum::{
    debug_handler,
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use fake::Fake;
use sea_query::{Expr, Iden, PostgresQueryBuilder};
use sea_query_binder::{SqlxBinder, SqlxValues};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[debug_handler]
pub async fn show_contact(
    State(pool): State<Pool<Postgres>>,
    Path(contact_id): Path<Uuid>,
) -> Result<Json<ContactFullOutput>> {
    match sqlx::query_as!(
        ContactFullOutput,
        r#"SELECT * FROM contact WHERE "id" = $1"#,
        contact_id
    )
    .fetch_optional(&pool)
    .await?
    {
        Some(contact) => Ok(Json(contact)),
        None => Err(Error::NotFound),
    }
}
#[debug_handler]
pub async fn index_contact(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<StdQueryParamsPreSerialize>,
) -> Result<Json<ContactResponse>> {
    let params_serialized = StdQueryParams::from(params)?;
    let contacts_res = get_contacts(&pool, params_serialized).await?;
    Ok(Json(contacts_res))
}

#[derive(Iden, Debug)]
pub enum Contact {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Mobile,
    Active,
    OrganisationId,
    CreatedAt,
    UpdatedAt,
}

impl GetColFromStr for Contact {
    type Item = Contact;
    fn get_col_from_str(search: &str) -> Option<Self::Item> {
        match search {
            "id" => Some(Contact::Id),
            "first_name" => Some(Contact::FirstName),
            "last_name" => Some(Contact::LastName),
            "email" => Some(Contact::Email),
            "mobile" => Some(Contact::Mobile),
            "active" => Some(Contact::Active),
            "organisation_id" => Some(Contact::OrganisationId),
            "created_at" => Some(Contact::CreatedAt),
            "updated_at" => Some(Contact::UpdatedAt),
            _ => None,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct ContactFullOutput {
    id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: String,
    mobile: String,
    active: Option<bool>,
    organisation_id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

struct ContactInput {
    first_name: String,
    last_name: String,
    email: String,
    mobile: String,
    organisation_id: Option<Uuid>,
    active: bool,
}
#[derive(Serialize)]
pub struct ContactResponse {
    count: i64,
    data: Vec<ContactFullOutput>,
}

pub async fn create_contacts(
    pool: &Pool<Postgres>,
    organisation_ids: &[Option<Uuid>],
) -> anyhow::Result<()> {
    let batch_size = 1_000;
    for batch_index in 0..(organisation_ids.len() / batch_size) {
        let contacts = (0..batch_size)
            .map(|i| get_fake_contact_input(organisation_ids[batch_index * batch_size + i]));
        let (sql, values) = insert_contacts_sql(contacts)?;
        sqlx::query_with(&sql, values).fetch_all(pool).await?;
    }
    Ok(())
}

fn insert_contacts_sql(
    contacts: impl Iterator<Item = ContactInput>,
) -> Result<(String, SqlxValues), sea_query::error::Error> {
    let mut query = sea_query::Query::insert();
    query.into_table(Contact::Table).columns([
        Contact::FirstName,
        Contact::LastName,
        Contact::Email,
        Contact::Mobile,
        Contact::OrganisationId,
        Contact::Active,
    ]);
    for contact in contacts {
        query.values([
            contact.first_name.into(),
            contact.last_name.into(),
            contact.email.into(),
            contact.mobile.into(),
            contact.organisation_id.into(),
            contact.active.into(),
        ])?;
    }
    Ok(query.returning_all().build_sqlx(PostgresQueryBuilder {}))
}
fn get_fake_contact_input(organisation_id: Option<Uuid>) -> ContactInput {
    use fake::faker::boolean::en::*;
    ContactInput {
        first_name: fake::faker::name::en::FirstName().fake(),
        last_name: fake::faker::name::en::LastName().fake(),
        email: fake::faker::internet::en::SafeEmail().fake(),
        mobile: fake::faker::phone_number::en::CellNumber().fake(),
        organisation_id,
        active: Boolean(64).fake(),
    }
}

fn get_contacts_sql(params: &StdQueryParams) -> Result<(String, SqlxValues)> {
    let limit = std::cmp::min(params.limit.unwrap_or(100), 10_000);
    let offset = params.offset.unwrap_or(0);
    let columns = [
        Contact::Id,
        Contact::FirstName,
        Contact::LastName,
        Contact::Email,
        Contact::Mobile,
        Contact::OrganisationId,
        Contact::Active,
        Contact::CreatedAt,
        Contact::UpdatedAt,
    ];
    let mut query = sea_query::Query::select();
    query.from(Contact::Table).columns(columns);
    Contact::add_filters(&mut query, &params.filters)?;
    Contact::add_sorting(&mut query, &params.sorting)?;
    Ok(query
        .limit(limit)
        .offset(offset)
        .build_sqlx(PostgresQueryBuilder {}))
}

fn get_contact_count_sql(filters: &Filters) -> Result<(String, SqlxValues)> {
    let mut query = sea_query::Query::select();
    query
        .expr(Expr::col((Contact::Table, Contact::Id)).count())
        .from(Contact::Table);

    Contact::add_filters(&mut query, filters)?;

    Ok(query.build_sqlx(PostgresQueryBuilder {}))
}

async fn get_contacts(pool: &Pool<Postgres>, params: StdQueryParams) -> Result<ContactResponse> {
    let (sql, values) = get_contacts_sql(&params)?;
    let contacts = sqlx::query_as_with::<_, ContactFullOutput, _>(&sql, values)
        .fetch_all(pool)
        .await?;

    let (sql, values) = get_contact_count_sql(&params.filters)?;
    let count = sqlx::query_as_with::<_, RowCount, _>(&sql, values)
        .fetch_one(pool)
        .await?
        .count;

    Ok(ContactResponse {
        count,
        data: contacts,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insert_contacts_sql_simple() {
        let org_ids = [Some(Uuid::default()), None, None];
        let contacts = (0..3).map(|i| get_fake_contact_input(org_ids[i]));
        let (sql, _) = insert_contacts_sql(contacts).unwrap();

        assert_eq!(sql, String::from(""));
    }

    #[test]
    fn select_contacts_sql_simple() {
        let params = StdQueryParams::from(StdQueryParamsPreSerialize {
            offset: Some(200),
            limit: Some(50),
            sorting: None,
            filters: None,
        })
        .unwrap();
        let (sql, _) = get_contacts_sql(&params).unwrap();
        assert_eq!(
            sql,
            String::from(
                r#"SELECT "first_name", "last_name", "email", "mobile", "organisation_id", "active" FROM "contact" LIMIT $1 OFFSET $2"#
            )
        );
    }
}
