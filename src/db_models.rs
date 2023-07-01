use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::items;

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