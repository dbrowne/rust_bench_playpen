use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, error::Error};
use crate::db_models::NewItem;
use chrono::prelude::*;
use chrono::{Local, NaiveDateTime};


/// Establishes a connection to a Postgres database using Diesel.
///
/// This function will read the `DATABASE_URL` environment variable,
/// which should contain the database connection string.
/// It is expected that this environment variable is already set before
/// calling this function.
///
/// If the `DATABASE_URL` environment variable is not set, the function will
/// return an Err value with the message "DATABASE_URL must be set".
///
/// If a connection to the database cannot be established,
/// the function will return an Err value with a message indicating
/// the failure to connect to the database URL provided.
///
/// # Returns
///
/// This function will return a Result:
/// - On successful connection to the database, it will return `Ok(PgConnection)`.
/// - On failure, it will return `Err`, with a dynamic Error (`Box<dyn Error>`) indicating the reason for failure.
///
/// # Example
///
/// ```rust,no_run
/// use db_funcs::establish_connection;
///
///
///     match establish_connection() {
///         Ok(conn) => println!("Successfully connected to the database."),
///         Err(e) => eprintln!("Database connection failed: {}", e),
///     }
///
/// ```
pub fn establish_connection() -> Result<PgConnection, Box<dyn Error>> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(db) => db,
        Err(_) => return Err("DATABASE_URL must be set".into()),
    };

    let conn = PgConnection::establish(&database_url)
        .map_err(|_| -> Box<dyn Error> { format!("Error connecting to {}", database_url).into() })?;

    Ok(conn)
}


/// Persist a new item to the database.
///
/// This function inserts a new item into the `items` table in the database. The new item has an ID, an event, and a creation and modification time. The creation and modification time are both set to the current time when this function is called.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the connection to the database.
/// * `id` - A reference to the ID of the new item. This should be a unique identifier.
/// * `event` - A reference to the event associated with the new item.
///
/// # Returns
///
/// This function returns a `Result<(), Box<dyn Error>>`. If the new item is successfully inserted into the database, the function returns `Ok(())`. If there is an error, the function returns `Err`, with the error inside.
///
/// # Errors
///
/// This function might return an error if there is a problem with the database connection, or if there is a problem with the insertion query. For example, if the ID of the new item is not unique, the function might return an error.
///
/// # Example
///
/// ```rust
/// let mut conn = establish_connection()?;
///     let id = 1;
///     let event = "item_created";
///     persist_item(&mut conn, &id, event)?;
/// ```
pub fn persist_item(conn: &mut PgConnection, id: &i64, event: &str) -> Result<(), Box<dyn Error>> {
    use crate::schema::items;
    let local_time: DateTime<Local> = Local::now();
    let now: NaiveDateTime = local_time.naive_local();
    let new_item = NewItem {
        id,
        event,
        c_time: &now,
        m_time: &now,
    };
    diesel::insert_into(items::table)
        .values(new_item)
        .execute(conn)?;
    Ok(())
}

pub fn get_last_id(conn: &mut PgConnection) -> Result<i64, Box<dyn Error>> {
    use crate::schema::items::dsl::items;
    use crate::schema::items::id;
    let last_id = items
        .select(id)
        .order(id.desc())
        .first::<i64>(conn)?;
    Ok(last_id)
}
