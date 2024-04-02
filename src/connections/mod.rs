pub mod connector {
    pub fn path_to_db() -> String {
        let postgres_user = var::env(POSTGRES_USER).expect("not found user");
        let postgres_pass = var::env(POSTGRES_PASSWORD).expect("not found pass");
        let postgres_db = var::env(POSTGRES_DB).expect("not found database name");
        let postgres_port = var::env(PORT).expect("not found port");

        let url = format!("postgres://{}:{}@localhost:{}/{}", postgres_user, postgres_pass, postgres_port, postgres_db);

        url
    }
}



