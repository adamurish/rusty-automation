#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;

use std::process::Command;

use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;
use rocket::State;

#[get("/on")]
fn on(gpio: State<Gpio>) -> String {
    let mut pin = gpio.get(26).unwrap().into_output();
    pin.set_high();
    format!("LED on")
}

#[get("/off")]
fn off(gpio: State<Gpio>) -> String {
    let mut pin = gpio.get(26).unwrap().into_output();
    pin.set_low();
    format!("LED off")
}

fn main() {
    let rocket_lobster = rocket::ignite();
    rocket_lobster.mount("/", routes![on, off]);
    rocket_lobster.manage(Gpio::new().unwrap());
    rocket_lobster.launch();
}
