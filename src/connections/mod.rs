pub mod connector {
    use std::env;
    use sqlx::{Connection, Error, PgConnection, Row};
    pub fn path_to_db() -> String {

        let postgres_user
            = dotenv::var("POSTGRES_USER").expect("not found user");
        let postgres_pass
            = dotenv::var("POSTGRES_PASSWORD").expect("not found pass");
        let postgres_db
            = dotenv::var("POSTGRES_DB").expect("not found database name");
        let postgres_port
            = dotenv::var("PORT").expect("not found port").parse::<u16>().expect("wrong env format");

        let url = format!("postgres://{}:{}@localhost:{}/{}", postgres_user, postgres_pass, postgres_port, postgres_db);

        url
    }

    pub(crate) async fn create_connections(addr: String)-> Result<PgConnection, sqlx::Error>{
        let mut conn = sqlx::postgres::PgConnection::connect(addr.as_str()).await?;
        Ok(conn)
    }
/*    async fn create_pool_connections(addr: String)-> Result<, sqlx::Error>{

    }*/
}



