
// use std::{path::Path, collections::HashMap};

use clap::Parser;
use cli_image::Args;
use cli_image::run;
use std::process::exit;
fn main() {

    let args = Args::parse();

    if let Err(e) =run(&args){
        eprintln!("{}",e.to_string());
        exit(1); 
    } 
   
}
