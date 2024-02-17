
use chrono::NaiveDateTime;
#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
#[cfg(test)]
use diesel::mysql::Mysql;
use diesel::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Queryable, PartialEq, Debug)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

fn examine_sql_from_insert_default_values() {
    use schema::users::dsl::*;

    let query = insert_into(users).default_values();
    let sql = "INSERT INTO `users` () VALUES () -- binds: []";
    assert_eq!(sql, debug_query::<Mysql, _>(&query).to_string());
}
