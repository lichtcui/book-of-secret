use dialoguer::{Input, Password, Select};

pub struct Question<'a> {
    text: &'a str,
}

impl<'a> Question<'a> {
    pub fn new(text: &'a str) -> Self {
        Question { text }
    }

    pub fn ask(&self) -> String {
        Input::new().with_prompt(self.text).interact_text().unwrap()
    }

    // TODO 获取时进行 base64 加密
    pub fn ask_secretly(&self) -> String {
        Password::new()
            .with_prompt(self.text)
            .with_confirmation("Confirm password", "Passwords mismatching")
            .interact()
            .unwrap()
    }

    pub fn ask_with_options(&self, options: Vec<&str>) -> String {
        let selection = Select::new()
            .with_prompt(self.text)
            .items(&options)
            .interact()
            .unwrap();

        options[selection].to_string()
    }
}
