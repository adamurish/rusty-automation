#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;

use std::process::Command;

use rppal::gpio::Gpio;

#[get("/on")]
fn on() -> String {
    let mut pin = Gpio::new()?.get(26)?.into_output();
    pin.set_high();
    format!("LED on")
}

#[get("/off")]
fn off() -> String {
    let mut pin = Gpio::new()?.get(26)?.into_output();
    pin.set_low();
    format!("LED off")
}

fn main() {
    let rocket_lobster = rocket::ignite();
    rocket_lobster.mount("/", routes![on, off]).launch();
}
