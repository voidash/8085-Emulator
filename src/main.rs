extern crate emulator;

use emulator::generate_assembly_code;

use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short,long)]
    output: Option<String>,
    filename: Option<String>,
   #[clap(short, long, conflicts_with("filename"))]
    interpreted: bool
}


fn main() {
    let args = Args::parse();
    let lines = vec![ 
        "nop; genie niskyo",
        "test: mvi a, 45h     ",
        "lda 3432h",
        "jmp test"
    ].iter().map(|&a| {
        String::from(a) 
    }).collect();
    println!("this is will some helper functions to read from file and write to file");


    generate_assembly_code(lines);

    println!("Hello {:?}",args);
}
