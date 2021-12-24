use crate::schema::users;
#[derive(Serialize, Queryable)]
pub struct User {
    pub id: Integer,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

