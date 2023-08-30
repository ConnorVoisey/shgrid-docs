use rand::Rng;

use server::{
    connect,
    models::{contact::create_contacts, organisation::create_organisations},
};
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let pool = connect().await;

    truncate(&pool).await.expect("failed to truncate");
    println!("finished truncating records");

    println!("started seeding");
    let organisation_count = 10_000;
    let org_res = create_organisations(&pool, organisation_count).await;
    match org_res {
        Ok(_) => println!("Success"),
        Err(err) => println!("error: {err}"),
    }
    let org_ids = match get_org_ids(&pool, 100_000).await {
        Ok(org_ids) => org_ids,
        Err(err) => {
            println!("error: {err}");
            return;
        }
    };
    println!("org ids count: {}", org_ids.len());

    let contact_res = create_contacts(&pool, &org_ids).await;
    match contact_res {
        Ok(_) => println!("Success"),
        Err(err) => println!("error: {err}"),
    }
}

async fn truncate(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query!(r#"TRUNCATE TABLE organisation CASCADE;"#)
        .execute(pool)
        .await?;

    sqlx::query!(r#"TRUNCATE TABLE contact CASCADE;"#)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Clone, Default)]
struct IdRow {
    id: Uuid,
}
async fn get_org_ids(
    pool: &Pool<Postgres>,
    count: usize,
) -> Result<Vec<Option<Uuid>>, sqlx::error::Error> {
    let mut vec = vec![None; count];
    let org_ids = sqlx::query_as!(
        IdRow,
        "SELECT id FROM organisation LIMIT $1",
        count as i64 / 2
    )
    .fetch_all(pool)
    .await?;
    // One org has many contacts - this assigns many contacts for each org
    //TODO: this is really not a great solution - improve this at some point
    for _ in 0..20 {
        for row in &org_ids {
            let index = rand::thread_rng().gen_range(0..vec.len());
            vec[index] = Some(row.id);
        }
    }

    Ok(vec)
}
