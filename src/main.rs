use clap::{command, Arg, ArgMatches};
use std::fs::File;
use std::io::prelude::*;
use std::{env, io};

enum Mode {
    Create,
    Open,
}

fn main() {
    let path_from_file = read_env_file();

    let matches: ArgMatches = command!()
        .arg(
            Arg::new("mode")
                .help("Mode of the program. Create or open a note.")
                .required(true)
                .value_parser(["create", "open"]),
        )
        .arg(Arg::new("title").required(true).help("Title of the note"))
        .arg(
            Arg::new("body")
                .required_if_eq("mode", "create")
                .help("Body of the note"),
        )
        .get_matches();

    let mode = matches.get_one::<String>("mode");
    println!("Mode: {:?}", mode);
    println!("The path is: {:?}", path_from_file);

    if mode == Some(&"open".to_string()) {
        println!("Open mode");
        let title = matches.get_one::<String>("title").unwrap();
        println!("Title: {:?}", title);
        open_note(&path_from_file, &title);
    }

    if mode == Some(&"create".to_string()) {
        println!("Create mode");
        let title = matches.get_one::<String>("title").unwrap();
        let body = matches.get_one::<String>("body").unwrap();
        println!("Title: {:?}", title);
        println!("Body: {:?}", body);
        create_note(&path_from_file, title, body);
    }

    // println!("Match result: {:?}", matches);
}
fn read_input() -> String {
    println!("Enter the path to the notes directory: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_env_file() -> String {
    let file = File::open(".env");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();

            let path_env = file.read_to_string(&mut contents);
            match path_env {
                Ok(_) => {
                    let path_value = contents.split('=').last().unwrap();
                    path_value.to_string()
                }
                Err(_) => {
                    write_path_to_env(&read_input()).unwrap();
                    read_env_file()
                }
            }
        }
        Err(_) => {
            write_path_to_env(&read_input()).unwrap_or(println!("Error wirting path to env"));
            read_env_file()
        }
    }
}

fn write_path_to_env(path: &String) -> io::Result<()> {
    let string_to_write = format!("NOTES_PATH={}", path);
    let mut file = File::create(".env")?;
    file.write_all(string_to_write.as_bytes())?;
    Ok(())
}

fn create_note(path: &String, title: &String, body: &String) {
    let file = File::create(format!("{}/{}.txt", path, title));
    match file {
        Ok(mut file) => {
            file.write_all(body.as_bytes()).ok();
        }
        Err(error) => {
            println!("Error creating file{:?}", error)
        }
    }
}

fn open_note(path: &String, title: &String) {
    let file = File::open(format!("{}/{}.txt", path, title));
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).ok();
            println!("{}", contents);
        }
        Err(_) => println!("Error opening file"),
    }
}
