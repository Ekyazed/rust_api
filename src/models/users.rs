use std::env::args;

use diesel::prelude::*;
use crate::{config, schema::users};
use crate::schema::users::dsl::*;


#[derive(Queryable, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name= users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

pub fn get_user(user_id: u64) -> Option<User> {
    let conn = &mut config::establish_connection();
    let result = users.find(user_id).first(conn);
    match result {
        Ok(user) => Some(user),
        Err(_) => None
    }
}

pub fn create_user(new_user: NewUser) {
    let conn = &mut config::establish_connection();
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("User haven't been created");

    println!("New user created")
}


pub fn update_user(p_id: usize, new_user: &NewUser) -> Result<usize, diesel::result::Error> {
    let conn = &mut config::establish_connection();
    let user_id = args().nth(p_id).expect("Please provide an id").parse::<u64>().expect("Invalid id");
    diesel::update(users.find(user_id))
        .set((username.eq(&new_user.username), email.eq(&new_user.email), password.eq(&new_user.password)))
        .execute(conn)
}

pub fn delete_user(p_id: usize) -> Result<usize, diesel::result::Error> {
    let conn = &mut config::establish_connection();
    let user_id = args().nth(p_id).expect("Please provide an id").parse::<u64>().expect("Invalid id");
    diesel::delete(users.find(user_id))
        .execute(conn)
}
