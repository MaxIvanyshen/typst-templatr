use std::fs::File;
use std::path::Path;
use std::io;
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

fn get_config_path() -> Option<String> {
    let home_dir = UserDirs::new()?
        .home_dir()
        .to_path_buf();

    match home_dir.to_str() {
        None => None,
        Some(p) => Some(p.to_string() + "/" + CONFIG_FILE_NAME),
    }
}

fn get_config() -> Option<Config> {
    let config_path = match get_config_path() {
        None => return None,
        Some(p) => p,
    };

    let file = match File::open(config_path).ok() {
        None => return None,
        Some(f) => f,
    };

    match serde_yaml::from_reader(file).ok() {
        None => None,
        Some(c) => c,
    }
}

fn write_config(config: &Config) -> bool {
    let file_path = match get_config_path() {
        None => return false,
        Some(p) => p,
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

#[cfg(unix)]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    if src.as_ref().is_dir() {
        std::os::windows::fs::symlink_dir(src, dst)
    } else {
        std::os::windows::fs::symlink_file(src, dst)
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
        Command::Add { mut template_name }=> {
            let config = match get_config() {
                None => {
                    eprintln!("ERROR: Config file does not exist. Use `typst-templatr init --templates_path <path>` to create it.");
                    std::process::exit(1);
                }
                Some(config) => config,
            };

            if !template_name.ends_with(".typ") {
                template_name.push_str(".typ");
            }

            let src_path = Path::new(&config.templates_path).join(&template_name);
            let dst_path = Path::new(".").join(&template_name);
            if !src_path.exists() {
                eprintln!("ERROR: Template '{}' does not exist in templates path.", template_name);
                std::process::exit(1);
            }

            if dst_path.exists() {
                eprintln!("ERROR: A file or directory named '{}' already exists in the current directory.", template_name);
                std::process::exit(1);
            }

            match create_symlink(&src_path, &dst_path) {
                Ok(_) => {
                    println!("Template '{}' added successfully.", template_name);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("ERROR: Failed to create symlink: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
