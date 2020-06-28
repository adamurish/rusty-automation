#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rppal::gpio::Gpio;
use rocket::State;
use std::thread;
use std::time::Duration;

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
    let gpio = Gpio::new().unwrap();
    gpio.get(26).unwrap().into_output().set_reset_on_drop(false);
    let rocket_lobster = rocket::ignite()
        .mount("/", routes![on, off])
        .manage(gpio);
    rocket_lobster.launch();
}
