use clap::{command, Arg, ArgMatches, Command};
use std::env;
use std::fs::File;
use std::io::prelude::*;
enum Mode {
    Create,
    Open,
}
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("Args: {:?}", args);
    // if args.len() == 3 {
    //     let open_config = OpenConfig::new(&args);
    //     println!("Should open file: {}", open_config.title);
    //     let _ = open_file(&open_config.title);
    // } else if args.len() == 4 {
    //     let config = CreateConfig::new(&args);
    //     println!("Should create file: {}", config.title);
    //     let _ = create_file(config);
    // }

    let matches: ArgMatches = command!()
        .arg(
            Arg::new("mode")
                // .short('m')
                // .long("mode")
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

    if mode == Some(&"open".to_string()) {
        println!("Open mode");
        let title = matches.get_one::<String>("title").unwrap();
        println!("Title: {:?}", title);
    }

    if mode == Some(&"create".to_string()) {
        println!("Create mode");
        let title = matches.get_one::<String>("title").unwrap();
        let body = matches.get_one::<String>("body").unwrap();
        println!("Title: {:?}", title);
        println!("Body: {:?}", body);
    }

    // println!("Match result: {:?}", matches);
}
fn create_file(config: CreateConfig) -> std::io::Result<()> {
    // let default_path: &str = "../../../notes";
    // let path = format!("{}/{}.txt", default_path, config.title);
    let path = format!("{}.txt", config.title);
    let mut file = File::create(path)?;
    let content = config.body.replace("_", " ");
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn open_file(filename: &str) -> std::io::Result<()> {
    let path = format!("{}.txt", filename);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File content: {}", contents);
    Ok(())
}

impl CreateConfig {
    fn new(args: &[String]) -> CreateConfig {
        if args.len() < 4 {
            panic!("Not enough args for create config");
        }
        let flag = args[1].clone();
        let title = args[2].clone();
        let body = args[3].clone();

        CreateConfig { flag, title, body }
    }
}

impl OpenConfig {
    fn new(args: &[String]) -> OpenConfig {
        if args.len() < 3 {
            panic!("Not enough args for open config");
        }
        let flag = args[1].clone();
        let title = args[2].clone();

        OpenConfig { flag, title }
    }
}

struct CreateConfig {
    flag: String,
    title: String,
    body: String,
}
struct OpenConfig {
    flag: String,
    title: String,
}
