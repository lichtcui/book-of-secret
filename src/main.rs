mod models;
use models::{Question, Secret, FILE_PATH, LOCAL_CACHE};

use std::fs;
use std::io::Write;

fn main() {
    let mut secrets = LOCAL_CACHE.configs.clone();

    let name = Question::new("name").ask();
    let secret = match secrets.iter_mut().find(|x| x.name == name) {
        Some(result) => result,
        None => {
            let mut new_secret = Secret::new();
            new_secret.name = name;
            secrets.push(new_secret);
            secrets.last_mut().unwrap()
        }
    };

    if !secret.is_empty() {
        let action = Question::new("take an action")
            .ask_with_options(vec!["add credential", "show details"]);
        if action == "show details" {
            println!("{:?}", serde_yaml::to_string(secret));
            return;
        }
    } else {
        secret.set_info();
    }

    secret.set_credential();

    store(secrets);
}

fn store(secrets: Vec<Secret>) {
    if let Some(parent_dir) = std::path::Path::new(FILE_PATH).parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    }

    let content = secrets
        .iter()
        .map(|i| serde_yaml::to_string(i).expect("failed to serialize"))
        .reduce(|acc, e| acc + "\n" + e.as_str())
        .unwrap();

    fs::File::create(FILE_PATH)
        .expect("Failed to create file")
        .write_all(content.as_bytes())
        .expect("Failed to write to file");
}
