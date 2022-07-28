use crate::{schema::videos::{self,dsl::*}};
use diesel::prelude::*;
use diesel::dsl::count;
use tracing::info;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };
use chrono::{NaiveDateTime, Utc};
use std::{collections::HashMap, sync::Arc};


use mongodb::{
    bson::doc,
    bson::Document,
    sync::Client,
    options::InsertManyOptions,
};

use crate::models::settings::{Setting};

#[derive(Queryable,Debug,Serialize, Deserialize,)]
pub struct Video {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub cid: String,
    pub size: Option<String>,
    pub img: Option<String>,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "videos"]
pub struct VideoDTO {
    pub name: String,
    pub title: String,
    pub cid: String,
    pub size: Option<String>,
    pub img: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosResult {
    pub data: Vec<Video>,
    pub recordsTotal: i64,
    pub recordsFiltered:i64,
}


#[derive(Debug, Serialize, Deserialize)]
struct Remote {
    name: String,
    cid: String,
    size: String,
    issync:i32
}



#[derive(Debug, Serialize, Deserialize)]
struct Task {
    url: String,
    isnow:i32
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

    pub fn insert(video: VideoDTO,conn: &Connection) -> QueryResult<usize> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(videos)
            .values((
                name.eq(&video.name),
                title.eq(&video.title),
                cid.eq(&video.cid),
                size.eq(video.size),
                img.eq(video.img),
                created_at.eq(now),
            ))
            .execute(conn)
    }

    pub fn edit (i: i32, video: VideoDTO,conn: &Connection) -> QueryResult<usize> {
        let now = Utc::now().naive_utc();
        diesel::update(videos.find(i))
            .set((
                name.eq(&video.name),
                title.eq(&video.title),
                cid.eq(&video.cid),
                size.eq(video.size),
                img.eq(video.img),
                created_at.eq(now),
            ))
            .execute(conn)
    }

    pub fn delete(vid: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(videos.filter(id.eq(vid)))
            .execute(conn)
    }

    pub fn sync(conn: &Connection) -> Result<String, mongodb::error::Error> {
        let mongodb_url = match Setting::find_value_by_key(&"MONGO_DB_CONNECT".to_string(), conn) {
            Ok(gateway) => gateway,
            Err(err) => {
                info!("{:?}", err);
                "".to_string()
            }
        };
        if  mongodb_url.is_empty(){
            return Ok("MongoDB url is empty".to_string());
        }

        let client = Client::with_uri_str(mongodb_url).unwrap();
        let db = client.database("mydb");
        let collection = db.collection::<Remote>("web3");
        let cursor = collection.find(doc! { "issync": 0 }, None)?;
        let mut sync_count = 0;

        for result in cursor {
            //info!("title: {}", result?.name);
           match result {
                Ok(remote) => {
                    let video = VideoDTO {
                        name: String::from(&remote.name),
                        title: String::from(&remote.name),
                        cid: remote.cid,
                        size: Some(remote.size),
                        img: Some("".to_string()),
                    };
                    let _ = Video::insert(video, conn);
                    sync_count += 1;
                }
                Err(err) => {
                    info!("{:?}", err);
                }
            }
        }
        collection.update_many(doc! { "issync": 0 }, doc! { "$set": { "issync": 1 } }, None)?;
        Ok(format!("同步成功，共同步{}个",sync_count))

    }

    pub fn add_task(download_url:&String,isnow:i32,conn: &Connection) -> Result<usize,mongodb::error::Error> {
        let mongodb_url = match Setting::find_value_by_key(&"MONGO_DB_CONNECT".to_string(), conn) {
            Ok(gateway) => gateway,
            Err(err) => {
                info!("{:?}", err);
                "".to_string()
            }
        };
        if  mongodb_url.is_empty(){
            return Ok(0);
        }
        
        let client = Client::with_uri_str(mongodb_url).unwrap();
        let db = client.database("mydb");
        let collection = db.collection::<Task>("task");
        let mut task_count = 0;
        let mut task_dto =  Vec::new();

        let task_list = download_url.split("\n").collect::<Vec<&str>>();
        for task in task_list  {
            if task.trim().is_empty(){
                continue;
            }
            task_dto.push(Task {
                url: task.to_string(),
                isnow: isnow,
            });
            task_count += 1;
        }

        if task_dto.len() > 0 {
            let _ = collection.insert_many(task_dto, None)?;
        }
            
        Ok(task_count)
    }

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