use diesel::prelude::*;
use crate::schema::items;
use std::error::Error;
use chrono::prelude::*;
use chrono::{Local, NaiveDateTime};

#[derive(Queryable, Debug)]
pub struct Item {
    pub id: i64,
    pub event: String,
    pub c_time: NaiveDateTime,
    pub m_time: NaiveDateTime,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub id: &'a i64,
    pub event: &'a str,
    pub c_time: &'a NaiveDateTime,
    pub m_time: &'a NaiveDateTime,

}


impl Item {
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
}