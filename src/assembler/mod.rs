pub mod ast;
pub mod utils;

pub mod assembly;
use ast::{Opcode, Op};
use utils::convert_8085_hex_to_i32;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn assemble(parsed_line: Opcode) -> Result<Vec<usize>,String> {
    let mut assembled_vec:Vec<usize> = Vec::new(); 

    match parsed_line.Op {
        Op::OpL(instruction,label) => {

            let ins = &instruction[..];
            match ins {
                "jc" => {assembled_vec.push(0xda);},
                "jmp" => {assembled_vec.push(0xc3);},
                "jnc" =>{assembled_vec.push(0xd2);},
                "jz" => {assembled_vec.push(0xca);},
                "jnz" => {assembled_vec.push(0xc2);},
                "jp" => {assembled_vec.push(0xf2);},
                "jm" => {assembled_vec.push(0xfa);},
                "jpe" => {assembled_vec.push(0xea);},
                "jpo" => {assembled_vec.push(0xe2);},
                _ => {
                    return Err(format!("{} not is one of the instruction jc, jmp , jnc, jz, jnz, jp, jm, jpe or jpo, that takes <opcode> <label> ",ins));
                }
            }

            // hash labels
            let mut s = DefaultHasher::new();
            label.clone().hash(&mut s);
            assembled_vec.push(s.finish() as usize);
        },
        Op::Op(instruction) => {
            let ins = &instruction[..];
            match ins {
                "nop" => {assembled_vec.push(0x00);},
                "rlc" => {assembled_vec.push(0x07);},
                "rrc" => {assembled_vec.push(0x0f);},
                "ral" => {assembled_vec.push(0x17);},
                "rar" => {assembled_vec.push(0x1f);},
                "rim" => {assembled_vec.push(0x20);},
                "daa" => {assembled_vec.push(0x27);},
                "cma" => {assembled_vec.push(0x2f);},
                "sim" => {assembled_vec.push(0x30);},
                "stc" => {assembled_vec.push(0x37);},
                "cmc" => {assembled_vec.push(0x3f);},
                "rnz" => {assembled_vec.push(0xc0);},
                "rz" => {assembled_vec.push(0xc8);},
                "rnc" => {assembled_vec.push(0xd0);},
                "rc" => {assembled_vec.push(0xd8);},
                "rpo" => {assembled_vec.push(0xe0);},
                "xthl" => {assembled_vec.push(0xe3);},
                "xchg" => {assembled_vec.push(0xeb);},
                "rp" => {assembled_vec.push(0xf0);},
                "di" => {assembled_vec.push(0xf3);},
                "rm" => {assembled_vec.push(0xf8);},
                "sphl" => {assembled_vec.push(0xf9);},
                "pchl" => {assembled_vec.push(0xe9);}
                "ei" => {assembled_vec.push(0xfb);},
                "hlt" => {assembled_vec.push(0x76);},
                _ => {

                        return Err(format!("{} not is one of the instruction that takes <opcode>. ",ins));
                }
            }
        },
        Op::OpRR(instruction,register1, register2 ) => {
            let ins = &instruction[..];
            let reg1 = &register1[..];
            let reg2 = &register2[..];

            match(ins, reg1, reg2) {
                ("mov", "a", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x78); },
                        "c" => { assembled_vec.push(0x79); },
                        "d" => { assembled_vec.push(0x7A); },
                        "e" => { assembled_vec.push(0x7B); },
                        "h" => { assembled_vec.push(0x7C); },
                        "l" => { assembled_vec.push(0x7D); },
                        "m" => { assembled_vec.push(0x7E); },
                        "a" => { assembled_vec.push(0x7F); },
                        _ => {
                            return Err(format!("mov a, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                    },
                ("mov", "b", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x40); },
                        "c" => { assembled_vec.push(0x41); },
                        "d" => { assembled_vec.push(0x42); },
                        "e" => { assembled_vec.push(0x43); },
                        "h" => { assembled_vec.push(0x44); },
                        "l" => { assembled_vec.push(0x45); },
                        "m" => { assembled_vec.push(0x46); },
                        "a" => { assembled_vec.push(0x47); },
                        _ => {
                            return Err(format!("mov b, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                    },
                ("mov", "c", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x48); },
                        "c" => { assembled_vec.push(0x49); },
                        "d" => { assembled_vec.push(0x4a); },
                        "e" => { assembled_vec.push(0x4b); },
                        "h" => { assembled_vec.push(0x4c); },
                        "l" => { assembled_vec.push(0x4d); },
                        "m" => { assembled_vec.push(0x4e); },
                        "a" => { assembled_vec.push(0x4f); },
                        _ => {
                            return Err(format!("mov c, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                    },
                ("mov", "d", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x50); },
                        "c" => { assembled_vec.push(0x51); },
                        "d" => { assembled_vec.push(0x52); },
                        "e" => { assembled_vec.push(0x53); },
                        "h" => { assembled_vec.push(0x54); },
                        "l" => { assembled_vec.push(0x55); },
                        "m" => { assembled_vec.push(0x56); },
                        "a" => { assembled_vec.push(0x57); },
                        _ => {
                            return Err(format!("mov d, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                },
                ("mov", "e", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x58); },
                        "c" => { assembled_vec.push(0x59); },
                        "d" => { assembled_vec.push(0x5a); },
                        "e" => { assembled_vec.push(0x5b); },
                        "h" => { assembled_vec.push(0x5c); },
                        "l" => { assembled_vec.push(0x5d); },
                        "m" => { assembled_vec.push(0x5e); },
                        "a" => { assembled_vec.push(0x5f); },
                        _ => {
                            return Err(format!("mov e, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                    },
                ("mov", "h", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x60); },
                        "c" => { assembled_vec.push(0x61); },
                        "d" => { assembled_vec.push(0x62); },
                        "e" => { assembled_vec.push(0x63); },
                        "h" => { assembled_vec.push(0x64); },
                        "l" => { assembled_vec.push(0x65); },
                        "m" => { assembled_vec.push(0x66); },
                        "a" => { assembled_vec.push(0x67); },
                        _ => {
                            return Err(format!("mov h, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                    },
                ("mov", "l", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x68); },
                        "c" => { assembled_vec.push(0x69); },
                        "d" => { assembled_vec.push(0x6a); },
                        "e" => { assembled_vec.push(0x6b); },
                        "h" => { assembled_vec.push(0x6c); },
                        "l" => { assembled_vec.push(0x6d); },
                        "m" => { assembled_vec.push(0x6e); },
                        "a" => { assembled_vec.push(0x6f); },
                        _ => { 
                            return Err(format!("mov l, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                },
                ("mov", "m", x) => {
                    match x {
                        "b" => { assembled_vec.push(0x70); },
                        "c" => { assembled_vec.push(0x71); },
                        "d" => { assembled_vec.push(0x72); },
                        "e" => { assembled_vec.push(0x73); },
                        "h" => { assembled_vec.push(0x74); },
                        "l" => { assembled_vec.push(0x75); },
                        "a" => { assembled_vec.push(0x77); },
                        _ => { 
                            return Err(format!("mov m, {} isn't supported. Supported Registers are b,c,d,e,h,l,m and a", x));
                        }
                        }
                },
                (_,_,_) => {
                            return Err(format!("{} {}, {} doesn't exist. Perhaps you meant mov r, r; Supported registers are b,c,d,e,h,l,m and a",ins, reg1, reg2));
                }
            }
        },
        Op::OpRA(instruction, register, address) => {
            let ins = &instruction[..];
            let register = &register[..];
            let a_u8 = address as usize;

            //for 16 bit
            // TODO: if data is greater than 2^16 then show error 
            match (ins, register) {
                ("lxi", r) => {
                    match r {
                        "b" => { assembled_vec.push(0x01); },
                        "d" => { assembled_vec.push(0x11); },
                        "h" => { assembled_vec.push(0x21); },
                        "sp" => { assembled_vec.push(0x31); },
                        _ => {
                            return Err(format!("lxi {} doesn't exist. Supported register pair  are b, d, h , sp ",r));
                        }
                    }
                    assembled_vec.append(&mut (address as u16).to_le_bytes().to_vec().iter().map(|&val| {val as usize}).collect::<Vec<usize>>());
                },
                ("mvi", r) => {
                    match r {
                        "b" => { assembled_vec.push(0x06); },
                        "c" => { assembled_vec.push(0x0e); },
                        "d" => { assembled_vec.push(0x16); },
                        "e" => { assembled_vec.push(0x1e); },
                        "h" => { assembled_vec.push(0x26); },
                        "l" => { assembled_vec.push(0x2e); },
                        "m" => { assembled_vec.push(0x36); },
                        "a" => { assembled_vec.push(0x3e); },
                        _ => {
                            return Err(format!("mvi {} doesn't exist. Supported registers  are b,c,d,e,h,l,m and a ",r));
                        }
                    }
                    assembled_vec.push(a_u8);
                },
                (_,_) => {
                            return Err(format!("{} {} doesn't exist. Perhaps you meant : mvi register, {value}  or lxi registerpair, {value}",ins, register,value = address));
                }
            }
        },
        Op::OpA(instruction, address) => {
            // TODO: remove assembed_vec from inside the matched block, only one is sufficient
            // after match block is completed add assembled_vec.push() 
            let ins = &instruction[..];
            match ins {
                "cpi" => {
                    assembled_vec.push(0xfe);
                    assembled_vec.push(address as usize);
                },
                "ori" => {
                    assembled_vec.push(0xf6);
                    assembled_vec.push(address as usize);
                },
                "ani" => {
                    assembled_vec.push(0xe6);
                    assembled_vec.push(address as usize);
                },
                "sbi" => {
                    assembled_vec.push(0xde);
                    assembled_vec.push(address as usize);
                },
                "in" => {
                    assembled_vec.push(0xdb);
                    assembled_vec.push(address as usize);
                },
                "sui" => {
                    assembled_vec.push(0xd6);
                    assembled_vec.push(address as usize);
                },
                "out" => {
                    assembled_vec.push(0xd3);
                    assembled_vec.push(address as usize);
                },
                "aci" => {
                    assembled_vec.push(0xd8);
                    assembled_vec.push(address as usize);
                },
                "adi" => {
                    assembled_vec.push(0xc6);
                    assembled_vec.push(address as usize);
                },
                "lda" => {
                    assembled_vec.push(0x3a);
                    assembled_vec.append(&mut (address as u16).to_le_bytes().to_vec().iter().map(|&val| {val as usize}).collect::<Vec<usize>>());
                },
                "sta" => {
                    assembled_vec.push(0x32);
                    assembled_vec.append(&mut (address as u16).to_le_bytes().to_vec().iter().map(|&val| {val as usize}).collect::<Vec<usize>>());
                },
                "jpo" => {
                    assembled_vec.push(0xe2);
                    assembled_vec.append(&mut (address as u16).to_le_bytes().to_vec().iter().map(|&val| {val as usize}).collect::<Vec<usize>>());

                },
                "cpo" => {
                    assembled_vec.push(0xe4);
                    assembled_vec.append(&mut (address as u16).to_le_bytes().to_vec().iter().map(|&val| {val as usize}).collect::<Vec<usize>>());

                },
                "jmp" => {
                    assembled_vec.push(0xc3);
                    assembled_vec.append(&mut (address as u16).to_le_bytes().to_vec().iter().map(|&val| {val as usize}).collect::<Vec<usize>>());
                },
                _ => {
                    return Err(format!("{} {} doesn't exist. perhaps you meant to use jmp, cpo, jpo, sta, lda, adi, aci,ori or cpi ",ins,address));
                } }
        },
        Op::OpR(instruction, register) => {
            let ins = &instruction[..];
            let reg = &register[..];
            match(ins, reg) {
                ("inr", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x04); },
                        "c" => { assembled_vec.push(0x0c); },
                        "d" => { assembled_vec.push(0x14); },
                        "e" => { assembled_vec.push(0x1c); },
                        "h" => { assembled_vec.push(0x24); },
                        "l" => { assembled_vec.push(0x2c); },
                        "m" => { assembled_vec.push(0x34); },
                        _ => {
                            return Err(format!("inr {} doesn't exist. perhaps you meant to increment b,c,d,e,h,l or m",a));
                        }
                    }
                },
                ("dcr", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x05); },
                        "c" => { assembled_vec.push(0x0d); },
                        "d" => { assembled_vec.push(0x15); },
                        "e" => { assembled_vec.push(0x1d); },
                        "h" => { assembled_vec.push(0x24); },
                        "l" => { assembled_vec.push(0x2d); },
                        "m" => { assembled_vec.push(0x35); },
                        _ => {
                            return Err(format!("dcr {} doesn't exist. perhaps you meant to decrement b,c,d,e,h,l or m",a));
                        }
                    }
                },
                ("inx", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x03); },
                        "d" => { assembled_vec.push(0x13); },
                        "h" => { assembled_vec.push(0x23); },
                        "sp" => { assembled_vec.push(0x33); },
                        _ => {
                            return Err(format!("inx {} doesn't exist. perhaps you meant to increment register pair b, d, h or sp",a));
                        }
                    }
                },
                ("dcx", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x0b); },
                        "d" => { assembled_vec.push(0x1b); },
                        "h" => { assembled_vec.push(0x2b); },
                        "sp" => { assembled_vec.push(0x3b); },
                        _ => {
                            return Err(format!("dcx {} doesn't exist. perhaps you meant to decrement register pair b, d, h or sp",a));
                        }
                    }
                },
                ("add", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x80); },
                        "c" => { assembled_vec.push(0x81); },
                        "d" => { assembled_vec.push(0x82); },
                        "e" => { assembled_vec.push(0x83); },
                        "h" => { assembled_vec.push(0x84); },
                        "l" => { assembled_vec.push(0x85); },
                        "m" => { assembled_vec.push(0x86); },
                        "a" => { assembled_vec.push(0x87); },
                        _ => { 
                            return Err(format!("add only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("sub", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x90); },
                        "c" => { assembled_vec.push(0x91); },
                        "d" => { assembled_vec.push(0x92); },
                        "e" => { assembled_vec.push(0x93); },
                        "h" => { assembled_vec.push(0x94); },
                        "l" => { assembled_vec.push(0x95); },
                        "m" => { assembled_vec.push(0x96); },
                        "a" => { assembled_vec.push(0x97); },
                        _ => { 
                            return Err(format!("sub only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("adc", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x88); },
                        "c" => { assembled_vec.push(0x89); },
                        "d" => { assembled_vec.push(0x8a); },
                        "e" => { assembled_vec.push(0x8b); },
                        "h" => { assembled_vec.push(0x8c); },
                        "l" => { assembled_vec.push(0x8d); },
                        "m" => { assembled_vec.push(0x8e); },
                        "a" => { assembled_vec.push(0x8f); },
                        _ => {
                            return Err(format!("adc only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("sbb", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x98); },
                        "c" => { assembled_vec.push(0x99); },
                        "d" => { assembled_vec.push(0x9a); },
                        "e" => { assembled_vec.push(0x9b); },
                        "h" => { assembled_vec.push(0x9c); },
                        "l" => { assembled_vec.push(0x9d); },
                        "m" => { assembled_vec.push(0x9e); },
                        "a" => { assembled_vec.push(0x9f); },
                        _ => { 
                            return Err(format!("sbb only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("ana", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xa0); },
                        "c" => { assembled_vec.push(0xa1); },
                        "d" => { assembled_vec.push(0xa2); },
                        "e" => { assembled_vec.push(0xa3); },
                        "h" => { assembled_vec.push(0xa4); },
                        "l" => { assembled_vec.push(0xa5); },
                        "m" => { assembled_vec.push(0xa6); },
                        "a" => { assembled_vec.push(0xa7); },
                        _ => { 
                            return Err(format!("ana only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("xra", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xa8); },
                        "c" => { assembled_vec.push(0xa9); },
                        "d" => { assembled_vec.push(0xaa); },
                        "e" => { assembled_vec.push(0xab); },
                        "h" => { assembled_vec.push(0xac); },
                        "l" => { assembled_vec.push(0xad); },
                        "m" => { assembled_vec.push(0xae); },
                        "a" => { assembled_vec.push(0xaf); },
                        _ => { 
                            return Err(format!("xra only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("ora", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xb0); },
                        "c" => { assembled_vec.push(0xb1); },
                        "d" => { assembled_vec.push(0xb2); },
                        "e" => { assembled_vec.push(0xb3); },
                        "h" => { assembled_vec.push(0xb4); },
                        "l" => { assembled_vec.push(0xb5); },
                        "m" => { assembled_vec.push(0xb6); },
                        "a" => { assembled_vec.push(0xb7); },
                        _ => { 
                            return Err(format!("ora only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                        }
                    }
                },
                ("cmp", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xb8); },
                        "c" => { assembled_vec.push(0xb9); },
                        "d" => { assembled_vec.push(0xba); },
                        "e" => { assembled_vec.push(0xbb); },
                        "h" => { assembled_vec.push(0xbc); },
                        "l" => { assembled_vec.push(0xbd); },
                        "m" => { assembled_vec.push(0xbe); },
                        "a" => { assembled_vec.push(0xbf); },
                        _ => { 
                            return Err(format!("cmp only supports registers b,c,d,e,h,l,m and a. It doesn't support register {}",a));
                                                        
                        }
                    }
                },
                ("stax", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x02); },
                        "d" => { assembled_vec.push(0x12); },
                        _ => { 
                            return Err(format!("stax only supports registers pair b and d. It doesn't support register {}",a));
                        }
                    }
                },
                ("ldax", a) => {
                    match a {
                        "b" => { assembled_vec.push(0x0a); },
                        "d" => { assembled_vec.push(0x1a); },
                        _ => {
                            return Err(format!("ldax only supports register pair b and d. It doesn't support register {}",a));
                        }
                    }
                },

                ("push", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xc5); },
                        "d" => { assembled_vec.push(0xd5); },
                        "h" => { assembled_vec.push(0xe5); },
                        "psw" => { assembled_vec.push(0xf5); },
                        _ => {
                            return Err(format!("push only supports registers pair b,d, h and psw. It doesn't support register {}",a));
                        }
                    }
                },
                ("pop", a) => {
                    match a {
                        "b" => { assembled_vec.push(0xc1); },
                        "d" => { assembled_vec.push(0xd1); },
                        "h" => { assembled_vec.push(0xe1); },
                        "psw" => { assembled_vec.push(0xf1); },
                        _ => {
                            return Err(format!("pop only supports registers pair b,d, h and psw. It doesn't support register {}",a));
                        }
                    }
                },
                (_,_) => {
                    return Err(format!("{ins} {register} isn't supported. supported instructions for this format are immediate opcodes such as adi , sbi, ani, xri, cpi, push and pop etc",ins=ins, register=register));
                }
            }
        }
    }
    Ok(assembled_vec)
}



#[test]
fn literal_check() {
    assert_eq!(assembly::DecParser::new().parse("12"), Ok(12));
}

#[test]
fn register_check() {
    assert_eq!(assembly::RegisterParser::new().parse("a").unwrap(), "a");
    assert_ne!(assembly::RegisterParser::new().parse("Z"), Ok("Z"));
}

#[test]
fn instruction_check() {
    assert_eq!(
        assembly::InstructionParser::new().parse("mov").unwrap(),
        "mov"
    );
    assert_ne!(
        assembly::InstructionParser::new().parse("ldaxx"),
        Ok("ldaxx")
    );
}

#[test]
fn opcode_check() {
    println!(
        "{:?}",
        assembly::OpcodeParser::new()
            .parse("check: mov a    , b")
            .unwrap()
    );
    println!(
        "{:?}",
        assembly::OpcodeParser::new().parse("mvi a, 32D").unwrap()
    );
    println!(
        "{:?}",
        assembly::OpcodeParser::new().parse("mvi a").unwrap()
    );

    println!("{:?}",assemble(assembly::OpcodeParser::new().parse("mvi a, 32D").unwrap()));
    println!("{:?}",assemble(assembly::OpcodeParser::new().parse("add b").unwrap()));
}

#[allow(dead_code)]
fn remove_comments(line: &str) -> &str{
    if let Some(index) = line.find(";") {
        return &line[..index];
    } 
    line
}

#[allow(dead_code)]
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


