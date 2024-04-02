pub mod reqwests{
    use futures::TryStreamExt;
    use sqlx::{PgConnection, query, Row};

    pub async fn try_field( conn:  &mut PgConnection) -> Result<(), sqlx::Error> {
        let mut res = query("SELECT * FROM app_user")
            .fetch(conn);
        while let Some(res) = res.try_next().await?{
            let value: &str = res.try_get("password")?;
            println!("{}", value);

        }
        Ok(())
    }
    pub async fn insert(conn: &mut PgConnection) -> Result<(), sqlx::Error>{
        sqlx::query("INSERT INTO app_user (username, password, email) VALUES ('kajnj', 'janwdk', 'akwjdn')")
            .execute(conn).await?;
        Ok(())
    }
    pub async fn simple_req( conn: &mut PgConnection) -> Result<(), sqlx::Error>{
        let mut res = query("SELECT * FROM app_user").execute(conn).await?;
        println!("{:?}", res);
        Ok(())
    }
}