pub mod core;
pub mod emulator; 
pub mod disassember;
pub mod utils;
pub mod assembler;


use std::collections::HashMap;
use assembler::utils::*;
use assembler::assemble;
use assembler::assembly;
use regex::RegexSet;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn remove_comments(line: &str) -> &str{
    if let Some(index) = line.find(";") {
        return &line[..index].trim();
    } 
    line
}

fn fix_numbers(line: &str) -> String {
    let set = RegexSet::new(&[
        r"[0-9abcdef]h", // hex
        r"0x[0-9abcdef]", // hex
        r"0o[0-7]", // octal
        r"0b[01]", // binary
    ]).unwrap();
    let matches = set.matches(line);
    // for hex starting with 0
    
    if matches.matched(0){
        //find comma
        if let Some(decimal_start_pos) = line.rfind(" ") {
             let d: i32 = convert_8085_hex_to_i32(&line[decimal_start_pos..line.len()].trim());
             let ret_val: String = format!("{} {}", &line[..decimal_start_pos], i32::to_string(&d));
             return ret_val;
        }
    } else if matches.matched(1) {
        if let Some(decimal_start_pos) = line.rfind("0x") {
             let d: i32 = convert_modern_hex_to_i32(&line[(decimal_start_pos)..line.len()].trim());
             println!("{}", d);
             let ret_val: String = format!("{} {}", &line[..decimal_start_pos], i32::to_string(&d));
             return ret_val;
        }
    } else if matches.matched(2) {
        if let Some(decimal_start_pos) = line.rfind("0o") {
             let d: i32 = convert_modern_oct_to_i32(&line[(decimal_start_pos)..line.len()].trim());
             let ret_val: String = format!("{} {}", &line[..decimal_start_pos], i32::to_string(&d));
             return ret_val;
        }
    } else if matches.matched(3) {
        if let Some(decimal_start_pos) = line.rfind("0b") {
             let d: i32 = convert_modern_bin_to_i32(&line[(decimal_start_pos)..line.len()].trim());
             let ret_val: String = format!("{} {}", &line[..decimal_start_pos], i32::to_string(&d));
             return ret_val;
        }
    } 

    return line.to_string();
}


#[test] 
fn remove_comments_test() {
    let data = "mov a, 56h ; where all the birds fly everything seems merrier";
    let b = ";where all the birds fly everything seems merrier";
    assert_eq!(remove_comments(data), "mov a, 56h");
    assert_eq!(remove_comments(b), "");
}

#[test]
fn test_fix_hexadecimal() {
    let data = "mvi a, 0x234";
    let y = "ldax ffffh";

    println!("{}", fix_numbers(data));
    println!("{}", fix_numbers(y));
}




#[allow(unused_variables,unused_mut)]
pub fn generate_assembly_code(lines:Vec<String>) -> Result<(Vec<u8>,Vec<usize>),(usize, String)> {

    let mut label_offset_map: HashMap<String, usize> = HashMap::new();

    let mut assembly_code :Vec<usize> = Vec::new(); 
    let mut line_pc_vec: Vec<usize> = Vec::new();
    let mut line_number: usize = 0;

    for line in lines {
       let original_line = line.clone();
       line_number += 1; 
       let line = &fix_numbers(remove_comments(line.trim().to_lowercase().as_str()))[..];
       println!("{}", line);
       match assembly::OpcodeParser::new().parse(line) {
           Ok(ops) => {
               if let Some(l) = &ops.Label {
                   label_offset_map.insert(l.clone(), assembly_code.len());
               }
               let assembled_code = assemble(ops);
               match assembled_code {
                   Ok(mut code) => {
                       assembly_code.append(&mut code);
                   }
                   Err(error) => {
                       return Err((line_number,error));
                   }
               }
           },
           Err(x) => {
            if !original_line.is_empty() {
               if !original_line.trim().starts_with(";") {
                   return Err((line_number,format!("No such instruction exists: {}",line)));
                }
               }
            }
           }
       line_pc_vec.push(assembly_code.len());
    }
    // now replace hashes of labels with equivalent linenumber or offset present in assembly_code
    for (key, value)  in label_offset_map.into_iter() {
        // calculate the hash of key
        let mut s = DefaultHasher::new();
        key.clone().hash(&mut s);
        let hash_value = s.finish() as usize;

        assembly_code = assembly_code.iter().map(|&code| { 
            if code == hash_value {
               return value as usize;
            }
            return code;
        }).collect();
    }

    // check if assembly_code still contains value bigger than 0xff
    for (i, &code) in assembly_code.iter().enumerate(){
        if code > 0xff {
           //get line number
           let line_num = line_pc_vec.iter().position(|&val| val == (i-1)).unwrap();
           println!("{:?}", line_pc_vec);
           return Err((line_num+2,format!("No where to jump to. There is no label ")));
        }
    }

    Ok((assembly_code.iter().map(|&val| { val as u8 }).collect::<Vec<u8>>(),line_pc_vec))
}


#[test]
fn test_if_assembly_generated() {
    assert_eq!(generate_assembly_code(vec!["nop","trello: mvi c, 34H ; sure this should work", "hello: mov b, c", "jz hello", "jnc trello"].iter().map(|&x| {String::from(x)}).collect()).unwrap().0,vec![0, 14, 52, 65, 202, 3, 210, 1]);
  assert_eq!(generate_assembly_code(vec!["mov a, b".to_string()]).unwrap().0, vec![120]);
}

#[test]
fn check_emulation() {
    let prog :Vec<String>  = vec!["nop","mvi c, 34h", "mov b, c", "add c" ,"hlt"].iter().map(|&x| {String::from(x)}).collect();

    let mut new_state = core::Processor8085::new();
    new_state.program_counter = 0;

    let (assembled_val,meta) = generate_assembly_code(prog).unwrap(); 
    for (counter,&opcode) in assembled_val.iter().enumerate() {
        new_state.memory[counter] = opcode;
    }
    println!("{:?}", assembled_val);
    println!("{:?}", meta);

    while let Some(pc) = emulator::emulate_8085(&mut new_state, 0) {
                
    }
}

