use anyhow::Result;
use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use sea_query::{Expr, Iden, InsertStatement, PostgresQueryBuilder, Query};
use sea_query_binder::{SqlxBinder, SqlxValues};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

#[derive(Iden)]
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

#[derive(sqlx::FromRow, Debug, Deserialize)]
struct ContactFullOutput {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    mobile: String,
    active: bool,
    organisation_id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
pub async fn create_contacts(
    pool: &Pool<Postgres>,
    organisation_ids: &[Option<Uuid>],
) -> Result<()> {
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
    let mut query = Query::insert();
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
}
