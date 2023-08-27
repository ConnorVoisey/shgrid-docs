use std::collections::HashMap;

use crate::error::{Error, Result};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use chrono::{offset, DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use sea_query::{Expr, Iden, InsertStatement, PostgresQueryBuilder};
use sea_query_binder::{SqlxBinder, SqlxValues};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Iden, Debug)]
enum Contact {
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

fn get_contact_col_from_str(col: &str) -> Option<Contact> {
    match col {
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

struct ContactInput {
    first_name: String,
    last_name: String,
    email: String,
    mobile: String,
    organisation_id: Option<Uuid>,
    active: bool,
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

fn get_contacts_sql(params: GetContactParams) -> Result<(String, SqlxValues)> {
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
    let filters = match params.filters {
        Some(filters) => serde_json::from_str::<Vec<(String, String)>>(&filters)?,
        None => vec![],
    };
    let sorting = match params.sorting {
        Some(sorting) => serde_json::from_str::<Vec<(String, String)>>(&sorting)?,
        None => vec![],
    };
    let mut query = sea_query::Query::select();
    query.from(Contact::Table).columns(columns);

    for filter in filters {
        let col = match get_contact_col_from_str(&filter.0) {
            Some(val) => val,
            None => {
                return Err(Error::UnprocessableEntity {
                    errors: HashMap::from([(
                        "filter".to_owned(),
                        vec![format!("invalid column name: {}", filter.0)],
                    )]),
                });
            }
        };
        let value = filter.1;
        query.and_where(Expr::col(col).like(&format!("%{value}%")));
    }
    dbg!(&sorting);
    for sorter in sorting {
        let col = match get_contact_col_from_str(&sorter.0) {
            Some(val) => val,
            None => {
                return Err(Error::UnprocessableEntity {
                    errors: HashMap::from([(
                        "sorting".to_owned(),
                        vec![format!("invalid column name: {}", sorter.0)],
                    )]),
                });
            }
        };
        let direction = match &sorter.1 {
            _ if &sorter.1 == "asc" => sea_query::Order::Asc,
            _ if &sorter.1 == "desc" => sea_query::Order::Desc,
            _ => {
                return Err(Error::UnprocessableEntity {
                    errors: HashMap::from([(
                        "sorting".to_owned(),
                        vec![format!(r#"invalid direction: {}, expecting "asc" or "desc""#, sorter.1)],
                    )]),
                });
            }
        };
        query.order_by(col, direction);
    }
    Ok(query
        .limit(limit)
        .offset(offset)
        .build_sqlx(PostgresQueryBuilder {}))
}
async fn get_contacts(
    pool: &Pool<Postgres>,
    params: GetContactParams,
) -> Result<Vec<ContactFullOutput>> {
    let (sql, values) = get_contacts_sql(params)?;
    Ok(sqlx::query_as_with::<_, ContactFullOutput, _>(&sql, values)
        .fetch_all(pool)
        .await?)
}

#[derive(Deserialize, Debug)]
pub struct GetContactParams {
    limit: Option<u64>,
    offset: Option<u64>,
    sorting: Option<String>,
    filters: Option<String>,
}

#[debug_handler]
pub async fn index_contact(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<GetContactParams>,
) -> Result<Json<Vec<ContactFullOutput>>> {
    let contacts_res = get_contacts(&pool, params).await?;
    Ok(Json(contacts_res))
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
        let params = GetContactParams {
            offset: Some(200),
            limit: Some(50),
            sorting: None,
            filters: None,
        };
        let (sql, _) = get_contacts_sql(params).unwrap();
        assert_eq!(
            sql,
            String::from(
                r#"SELECT "first_name", "last_name", "email", "mobile", "organisation_id", "active" FROM "contact" LIMIT $1 OFFSET $2"#
            )
        );
    }
}