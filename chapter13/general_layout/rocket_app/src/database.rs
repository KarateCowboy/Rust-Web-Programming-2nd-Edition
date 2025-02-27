use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

use lazy_static::lazy_static;
use diesel::{
   r2d2::{Pool, ConnectionManager, PooledConnection},
   pg::PgConnection,
};

use crate::config::Config;


#[derive(Debug)]
pub enum DBError {
    Unavailable
}

type PgPool = Pool<ConnectionManager<PgConnection>>;


/// This struct is responsble handling a pooled database connection. 
/// 
/// # Attributes 
/// * db_connection (PgPool): the connection pool for the database
pub struct DbConnection {
   pub db_connection: PgPool,
}

lazy_static! {
   pub static ref DBCONNECTION: DbConnection = {
      let connection_string = Config::new().map.get("DB_URL").unwrap().as_str().unwrap().to_string();
      DbConnection {
         db_connection: PgPool::builder()
            .max_size(8)
            .build(ConnectionManager::new(connection_string))
            .expect("failed to create db connection_pool")
       }
   };
}


/// This struct is responsible for getting a pooled database connection for a web request.ConnectionManager
/// 
/// # Attribtes
/// * connection (PooledConnection<ConnectionManager<PgConnection>>): a pooled connection for the database
pub struct DB {
   pub connection: PooledConnection<ConnectionManager<PgConnection>>
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for DB {
    type Error = DBError;

    async fn from_request(_: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      match DBCONNECTION.db_connection.get() {
         Ok(connection) => {
            return Outcome::Success(DB{connection})
         },
         Err(_) => {
            return Outcome::Failure((Status::BadRequest, DBError::Unavailable))
         }
      }
    }
}
