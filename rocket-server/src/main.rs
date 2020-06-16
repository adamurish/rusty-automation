#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;

use std::process::Command;

#[derive(Deserialize)]
struct Torrent {
    filename: String,
    url: String
}

#[get("/")]
fn index() -> &'static str {
    "Automated Download Server"
}

#[post("/torrent", format = "json", data = "<message>")]
fn add_torrent(message: Json<Torrent>) -> String{
    //setup command
    let mut curl = Command::new("powershell");
    let curl_command = format!("Invoke-WebRequest \"{}\" -O {}", message.url, message.filename);
    curl.args(&["-C", curl_command.as_str()]);
    println!("Command: {}", curl_command);

    //handle output
    match curl.output() {
        Ok(out) =>  {
            if out.status.success() {
                format!("Torrent recieved\n{}\n{}\n{}", message.filename, message.url, out.status)
            }else {
                format!("Curl error: {}", out.status)
            }
        },
        Err(e) => format!("CMD error: {}", e)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, add_torrent]).launch();
}