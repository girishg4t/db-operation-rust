use crate::db;
use crate::error_handler::CustomError;
use crate::model::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
}

impl Users {
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn create(user: User) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }
}

impl User {
    fn from(user: User) -> User {
        User { name: user.name }
    }
}
