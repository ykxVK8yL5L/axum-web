use crate::{schema::videos::{self,dsl::*}};
use diesel::prelude::*;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };


#[derive(Queryable,Debug,Serialize, Deserialize,)]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


impl Video {
    pub fn get_all(conn: &Connection) -> QueryResult<Video> {
        videos.filter(published.eq(true)).get_result::<Video>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Video> {
        videos.find(i).get_result::<Video>(conn)
    }

    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Option<Video>> {
    //     let mut video = videos.find(i).load::<Video>(conn)?;
    //     Ok(video.pop())
    // }

}