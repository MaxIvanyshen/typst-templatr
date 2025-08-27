use std::fs::File;
use directories::UserDirs;
use clap::{arg, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[arg(long = "templates_path", default_value = "~/.typst-templates")]
        templates_path: String,
    },
    List,
    Add {
        template_name: String,
    }
}

#[derive(Parser, Debug)]
#[command(name = "typst-templatr", about = "A tool to manage Typst templates")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

const CONFIG_FILE_NAME: &str = ".typst-templatr.yaml";

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    templates_path: String,
}

fn get_config() -> Option<Config> {
    let home_dir = UserDirs::new()?
        .home_dir()
        .to_path_buf();

    let file_path = match home_dir.to_str() {
        None => return None,
        Some(_) => home_dir.to_str().unwrap().to_string() + "/" + CONFIG_FILE_NAME,
    };

    let file = match File::open(file_path).ok() {
        None => return None,
        Some(f) => f,
    };

    match serde_yaml::from_reader(file).ok() {
        None => None,
        Some(c) => c,
    }
}

fn write_config(config: &Config) -> bool {
    let home_dir = match UserDirs::new() {
        None => return false,
        Some(d) => d.home_dir().to_path_buf(),
    };

    let file_path = match home_dir.to_str() {
        None => return false,
        Some(p) => p.to_string() + "/" + CONFIG_FILE_NAME,
    };

    let file = match File::create(file_path).ok() {
        None => return false,
        Some(f) => f,
    };

    match serde_yaml::to_writer(file, config).ok() {
        None => false,
        Some(_) => true,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init { templates_path } => {
            let config = Config { templates_path };
            if write_config(&config) {
                println!("Config file created successfully.");
                std::process::exit(0);
            } else {
                eprintln!("ERROR: Failed to create config file.");
                std::process::exit(1);
            }
        }
        Command::List => {
            let config = match get_config() {
                None => {
                    eprintln!("ERROR: Config file does not exist. Use `typst-templatr init --templates_path <path>` to create it.");
                    std::process::exit(1);
                }
                Some(config) => config,
            };

            let files = match std::fs::read_dir(&config.templates_path).ok() {
                None => {
                    eprintln!("ERROR: Failed to read templates directory.");
                    std::process::exit(1);
                }
                Some(f) => f,
            };

            for file in files {
                if let Ok(entry) = file {
                    if let Some(name) = entry.path().file_name() {
                        if let Some(name_str) = name.to_str() {
                            if name_str.ends_with(".typ") {
                                println!("- {}", name_str);
                            }
                        }
                    }
                }
            }
            std::process::exit(0);
        }
        _ => {
            eprintln!("ERROR: Command not implemented yet.");
            std::process::exit(1);
        }
    }
}
