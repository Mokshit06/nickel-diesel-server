use diesel::prelude::*;
use dotenv::dotenv;
use std::{env, fmt, sync::Mutex};

pub struct Conn(pub PgConnection);

#[derive(Debug)]
pub struct ServerData {
    pub connection: Mutex<Conn>,
}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Conn").field(&"PgConnection").finish()
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // TODO use r2d2 connection pool instead
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// kind of hacky, but works
#[macro_export]
macro_rules! get_connection {
    ($req: expr) => {{
        let data: &ServerData = &$req.server_data();
        &data.connection.lock().unwrap().0
    }};
}
