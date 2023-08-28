use chrono::{DateTime, Utc};
use fake::Fake;
use sea_query::{Iden, PostgresQueryBuilder, Query};
use sea_query_binder::{SqlxBinder, SqlxValues};
use serde::Deserialize;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

#[derive(Iden)]
enum Organisation {
    Table,
    Id,
    Name,
    Postcode,
    Active,
    CreatedAt,
    UpdatedAt,
}

#[derive(sqlx::FromRow, Debug, Deserialize)]
struct OrganisationOutput {
    id: Uuid,
    name: String,
    postcode: String,
    active: bool,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
pub async fn create_organisations(pool: &Pool<Postgres>, count: u32) -> Result<(), Error> {
    let batch_size = 1_000;
    for _ in 0..(count / batch_size) {
        let organisations = (0..batch_size).map(|_| get_fake_organisation_input());
        let (sql, values) = insert_organisations_sql(organisations);
        sqlx::query_with(&sql, values).fetch_all(pool).await?;
    }
    Ok(())
}

fn insert_organisations_sql(
    organisations: impl Iterator<Item = OrganisationInput>,
) -> (String, SqlxValues) {
    let mut query = Query::insert();
    query.into_table(Organisation::Table).columns([
        Organisation::Name,
        Organisation::Postcode,
        Organisation::Active,
    ]);
    organisations.for_each(|org| {
        query.values([org.name.into(), org.postcode.into(), org.active.into()]);
    });
    query.returning_all().build_sqlx(PostgresQueryBuilder {})
}

struct OrganisationInput {
    name: String,
    postcode: String,
    active: bool,
}
fn get_fake_organisation_input() -> OrganisationInput {
    use fake::faker::boolean::en::*;
    use fake::faker::lorem::en::*;
    let name: Vec<String> = Words(2..5).fake();
    OrganisationInput {
        name: name.join(" "),
        postcode: fake::faker::address::en::PostCode().fake(),
        active: Boolean(64).fake(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insert_org_sql() {
        let organisations = (0..5).map(|_| get_fake_organisation_input());
        let (sql, _) = insert_organisations_sql(organisations);

        assert_eq!(sql, String::from(""));
    }
}
