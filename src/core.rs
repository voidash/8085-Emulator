#[allow(dead_code)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub auxillary_carry: bool,
}
impl Flags {
    pub fn new() -> Self {
        Self {
            zero: false,
            sign: false,
            parity: false,
            carry: false,
            auxillary_carry: false,
        }
    }
}

#[allow(dead_code)]
pub struct Processor8085 {
    pub accumulator: u8, //accumulator
    pub flag: Flags,
    pub b: u8, 
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub memory: Vec<u8>
}

impl Processor8085 {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            flag: Flags::new(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            stack_pointer: 0,
            program_counter: 0,
            memory: std::iter::repeat(0).take(1000000).collect()
        }            
    }
}

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct AssembledMeta {
    pub line_number: usize,
    pub start_position : usize,
    pub original_text: String
}


#[allow(dead_code,non_camel_case_types)]
pub enum CARRY {
    UPDATE_CARRY,
    PRESERVE_CARRY
}

#[allow(dead_code,non_camel_case_types)]
pub struct AssembledData {
    pub name: Option<String>,
    pub data: Option<u8>
}
