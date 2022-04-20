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
pub fn generate_assembly_code(lines:Vec<String>) -> (Vec<u8>,Vec<usize>) {

    let mut label_offset_map: HashMap<String, u32> = HashMap::new();


    let mut assembly_code :Vec<u8> = Vec::new(); 
    let mut line_pc_vec: Vec<usize> = Vec::new();
    let mut line_number: u32 = 0;

    for line in lines {
       let original_line = line.clone();
       line_number += 1; 
       let line = &fix_hexadecimal(remove_comments(line.trim().to_lowercase().as_str()))[..];
       println!("{}", line);
       match assembly::OpcodeParser::new().parse(line) {
           Ok(ops) => {
               if let Some(l) = &ops.Label {
                   label_offset_map.insert(l.clone(), assembly_code.len() as u32);
               }
               assembly_code.append(&mut assemble(ops, &label_offset_map));
           },
            Err(_) => {println!("error");}
       }
       line_pc_vec.push(assembly_code.len());
    }
    (assembly_code,line_pc_vec)
}


#[test]
fn test_if_assembly_generated() {
    assert_eq!(generate_assembly_code(vec!["nop","trello: mvi c, 34H", "hello: mov b, c", "jz hello", "jnc trello"].iter().map(|&x| {String::from(x)}).collect()).0,vec![0, 14, 52, 65, 202, 3, 210, 1]);
  assert_eq!(generate_assembly_code(vec!["mov a, b".to_string()]).0, vec![120]);
}

#[test]
fn check_emulation() {
    let prog :Vec<String>  = vec!["nop","mvi c, 34h", "mov b, c", "add c" ,"hlt"].iter().map(|&x| {String::from(x)}).collect();

    let mut new_state = core::Processor8085::new();
    new_state.program_counter = 0;

    let (assembled_val,meta) = generate_assembly_code(prog); 
    for (counter,&opcode) in assembled_val.iter().enumerate() {
        new_state.memory[counter] = opcode;
    }
    println!("{:?}", assembled_val);
    println!("{:?}", meta);

    while let Some(pc) = emulator::emulate_8085(&mut new_state, 0) {
//        println!("{}", pc);
    }
 //   println!("{}", new_state.accumulator);
  //  println!("{}", new_state.c);
}

