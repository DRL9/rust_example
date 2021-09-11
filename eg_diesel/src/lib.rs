use std::{convert::TryFrom, error::Error, io, process};

use db::get_posts;

use crate::db::{delete_post, models::NewPost, update_post};

// 加上这句，才会启动该 crate 的 宏
#[macro_use]
extern crate diesel;

pub mod db;

pub enum Command {
    Print = 1,
    Create = 2,
    Publish = 3,
    Delete = 4,
    Exit,
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "1" => Ok(Command::Print),
            "2" => Ok(Command::Create),
            "3" => Ok(Command::Publish),
            "4" => Ok(Command::Delete),
            _ => Ok(Command::Exit),
        }
    }
}

pub fn print_commands() {
    println!(
        r"
        1: print posts
        2: create posts
        3: publish post
        4: delete post
        other input exit
    "
    );
}

pub fn parse_command(str: &str) -> Result<Command, ()> {
    Command::try_from(str)
}

pub fn run_command(command: &Command) {
    match &command {
        Command::Create => create(),
        Command::Print => show_all(),
        Command::Publish => publish_post().unwrap_or_default(),
        Command::Delete => delete().unwrap_or_default(),
        Command::Exit => {
            println!("Bye!");
            process::exit(0)
        }
    }
}
fn create() {
    println!("please input title");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let mut body = String::new();
    println!("please input post body");
    io::stdin().read_line(&mut body).unwrap();
    db::create_post(&NewPost {
        title: &title.trim(),
        body: &&body,
    })
    .unwrap();
    println!("create success!");
}

fn show_all() {
    let posts = get_posts(true).unwrap();
    println!("total post count: {}", posts.len());
    for post in posts {
        println!("id: {}", post.id);
        println!("title: {}", post.title);
        println!("body:");
        println!("{}", post.body);
        println!("======");
    }
}

type PopUpResult = Result<(), Box<dyn Error>>;

fn publish_post() -> PopUpResult {
    println!("unpublished post as follow: ");
    let posts = get_posts(false).unwrap();
    for post in posts {
        println!("{:?}", post);
    }
    let mut id = String::new();
    println!("Please input id you want to publish");
    io::stdin().read_line(&mut id).unwrap();
    if update_post(id.trim().parse()?).unwrap() > 0 {
        println!("publish success")
    } else {
        println!("id not found");
    }
    Ok(())
}

fn delete() -> PopUpResult {
    println!("Please input id you want to delete.");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    if delete_post(id.trim().parse()?)? > 0 {
        println!("delete success!")
    } else {
        println!("id not found")
    }
    Ok(())
}
