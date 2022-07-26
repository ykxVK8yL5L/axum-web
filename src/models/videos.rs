use crate::{schema::videos::{self,dsl::*}};
use diesel::prelude::*;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };
use chrono::{NaiveDateTime, Utc};

#[derive(Queryable,Debug,Serialize, Deserialize,)]
pub struct Video {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub cid: String,
    pub size: Option<i32>,
    pub img: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Video {
    pub fn get_all(conn: &Connection) -> QueryResult<Vec<Video>> {
        videos.order(id.asc()).load::<Video>(conn)
    }

    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Video> {
    //     videos.find(i).get_result::<Video>(conn)
    // }

    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Option<Video>> {
    //     let mut video = videos.find(i).load::<Video>(conn)?;
    //     Ok(video.pop())
    // }

}