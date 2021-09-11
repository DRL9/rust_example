use std::env;

use diesel::{Connection, QueryDsl, SqliteConnection};
// 加上这个，才有 eq 等方法
use diesel::prelude::*;
use dotenv::dotenv;

use self::models::{NewPost, Post};
use self::schema::posts;
pub mod models;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not exist");
    SqliteConnection::establish(&db_url).expect("connection error")
}

type DbResult<T> = Result<T, diesel::result::Error>;

// 加上这个，才有 filter 等方法
use self::schema::posts::dsl::*;
pub fn get_posts(is_published: bool) -> DbResult<Vec<Post>> {
    let conn = establish_connection();
    // 打印查询日志
    // println!(
    //     "{:?}",
    //     diesel::debug_query(&posts.filter(published.eq(is_published)).limit(5))
    // );
    posts
        .filter(published.eq(is_published))
        .limit(5)
        .load::<Post>(&conn)
}

pub fn get_post(post_id: i32) -> DbResult<Post> {
    let conn = establish_connection();
    posts.filter(id.eq(post_id)).first(&conn)
}

pub fn create_post(post: &NewPost) -> DbResult<()> {
    let conn = establish_connection();
    diesel::insert_into(posts::table)
        .values(post)
        // 只是执行
        // .get_result() 只有 postgres 才会有
        .execute(&conn)?;
    Ok(())
}

pub fn update_post(post_id: i32) -> DbResult<usize> {
    let conn = establish_connection();
    diesel::update(posts::table)
        .filter(id.eq(post_id))
        .set(published.eq(true))
        .execute(&conn)
}

pub fn delete_post(post_id: i32) -> DbResult<usize> {
    let conn = establish_connection();
    diesel::delete(posts::table)
        .filter(id.eq(post_id))
        .execute(&conn)
}
