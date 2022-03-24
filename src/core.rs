#[allow(dead_code)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub auxillary_carry: bool,
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


#[allow(dead_code,non_camel_case_types)]
pub enum CARRY {
    UPDATE_CARRY,
    PRESERVE_CARRY
}
