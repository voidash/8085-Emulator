

pub mod core;
pub mod emulator; 
pub mod disassember;
pub mod utils;
pub mod assembler;

use std::collections::HashMap;
use assembler::utils::convert_8085_hex_to_i32;
use assembler::assemble;
use assembler::assembly;

fn remove_comments(line: &str) -> &str{
    if let Some(index) = line.find(";") {
        return &line[..index];
    } 
    line
}

fn fix_hexadecimal(line: &str) -> String {
    if line.ends_with("h") {
        //find comma
        if let Some(decimal_start_pos) = line.rfind(" ") {
             let d: i32 = convert_8085_hex_to_i32(&line[decimal_start_pos..line.len()].trim());
             let ret_val: String = format!("{} {}", &line[..decimal_start_pos], i32::to_string(&d));
             return ret_val;
        }
    }
    return line.to_string();
}

#[test] 
fn remove_comments_test() {
    let data = "mov a, b ; where all the birds fly everything seems merrier";
    let b = ";where all the birds fly everything seems merrier";
    println!("{}", remove_comments(data));
    println!("{}", remove_comments(b));
}

#[test]
fn test_fix_hexadecimal() {
    let data = "mvi a, 12h";
    let y = "ldax ffffh";
    println!("{}", fix_hexadecimal(data));
    println!("{}", fix_hexadecimal(y));
}


#[allow(unused_variables,unused_mut)]
pub fn generate_assembly_code(lines:Vec<String>) -> Vec<u8> {

    let mut label_offset_map: HashMap<String, u32> = HashMap::new();

    let lines = vec![ 
        "nop; genie niskyo",
        "test: mvi a, 45h     ",
        "lda 3432h",
        "jmp test"
    ];

    let mut assembly_code :Vec<u8> = Vec::new(); 
    let mut line_number: u32 = 0;
    for line in lines {
       line_number += 1; 
       let line = &fix_hexadecimal(remove_comments(line.trim()))[..];
       match assembly::OpcodeParser::new().parse(line) {
           Ok(ops) => {
               if let Some(l) = &ops.Label {
                   label_offset_map.insert(l.clone(), line_number);
               }
                assembly_code.append(&mut assemble(ops, &label_offset_map));
           },
            Err(_) => {}
       }
       println!("{}", line);
    }

   println!("{:?}", assembly_code);
   assembly_code
}

pub fn test() {
    println!("this is test");
}
