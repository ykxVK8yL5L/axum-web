use crate::{schema::videos::{self,dsl::*}};
use diesel::prelude::*;
use diesel::dsl::count;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;

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


#[derive(Debug, Serialize, Deserialize)]
pub struct VideosResult {
    pub data: Vec<Video>,
    pub recordsTotal: i64,
    pub recordsFiltered:i64,
}




impl Video {
    pub fn get_all(conn: &Connection) -> QueryResult<Vec<Video>> {
        videos.order(id.asc()).load::<Video>(conn)
    }
    pub fn pagination(params:&HashMap<String, String>,conn: &Connection) -> QueryResult<String> {
        let start:i64 = params.get("start").unwrap().parse::<i64>().unwrap();
        let keyword:String = params.get("search[value]").unwrap().to_string();
        let length:i64 = params.get("length").unwrap().parse::<i64>().unwrap();
        let total = query(&keyword).select(count(id)).get_result::<i64>(conn)?;
        let data = query(&keyword).limit(length).offset(start).load::<Video>(conn)?;
        let result = VideosResult {
            data,
            recordsTotal:total,
            recordsFiltered:total,
        };
        Ok(serde_json::to_string(&result).unwrap())
    }

    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Video> {
    //     videos.find(i).get_result::<Video>(conn)
    // }

    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Option<Video>> {
    //     let mut video = videos.find(i).load::<Video>(conn)?;
    //     Ok(video.pop())
    // }

}


fn query<'a>(keyword: &String) -> videos::BoxedQuery<'a, diesel::sqlite::Sqlite> {
    let mut query = videos::table.order(videos::id.desc()).into_boxed();
    if keyword.len() > 0 {
        query = query.filter(title.like("%".to_string() + keyword + "%")).or_filter(name.like("%".to_string() + keyword + "%"));
    }
    query
}