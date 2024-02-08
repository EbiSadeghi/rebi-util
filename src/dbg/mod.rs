use std::dbg;

/// This module provides convinience functions
/// that report information consistently 

pub fn Success (value: &str) {
    //! For use when logging something
    //! With the expected behaviour
    println!("[+] {}", value);
}

pub fn Info (value: &str) {
    //! For use when reporting a fact
    //! or possibility about the 
    //! configuration, setup, or 
    //! environment of the code
    println!("[i] {}", value);
}

pub fn UserIO (value: &str) {
    //! For use when waiting for
    //! user input
    println!("[#] {}", value);
}

pub fn Error (value: &str) {
    //! For use when reporting an Error,
    //! or unstable and unexpected behaviour
    println!("[!] {}", value);
}    


