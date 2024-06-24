use diesel::prelude::*;

use crate::schema::users::{self, id};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User<'a> {
    pub id: Option<&'a str>,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub age: i32,
}

pub fn create_user(conn: &mut SqliteConnection, name: &str, age: i32) {
    use crate::schema::users;

    let new_user = NewUser { name, age };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("error saving new user.");

    println!("{result:?}")
}

pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: i32) -> User {
    use crate::schema::users::dsl::*;

    let result = users.first(conn).expect("fuck");

    result
}
