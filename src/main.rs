use std::env;
use tera::Tera;
use tera::Context;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read arguments.
    let args: Vec<String> = env::args().collect();
    let template_filepath = &args[1];
    let context_filepath = &args[2];

    // Load Tera.
    let tera = match Tera::new("samples/*") {
        Ok(t) => t,
        Err(e) => {
            println!("ERROR reading template {}: {}", template_filepath, e);
            ::std::process::exit(1);
        }
    };

    // Read JSON.
    let mut file = File::open(context_filepath).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    // Transform JSON into a Tera context.
    let context: tera::Context = match Context::from_serialize(&json) {
        Ok(t) => t,
        Err(e) => {
            println!("ERROR reading context {}", e);
            ::std::process::exit(1);
        }
    };

    // Render template.
    match tera.render(template_filepath, &context) {
        Ok(t) => println!("{}", t),
        Err(e) => {
            println!("ERROR rendering template {}: {}", template_filepath, e);
            ::std::process::exit(1);
        }
    };
}
