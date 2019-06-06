#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::Pin;

#[get("/")]
fn index() -> &'static str {
    let in1 = Pin::new(16);
    let in2 = Pin::new(26);
    in1.with_exported(|| {
            in1.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            in1.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
    }).unwrap();

    "Hello, world!"
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

