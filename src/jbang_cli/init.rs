use std::collections::HashMap;
use clap::{Arg, Command};
use handlebars::{Handlebars};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::jbang_cli::set_executable;

const TEMPLATES: &[&str] = &["hello", "cli"];

fn handlebars() -> Handlebars<'static> {
    let mut hbs = Handlebars::new();
    hbs.register_template_string("hello", include_str!("templates/hello.java")).unwrap();
    hbs.register_template_string("cli", include_str!("templates/cli.java")).unwrap();
    hbs
}

pub fn manage_init(init_matches: &clap::ArgMatches) {
    let mut script_file = init_matches.get_one::<String>("scriptOrFile").unwrap().to_string();
    let mut class_name = script_file.clone();
    if !script_file.contains('.') {
        script_file = format!("{}.java", script_file);
    } else {
        class_name = script_file.split('.').next().unwrap().to_string();
    }
    let file_name = if script_file.contains('/') {
        script_file.split('/').last().unwrap().to_string()
    } else {
        script_file.clone()
    };
    let params = if let Some(params) = init_matches.get_many::<String>("params") {
        params.into_iter().collect_vec()
    } else {
        vec![]
    };
    let mut code: Option<String> = None;
    if !params.is_empty() { // generate code from AI
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            code = extract_code_from_openai(&api_key, params.get(0).unwrap());
        } else {
            println!("Please specify OPENAI_API_KEY environment variable to generate code from AI.");
            return;
        };
    } else { // generate code from template
        let default_template = "hello".to_owned();
        let template = init_matches.get_one::<String>("template").unwrap_or(&default_template);
        if TEMPLATES.contains(&template.as_str()) {
            let mut context: HashMap<String, String> = HashMap::new();
            context.insert("className".to_string(), class_name);
            context.insert("fileName".to_string(), file_name);
            code = handlebars().render(template, &context).ok()
        } else {
            println!("Please specify correct template you wish to generate code.");
        }
    }
    if let Some(code) = code {
        std::fs::write(&script_file, code).unwrap();
        set_executable(&script_file);
        println!("Script file: {}", script_file);
    } else {
        println!("Please specify OPENAI_API_KEY environment variable to generate code from AI.");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<AIChoice>,
}

impl OpenAIResponse {
    pub fn get_answer(&self) -> String {
        let mut answer = String::new();
        for choice in &self.choices {
            answer.push_str(&choice.message.content);
        }
        answer
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIChoice {
    pub index: i32,
    pub message: AIMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

fn generate_code_from_openai(api_key: &str, question: &str) -> String {
    let request = json!({
  "model": "gpt-4o-mini",
  "messages": [
    {
      "role": "system",
      "content": "You are an Java expert. You are writing Java code with [JBang](https://www.jbang.dev/) support. Add no additional text. Please add dependencies, javadoc in the code."
    },
    {
      "role": "user",
      "content": question
    }
  ]
});
    let client = reqwest::blocking::Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .json(&request)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .unwrap()
        .json::<OpenAIResponse>()
        .unwrap();
    response.get_answer()
}

fn extract_code_from_openai(api_key: &str, question: &str) -> Option<String> {
    let answer = generate_code_from_openai(api_key, question);
    let mut code_found = false;
    let mut code_lines: Vec<&str> = vec![];
    for line in answer.lines() {
        if code_found == true && line.starts_with("```") {
            break;
        }
        if code_found == true {
            code_lines.push(line);
        }
        if code_found == false && line.starts_with("```") {
            code_found = true;
        }
    }
    if !code_lines.is_empty() {
        if !code_lines.get(0).unwrap().starts_with("#!") {
            code_lines.insert(0, "///usr/bin/env jbang \"$0\" \"$@\" ; exit $?");
        }
        return Some(code_lines.join("\n"));
    }
    None
}

pub fn build_init_command() -> Command {
    Command::new("init")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .num_args(1)
                .help("Init script with a java class useful for scripting")
                .required(false)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("file or URL to a Java code file")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("params")
                .help("Parameters to pass on to the generation.")
                .required(false)
                .index(2)
                .num_args(1..)
        )
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use crate::jbang_cli::clap_app::build_jbang_app;
    use super::*;

    #[test]
    fn test_render_template() {
        let template = "cli";
        let mut context: HashMap<String, String> = HashMap::new();
        context.insert("className".to_string(), "hello".to_string());
        context.insert("fileName".to_string(), "hello.java".to_string());
        let code = handlebars().render(template, &context).unwrap();
        println!("{}", code);
    }

    #[test]
    fn test_generate_code_from_openai() {
        dotenv().unwrap();
        let app = build_jbang_app();
        let matches = app.get_matches_from(["jbang", "init", "Hello.java", "Please write a simple Hello.java"]);
        manage_init(matches.subcommand_matches("init").unwrap());
    }
}
