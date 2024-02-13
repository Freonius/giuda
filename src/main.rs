use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} file.yaml", args[0]);
    } else {
        let file: &str = &args[1].as_str();
        let data: YamlDoc = load_file(file);
        println!(
            "{}: {}",
            data.name,
            data.content.unwrap_or("default".to_string())
        );
    }
}

struct YamlDoc {
    name: String,
    content: Option<String>,
}

fn default() -> YamlDoc {
    // default
    YamlDoc {
        name: "".to_string(),
        content: None,
    }
}

fn load_file(file: &str) -> YamlDoc {
    let mut file: File = File::open(file).expect("Unable to open file");
    let mut contents: String = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs: Vec<Yaml> = YamlLoader::load_from_str(&contents).unwrap();
    let yaml: &Yaml = &docs[0];
    let mut out: YamlDoc = default();

    out.name = yaml["name"].as_str().unwrap().to_string();
    if !yaml["content"].is_null() {
        out.content = Some(yaml["content"].as_str().unwrap().to_string());
    }
    return out; // iterate / process doc[s] ..
}
