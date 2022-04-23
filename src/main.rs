extern crate emulator;

pub mod utils;
pub mod core; 
pub mod assembler; 


use clap::Parser;
use std::path::Path;
use std::fs;

use core::Processor8085;
use utils::print_8085_state;


#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// If output is not provided then the program will assume writing to stdout
    #[clap(short,long)]
    output: Option<String>,
    /// if filename is not passed then the program will run on interpreted mode
    filename: Option<String>,
    /// run on interpreted mode
   #[clap(short, long, conflicts_with("filename"))]
    interpreted: bool,
    /// pass filename and generate assembly code 
   #[clap(short, long,conflicts_with("interpreted"))]
    assemble : bool,
    /// pass .asm file and disassemble them
   #[clap(short, long, conflicts_with("assemble"), conflicts_with("interpreted"))]
    disassemble: bool,
    /// emulate 8085 and write its state and memory values to file. 
   #[clap(short, long, conflicts_with("assemble"),conflicts_with("disassemble"),conflicts_with("interpreted"))]
    emulate: bool
}

#[allow(dead_code)]
fn read_from_file(file:&Path) -> Result<String,String> {
    if let Ok(contents) = fs::read_to_string(file) {
      return Ok(contents);
    }
    return Err(format!("couldn't read file"));
}


fn main() {
    let args = Args::parse();

    // if filename is provided then write to file except write to stdout
    let mut print_to_stdout = false;
    match args.filename {
        Some(file) => {
            print_to_stdout = false;
        }
        None => {
            print_to_stdout = true;
        }
    }

    // if running on emulator mode file shouldn't contain
    if args.emulate == true {
        println!("running emulator");
    }
    if args.disassemble == true {

    } 
    if args.assemble == true {

    }
    if args.interpreted == true {

    }

    let s = Processor8085::new();
    println!("{}", print_8085_state(s));



}
