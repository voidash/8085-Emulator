extern crate emulator;

pub mod utils;
pub mod core; 
pub mod assembler; 


use clap::Parser;
use emulator::generate_assembly_code;
use emulator::utils::print_8085_state;
use std::path::Path;
use std::fs;

use crate::emulator::core::Processor8085;
use crate::emulator::emulator::emulate_8085;
use emulator::disassember::disassemble_data;

//use utils::print_8085_state;
use std::fs::File;
use std::io::prelude::*;


// for interpreted mode
use std::io::Write;

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

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

fn main() {
    let args = Args::parse();

    // if filename is provided then write to file except write to stdout
    let mut print_to_stdout = false;
    match &args.output {
        Some(_) => {
            print_to_stdout = false;
        }
        None => {
            print_to_stdout = true;
        }
    }

    // if running on emulator mode file shouldn't contain
    if args.emulate == true {
        println!("running emulator");
        // check the extension. Only asm file should be allowed as filename
        if !args.filename.clone().unwrap().ends_with(".dsm") {
            panic!("Error!!! you should assemble your code first and pass the .asm file");
        }
        
        let read_contents  = std::fs::read(Path::new(args.filename.clone().unwrap().as_str())).unwrap();
        println!("{:?}", read_contents);
        let mut program_counter = 0;

        // create emulator instance
        let mut new_state = Processor8085::new();
        new_state.program_counter = 0;

        println!("Loading contents to 8085 memory");
        let mut total_mem: usize =  0;
        for (counter,&opcode) in read_contents.iter().enumerate() {
            new_state.memory[counter] = opcode;
            total_mem = counter;
        }

        loop {
            let emulate = emulate_8085(&mut new_state, 0); 
            match emulate {
                Some(pc) => {
                    program_counter += pc;
                }
                None => {break;}
            }
            if program_counter >= total_mem as u16 {
                break;
            }
        }
       println!("{}",print_8085_state(&new_state));

    }
    if args.disassemble == true {
        println!("running on disassembly mode");
        let read_contents  = std::fs::read(Path::new(args.filename.clone().unwrap().as_str())).unwrap();
        let mut disassembled_contents = Vec::new();
        println!("{:?}", read_contents);
        let mut program_counter = 0;
        loop {
            let (content, pc) = disassemble_data(&read_contents[..],program_counter);
            disassembled_contents.push(content);
            program_counter += pc;
            if program_counter >= read_contents.len() {
                break;
            }
        }

        if print_to_stdout {
            for content in disassembled_contents {
                println!("{}", content);

            }
        }else {
            // write to file
            let mut file = File::create(Path::new(args.output.clone().unwrap().as_str())).expect("unable to create file");

            for content in disassembled_contents {
                write!(&mut file ,"{} \n", content).expect("couldn't write to file");
            } 

        }
    } 

    if args.assemble == true {
        println!("running on assembly mode");
        //first read from file 
        let read_contents  = read_from_file(Path::new(args.filename.clone().unwrap().as_str()));
        let to_pass = read_contents.expect("filename not privided").split("\n").
            filter(|d| {d.trim().len() != 0}).
            map(|d| {String::from(d)}).
            collect::<Vec<String>>();


        let output = emulator::generate_assembly_code(to_pass).unwrap().0;
        
        if print_to_stdout {
            println!("{:?}", output )
        }else {
            // write to file
            let mut file = File::create(Path::new(args.output.clone().unwrap().as_str())).expect("unable to create file");
            file.write_all(&output[..]).unwrap();
        }


    }
    if args.interpreted == true {
        let mut new_state = Processor8085::new();
        new_state.program_counter = 0;

        let mut memory_position = 0;
        let mut prev_memory_position = 0;
        loop {
            let input = prompt("8085 Prompt > ");
            match input.as_str() {
                "show state" => {
                    println!("{}", print_8085_state(&new_state));
                },
                _ => {
                    prev_memory_position = memory_position;
                    match generate_assembly_code(vec![input]) {
                        Ok((read_contents,_)) => {
                        for &opcode in read_contents.iter(){
                            new_state.memory[memory_position] = opcode;
                            memory_position += 1;
                        }

                        emulate_8085(&mut new_state, prev_memory_position);
                        println!("{:?}", read_contents);
                        },
                        Err(err) => {
                            println!("{:?}", err.1);
                        }
                    }
                }
            }
        }
    }


}
