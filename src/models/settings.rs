use crate::{schema::settings::{self,dsl::*}};
use diesel::prelude::*;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };


#[derive(Queryable,Debug,Serialize, Deserialize,)]
pub struct Setting {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub desc: Option<String>,
    pub hidden: bool,
}



#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "settings"]
pub struct SettingDTO {
    pub key: String,
    pub value: String,
    pub desc: Option<String>,
    pub hidden: bool,
}


impl Setting {
    #[allow(dead_code)]
    pub fn get_all(conn: &Connection) -> QueryResult<Vec<Setting>> {
        settings.order(id.asc()).load::<Setting>(conn)
    }

    pub fn find_value_by_key(key_name: &String, conn: &Connection) -> QueryResult<String> {
        settings.filter(key.eq(key_name)).select(value).first::<String>(conn)
    }

    pub fn update_value_by_key(key_name: &String, update_value: &String, conn: &Connection) -> QueryResult<usize> {
        diesel::update(settings.filter(key.eq(key_name))).set(value.eq(&update_value)).execute(conn)
    }

}
