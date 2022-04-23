use crate::core::Processor8085;

pub fn check_parity(data: u8 , size: u8) -> bool{
let mut ones = 0;
for i in 0..size {
    if (data >> i) & 0x1 == 1{
        ones += 1;
    }
}
    ones % 2 == 0
}

pub fn bool_to_bin(data:bool) -> u8{
    if data == true {1} else {0}
}

pub fn bin_to_bool(data:u8) -> bool{
    if data == 1 {true} else {false}
}

pub fn print_8085_state(state: &Processor8085) -> String{ 
    let output_string  =  format!(
        r#" 
|-------------|-------------|
|b: {:x}      | c: {:x}     | 
|-------------|-------------|
|d: {:x}      | e: {:x}     | 
|-------------|-------------|
|h: {:x}      | l: {:x}     | 
|-------------|-------------|
|stack pointer:  {:x}       | 
|-------------|-------------|
|program counter:  {:x}     | 
|-------------|-------------|
|s: {},z:{},AC:{},P:{},CY:{}| 
|-------------|-------------|
        "#
    ,state.b, state.c, state.d, state.e, state.h, state.l, state.stack_pointer, state.program_counter
    , state.flag.sign, state.flag.zero, state.flag.auxillary_carry, state.flag.parity, state.flag.carry);

    output_string
}

#[test]
fn parity_of_10() {
    //0b1010
    assert_eq!(check_parity(10,8),true);
}

#[test]
fn parity_of_59() {
    //0b111011
    assert_eq!(check_parity(59,8),false);
}



