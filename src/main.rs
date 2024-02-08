use std::net;

mod networking;
use networking::protocol;

mod config;
use config::Config;

mod dbg;
use dbg::Error;
use dbg::Info;
use dbg::Success;


fn main() {
    let string = 
    "Hello";
        
    println!("{}", string);
    println!("Welcome to ...");

    /// Read Configuration
    let cfg: Config = Config::new();

    Info("Info");
    Error("Error");
    Success("Success");

    println!("{:#?}", cfg);

}



