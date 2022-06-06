use sea_orm::{Database, DatabaseConnection};

pub async fn estabilish_connection() -> DatabaseConnection{
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    match Database::connect(database_url.as_str()).await{
        Ok(estabilished_connection) => estabilished_connection,
        Err(_err) => panic!("Não foi possível conectar ao banco.")
    }
}
