#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
//use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
////use serde_json::value::Value as JsonValue;
//use std::fmt;
//use std::fs;
//use std::path::PathBuf;

//#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//struct Root {
    //name: String,
    //version: String,
    //main: String,
    //repository: String,
    //author: String,
    //license: String,
    //dependencies: HashMap<String, String>,
//}

//impl fmt::Display for Root {
    //fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(
            //f,
            //"(name : {}, version : {} , dependencies : {:?})",
            //self.name, self.version, self.dependencies
        //)
    //}
//}

//fn main() {
    //let src_dir = PathBuf::from("/Users/selva/repos/misc/read-file/hello-rust/p.json");
    //let data = fs::read_to_string(src_dir).expect("Unable to read file");
    //let seri_val: Result<Root, serde_json::Error> = serde_json::from_str(&data);
    //let x = match seri_val {
        //Ok(value) => format!("{}", value),
        //Err(why) => format!("{}", why),
    //};

    //println!("{}", x);
//}
