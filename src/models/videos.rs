use crate::{schema::videos::{self,dsl::*}};
use diesel::prelude::*;
use diesel::dsl::count;
use tracing::info;
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
        let sort_index:i64 = if params.get("order[0][column]").is_none(){
            0
        }else{
            params.get("order[0][column]").unwrap().parse::<i64>().unwrap()
        };
        let column_sort_name = format!("columns[{}][data]",sort_index); //the key of sort column of form query 
        let sort_name = params.get(&column_sort_name).unwrap(); //the value of sort column of form query
        let sort_dir = if params.get("order[0][dir]").is_none(){
            "desc"
        }else{
            params.get("order[0][dir]").unwrap()
        };


        let total = query(&keyword,sort_name,sort_dir).select(count(id)).get_result::<i64>(conn)?;
        let data = query(&keyword,sort_name,sort_dir).limit(length).offset(start).load::<Video>(conn)?;
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

fn query<'a>(keyword: &String, order_column_name:&str,sort_dir:&str) -> videos::BoxedQuery<'a, diesel::sqlite::Sqlite> {
    let mut query = videos::table.into_boxed();
    if keyword.len() > 0 {
        query = query.filter(title.like("%".to_string() + keyword + "%")).or_filter(name.like("%".to_string() + keyword + "%"));
    }
    if sort_dir == "desc" {
        match order_column_name {
            "id" => query = query.order(id.desc()),
            "name" => query = query.order(name.desc()),
            "title" => query = query.order(title.desc()),
            "size" => query = query.order(size.desc()),
            "created_at" => query = query.order(created_at.desc()),
            _ => query = query.order(id.desc())
        }
    }else{
        match order_column_name {
            "id" => query = query.order(id.asc()),
            "name" => query = query.order(name.asc()),
            "title" => query = query.order(title.asc()),
            "size" => query = query.order(size.asc()),
            "created_at" => query = query.order(created_at.asc()),
            _ => query = query.order(id.asc())
        }
    }

    query
}