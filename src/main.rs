mod reqwest;
mod connections;
use futures::TryStreamExt;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Connection, PgConnection, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    //connection
    let addr = connections::connector::path_to_db();
    let mut conn = connections::connector::create_connections(addr).await?;

    //reqwest

    reqwest::reqwests::simple_req(&mut conn).await?;
    reqwest::reqwests::insert(&mut conn).await?;

    reqwest::reqwests::try_field(&mut conn).await?;
    Ok(())
}
