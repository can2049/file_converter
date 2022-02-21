use std::io::prelude::*;
use std::path::Path;

use clap::Parser;

/// Simple program to convert .toml .json .yaml to each other
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// file convert mode. like toml2json json2yaml
    #[clap(short, long)]
    mode: String,

    /// file path
    file: String,
}

fn main() {
    let args = Args::parse();
    let mode = args.mode;
    let ops: Vec<&str> = mode.split('2').collect();
    if ops.len() != 2 {
        println!("Mode should like this: toml2json. Input file on left, output file type on right");
        std::process::exit(1);
    }

    let path = Path::new(&args.file);
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut all_lines = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        all_lines = format!("{}{}\n", all_lines, line);
    }

    let in_type = ops[0].to_uppercase();
    let in_type = in_type.as_str();

    let object = match in_type {
        "JSON" => {
            let object: serde_json::Value =
                serde_json::from_str(&all_lines).expect("Failed to parse input as valid JSON!");
            object
        }
        "TOML" => {
            let object: toml::Value =
                toml::from_str(&all_lines).expect("Failed to parse input as valid TOML!");
            let json_data =
                serde_json::to_string_pretty(&object).expect("Failed to convert object to JSON!");
            let output = serde_json::from_str(&json_data).unwrap();
            output
        }
        "YAML" => {
            let object: serde_yaml::Value =
                serde_yaml::from_str(&all_lines).expect("Failed to parse input as valid YAML!");
            let json_data =
                serde_json::to_string_pretty(&object).expect("Failed to convert object to JSON!");
            let output = serde_json::from_str(&json_data).unwrap();
            output
        }
        _ => {
            println!(
                "Invalid input type: `{}`. Valid type contains: yaml/json/toml",
                ops[0]
            );
            std::process::exit(1)
        }
    };

    let out_type = ops[1].to_uppercase();
    let out_type = out_type.as_str();
    let output = match out_type {
        "TOML" => {
            let toml = toml::to_string_pretty(&object).expect("Failed to convert object to TOML!");
            toml
        }
        "JSON" => {
            let json =
                serde_json::to_string_pretty(&object).expect("Failed to convert object to JSON!");
            json
        }
        "YAML" => {
            let yaml = serde_yaml::to_string(&object).expect("Failed to convert object to YAML!");
            yaml
        }
        _ => {
            println!(
                "Invalid output type: `{}`. Valid type contains: yaml/json/toml",
                ops[1]
            );
            std::process::exit(1)
        }
    };

    println!("{}", output);
}
