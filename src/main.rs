#![allow(non_snake_case)]

use colored::*;
use regex::Regex;
use std::env;
use std::fs;
use std::process;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("You need to give at least 2 argument(run \"cargo run +help\" for more details on the commands available)")]
    NotEnoughArguments,
    #[error("The option -n needs to be proceded by a number")]
    NoNumberForCommandN,
    #[error("The argument of the option -n needs to be a number")]
    OptionForCommandNisNotaNumber,
    #[error(
        "Invalid option {0} (run \"cargo run +help\" for more details on the commands available)"
    )]
    InvalidOption(String),
    #[error("Help Display")]
    HelpRequested,
    #[error("{0} isn't the path to a folder")]
    IsNotAFolder(String),
    #[error("Failed to read directory: {0}")]
    ReadDirError(String),
    #[error("Error processing directory entry: {0}")]
    EntryProcessingError(String),
    #[error("Entry isn't a file")]
    EntryNotFile,
    #[error("There was an error reading a file {0}")]
    FileReadError(String),
    #[error("Invalid Regex Format")]
    InvalidRegex,
}

struct Config {
    search_string: String,
    max_lines: Option<usize>,
    ignore_case: bool,
    only_count: bool,
    regex: Option<Regex>,
    folder_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, ConfigError> {
        if args.len() < 2 {
            return Err(ConfigError::NotEnoughArguments);
        }

        if args[1] == "+help" {
            return Err(ConfigError::HelpRequested);
        }

        if args.len() < 3 {
            return Err(ConfigError::NotEnoughArguments);
        }

        let search_string = args[1].clone();
        let folder_name = args[args.len() - 1].clone();
        let mut max_lines = None;
        let mut ignore_case = false;
        let mut only_count = false;
        let mut regex = None;

        let mut i = 2;
        while i < args.len() - 1 {
            match args[i].as_str() {
                "-n" => {
                    if i + 2 < args.len() {
                        match args[i + 1].parse::<usize>() {
                            Ok(result) => {
                                max_lines = Some(result);
                            }
                            Err(_) => {
                                return Err(ConfigError::OptionForCommandNisNotaNumber);
                            }
                        }

                        i += 2;
                    } else {
                        return Err(ConfigError::NoNumberForCommandN);
                    }
                }
                "-i" => {
                    ignore_case = true;
                    i += 1;
                }
                "-c" => {
                    only_count = true;
                    i += 1;
                }
                "-r" => {
                    match Regex::new(&search_string) {
                        Ok(r) => {
                            regex = Some(r);
                        }
                        Err(_) => {
                            return Err(ConfigError::InvalidRegex);
                        }
                    }
                    i += 1;
                }
                _ => {
                    let invalid_option = args[i].clone();
                    return Err(ConfigError::InvalidOption(invalid_option));
                }
            }
        }

        Ok(Config {
            search_string,
            max_lines,
            ignore_case,
            only_count,
            regex,
            folder_name,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let decoy = Config::new(&args);

    match decoy {
        Ok(result) => {
            let config = result;

            let folder_path = config.folder_name;
            let folder_path = std::path::Path::new(&folder_path);
            let mut max_number_of_lines = config.max_lines;
            let mut at_least_once = false;

            println!();
            if config.regex.is_some() {
                let printed_text = "The search will be done using Regex\n"
                    .purple()
                    .bold()
                    .italic();
                println!("{}", printed_text);
            }

            match folder_iterator(
                folder_path,
                &config.search_string,
                &mut max_number_of_lines,
                config.ignore_case,
                config.only_count,
                &mut at_least_once,
                &config.regex,
            ) {
                Ok(()) => {
                    if !at_least_once {
                        let attention = "The word ".blue().bold();
                        let attention1 = " hasn't been found in any of the files.".blue().bold();
                        let search_string = config.search_string.red().bold();
                        println!("{}\"{}\"{}", attention, search_string, attention1);
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        Err(ConfigError::HelpRequested) => {
            println!(
                "\nExample:cargo run <string_to_search> -i -c -r -n 10 <folder_to_be_searched>\n"
            );
            println!("<>(Mandatory) the rest are optional\n");
            println!("Command line arguments:");
            println!("<test_string>: The string that's supposed to be searched for");
            println!("-n <number>: Max number of lines (default: infinite)");
            println!("-i: Ignore case (default: off)");
            println!("-c: Only count (prints only the number of matches in a file) (default: off)");
            println!("-r: Option to enable regex searching (default: off)");
            println!("<test_folder>: The name of folder that's gonna be searched\n");
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    };
}

fn folder_iterator(
    dir: &std::path::Path,
    word_to_search: &String,
    max_lines: &mut Option<usize>,
    ignore_case: bool,
    only_count: bool,
    at_least_once: &mut bool,
    regex: &Option<Regex>,
) -> Result<(), ConfigError> {
    if dir.is_dir() {
        let entries;

        match fs::read_dir(dir) {
            Ok(result) => {
                entries = result;

                for entry in entries {
                    match entry {
                        Ok(result) => {
                            let entry = result;

                            match entry.file_type() {
                                Ok(result) => {
                                    if result.is_dir() {
                                        let folder_path = entry.path();

                                        match folder_iterator(
                                            &folder_path,
                                            word_to_search,
                                            max_lines,
                                            ignore_case,
                                            only_count,
                                            at_least_once,
                                            regex,
                                        ) {
                                            Ok(()) => {}
                                            Err(err) => {
                                                return Err(err);
                                            }
                                        }
                                    } else if result.is_file() {
                                        let file_name = entry.file_name();

                                        let file_name_str = file_name.to_string_lossy();

                                        let file_path = entry.path();

                                        match fs::read_to_string(&file_path) {
                                            Ok(mut content) => {
                                                if let Some(value) = max_lines {
                                                    if *value > 0 {
                                                        let word;

                                                        if ignore_case {
                                                            content = content.to_lowercase();
                                                            word = word_to_search
                                                                .clone()
                                                                .to_lowercase();
                                                        } else {
                                                            word = word_to_search.clone();
                                                        }

                                                        if content.contains(&word) {
                                                            *at_least_once = true;
                                                            let first = "The file ".bold().green();
                                                            let second = " contains the string "
                                                                .bold()
                                                                .green();
                                                            let third: ColoredString =
                                                                if ignore_case {
                                                                    " (ignore case is on)".green()
                                                                } else {
                                                                    "".green()
                                                                };

                                                            println!(
                                                                "{}\"{}\"{}\"{}\"{}\n",
                                                                first,
                                                                file_name_str,
                                                                second,
                                                                word_to_search,
                                                                third
                                                            );
                                                        }
                                                        if let Some(regex) = regex {
                                                            if regex.is_match(&content) {
                                                                *at_least_once = true;
                                                                let first =
                                                                    "The file ".bold().green();
                                                                let second = " contains a substring that the Regex ".bold().green();
                                                                let third = if ignore_case {
                                                                    " has found (ignore case is on)"
                                                                        .green()
                                                                } else {
                                                                    " has found".green()
                                                                };

                                                                println!(
                                                                    "{}\"{}\"{}\"{}\"{}\n",
                                                                    first,
                                                                    file_name_str,
                                                                    second,
                                                                    word_to_search,
                                                                    third
                                                                );
                                                            }
                                                        }

                                                        finds(
                                                            &word, content, max_lines, only_count,
                                                            regex,
                                                        );
                                                    } else {
                                                        process::exit(0);
                                                    }
                                                } else {
                                                    let word;

                                                    if ignore_case {
                                                        content = content.to_lowercase();
                                                        word =
                                                            word_to_search.clone().to_lowercase();
                                                    } else {
                                                        word = word_to_search.clone();
                                                    }

                                                    if content.contains(&word) {
                                                        *at_least_once = true;
                                                        let first = "The file ".bold().green();
                                                        let second =
                                                            " contains a substring that the Regex "
                                                                .bold()
                                                                .green();

                                                        let third = if ignore_case {
                                                            " has found (ignore case is on)".green()
                                                        } else {
                                                            " has found".green()
                                                        };

                                                        println!(
                                                            "{}\"{}\"{}\"{}\"{}\n",
                                                            first,
                                                            file_name_str,
                                                            second,
                                                            word_to_search,
                                                            third
                                                        );
                                                    }
                                                    if let Some(regex) = regex {
                                                        if regex.is_match(&content) {
                                                            *at_least_once = true;
                                                            let first = "The file ".bold().green();
                                                            let second = " contains the string "
                                                                .bold()
                                                                .green();
                                                            let third = if ignore_case {
                                                                " (ignore case is on)".green()
                                                            } else {
                                                                "".green()
                                                            };

                                                            println!(
                                                                "{}\"{}\"{}\"{}\"{}\n",
                                                                first,
                                                                file_name_str,
                                                                second,
                                                                word_to_search,
                                                                third
                                                            );
                                                        }
                                                    }

                                                    finds(
                                                        &word, content, max_lines, only_count,
                                                        regex,
                                                    );
                                                }
                                            }
                                            Err(err) => {
                                                return Err(ConfigError::FileReadError(
                                                    err.to_string(),
                                                ));
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Err(ConfigError::EntryNotFile);
                                }
                            }
                        }
                        Err(err) => {
                            return Err(ConfigError::EntryProcessingError(err.to_string()));
                        }
                    }
                }
            }
            Err(_) => {
                let error_dir = dir.to_string_lossy().into_owned();
                return Err(ConfigError::ReadDirError(error_dir));
            }
        }
    } else {
        let invalid_path = dir.to_string_lossy().into_owned();
        return Err(ConfigError::IsNotAFolder(invalid_path));
    }

    Ok(())
}

fn finds(
    word_to_search: &String,
    file: String,
    max_lines: &mut Option<usize>,
    only_count: bool,
    regex: &Option<Regex>,
) {
    let mut occurrences = Vec::new();
    for (line_number, line) in file.lines().enumerate() {
        if let Some(regex) = regex {
            for cap in regex.captures_iter(line) {
                for (_, m) in cap.iter().enumerate() {
                    if let Some(match_str) = m {
                        let start = match_str.start();
                        let end = match_str.end();

                        let first_third: &str = &line[..start];
                        let second_third: String = line[start..end].blue().to_string();
                        let third_third: &str = &line[end..];

                        occurrences.push((line_number + 1, first_third, second_third, third_third));
                    }
                }
            }
        } else {
            for (s, _) in line.match_indices(word_to_search) {
                let first_third: &str = &line[..s];
                let second_third: String = line[s..s + word_to_search.len()].blue().to_string();
                let third_third: &str = &line[s + word_to_search.len()..];

                occurrences.push((line_number + 1, first_third, second_third, third_third));
            }
        }

        if let Some(value) = max_lines {
            *value -= 1;

            if *value == 0 {
                break;
            }
        }
    }

    let num_occurrences = occurrences.len();
    if num_occurrences != 0 {
        let number_of_occurrences = "Number of occurrences: ".red();

        if !only_count {
            println!("{}{}\n", number_of_occurrences, num_occurrences);
            for (line_num, f, s, t) in occurrences {
                println!("({}) {}{}{}", line_num, f, s, t);
            }
        } else {
            println!("{}{}", number_of_occurrences, num_occurrences);
        }
        if let Some(value) = max_lines {
            let line_left = "\nLines left to search: ".red();
            println!("{}{}\n", line_left, value);
        } else {
            let line_left = "\nLines left to search: ".red();
            println!("{}Inf\n", line_left,);
        }
    }
}