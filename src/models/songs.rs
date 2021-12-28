use crate::{schema::songs::{self,dsl::*}};
use diesel::prelude::*;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };


#[derive(Queryable,Debug,Serialize, Deserialize,)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub artist: String,
    pub time: String,
    pub filename: Option<String>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "songs"]
pub struct SongDTO {
    pub name: String,
    pub artist: String,
    pub time: String,
    pub filename: Option<String>,
}


impl Song {
    pub fn get_all(conn: &Connection) -> QueryResult<Vec<Song>> {
        songs.order(id.asc()).load::<Song>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Song> {
        songs.find(i).get_result::<Song>(conn)
    }


    pub fn insert(new_song: SongDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(songs)
            .values(&new_song)
            .execute(conn)
    }

    pub fn update(i: i32, updated_song: SongDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(songs.find(i))
            .set(&updated_song)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(songs.find(i)).execute(conn)
    }


    // pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Option<Video>> {
    //     let mut video = videos.find(i).load::<Video>(conn)?;
    //     Ok(video.pop())
    // }

}