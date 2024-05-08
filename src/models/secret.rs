use crate::Question;

use serde::{Deserialize, Serialize};
use std::process;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    #[serde(rename = "apiVersion")]
    api_version: String,
    pub kind: String,
    pub name: String,
    #[serde(default)]
    annotations: String,
    credentials: Vec<Credential>,
}

impl Secret {
    pub fn new() -> Self {
        Self {
            api_version: String::from("v1"),
            kind: String::from("ssh"),
            name: String::from(""),
            annotations: String::from(""),
            credentials: vec![],
        }
    }

    pub fn set_info(&mut self) {
        self.kind = Question::new("chose secret kind").ask_with_options(vec!["ssh", "website"]);
        self.annotations = Question::new("annotations").ask();
    }

    pub fn set_credential(&mut self) {
        let username = Question::new("username").ask();
        match self.credentials.iter().find(|x| x.username == username) {
            Some(result) => {
                println!("{:?}", result);
                process::exit(0)
            }
            None => {
                let password = Question::new("password").ask_secretly();
                self.credentials.push(Credential::new(&username, &password))
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.credentials.is_empty()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Credential {
    username: String,
    password: String,
}

impl Credential {
    fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}
