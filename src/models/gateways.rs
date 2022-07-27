use crate::{schema::gateways::{self,dsl::*}};
use diesel::prelude::*;
use diesel::dsl::count;
use tracing::info;
use crate::db::Connection;
use serde::{ Deserialize, Serialize };
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;

#[derive(Queryable,Debug,Serialize, Deserialize,)]
pub struct Gateway {
    pub id: i32,
    pub name: String,
    pub url: String,
}



#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "gateways"]
pub struct GatewayDTO {
    pub name: String,
    pub url: String,
}


impl Gateway {
    pub fn get_all(conn: &Connection) -> QueryResult<Vec<Gateway>> {
        gateways.order(id.asc()).load::<Gateway>(conn)
    }
    pub fn insert(new_gateway: GatewayDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(gateways)
            .values(&new_gateway)
            .execute(conn)
    }

    pub fn delete(gateway_url: &String, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(gateways.filter(url.eq(gateway_url)))
            .execute(conn)
    }
}
