/*
 * Author=> { 8085 Project Team
 *
 *
 */

// 8085 is little endian 

use crate::core::{Processor8085,CARRY};
use crate::utils::{check_parity, bool_to_bin};

//returns program counter
#[allow(unused_variables,unused_mut)]
pub fn emulate_8085(state:&mut Processor8085 ,mut pc: usize)  -> usize {

    let mut cycles:  u8 = 4;
    // for single byte code the offset is just one pc + 1 = next opcode

    let opcode: &Vec<u8>  = &state.memory;
    
    #[allow(unused_parens,unused_assignments)]
    match (opcode[pc]) {
    // NOP 
    0x00 => { 
       cycles = 0;
    }
    
    /* LXI portion */

    // LXI B, data
    // load data into BC register pair immediate
    0x01 =>  { 
        // due to little endianess 4455 is divided into 55 44 
        // least significant byte comes first
        // so c will hold least significant byte 
        // b will hold most significant byte
        state.c = opcode[pc+1];
        state.b = state.memory[pc+2];
        pc += 2;
    }
    //LXI D, data
    0x11 =>  { 
        state.e = opcode[pc+1];
        state.d = state.memory[pc+2];
        pc += 2;
    }
    //LXI H, data
    0x21 =>  { 
        state.l = opcode[pc+1];
        state.h = state.memory[pc+2];
        pc += 2;
    }
    //LXI SP, data 
    0x31 =>  { 
        state.stack_pointer = ((opcode[pc+2] as u16) << 8 | (opcode[pc+1] as u16));
        pc += 2;
    }

    // ADD portion
   0x80 => { // ADD B
        state.accumulator = add_byte(state, state.accumulator, state.b, CARRY::UPDATE_CARRY);
		}
	 0x81=> { // ADD C
        state.accumulator = add_byte(state, state.accumulator, state.c, CARRY::UPDATE_CARRY);
		}
	 0x82=> { // ADD D
        state.accumulator = add_byte(state, state.accumulator, state.d, CARRY::UPDATE_CARRY);
		}
	 0x83=> { // ADD E
        state.accumulator = add_byte(state, state.accumulator, state.e, CARRY::UPDATE_CARRY);
		}
	 0x84=> { // ADD H
        state.accumulator = add_byte(state, state.accumulator, state.h, CARRY::UPDATE_CARRY);
		}
	 0x85=> { // ADD L
        state.accumulator = add_byte(state, state.accumulator, state.l, CARRY::UPDATE_CARRY);
    }

    // SUB portion
   0x8f=> { // SUB B
        state.accumulator = subtract_byte(state, state.accumulator, state.b, CARRY::UPDATE_CARRY);
		}
	 0x90=> { // SUB C
        state.accumulator = subtract_byte(state, state.accumulator, state.c, CARRY::UPDATE_CARRY);
		}
	 0x91=> { // SUB D
        state.accumulator = subtract_byte(state, state.accumulator, state.d, CARRY::UPDATE_CARRY);
		}
	 0x92=> { // SUB E
        state.accumulator = subtract_byte(state, state.accumulator, state.e, CARRY::UPDATE_CARRY);
		}
	 0x93=> { // SUB H
        state.accumulator = subtract_byte(state, state.accumulator, state.h, CARRY::UPDATE_CARRY);
		}
	 0x94=> { // SUB L
        state.accumulator = subtract_byte(state, state.accumulator, state.l, CARRY::UPDATE_CARRY);
    }

   // ADC portion
   // add bytes with carry
   0x88=> { // ADC B
        state.accumulator = add_byte_with_carry(state, state.accumulator, state.b, CARRY::UPDATE_CARRY);
		}
	 0x89=> { // ADC C
        state.accumulator = add_byte_with_carry(state, state.accumulator, state.c, CARRY::UPDATE_CARRY);
		}
	 0x8a=> { // ADC D
        state.accumulator = add_byte_with_carry(state, state.accumulator, state.d, CARRY::UPDATE_CARRY);
		}
	 0x8b=> { // ADC E
        state.accumulator = add_byte_with_carry(state, state.accumulator, state.e, CARRY::UPDATE_CARRY);
		}
	 0x8c=> { // ADC H
        state.accumulator = add_byte_with_carry(state, state.accumulator, state.h, CARRY::UPDATE_CARRY);
		}
	 0x8d=> { // ADC L
        state.accumulator = add_byte_with_carry(state, state.accumulator, state.l, CARRY::UPDATE_CARRY);
    }

   // SBB portion
   // subtract bytes with carry
   0x98=> { // SBB B
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, state.b, CARRY::UPDATE_CARRY);
		}
	 0x99=> { // SBB C
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, state.c, CARRY::UPDATE_CARRY);
		}
	 0x9a=> { // SBB D
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, state.d, CARRY::UPDATE_CARRY);
		}
	 0x9b=> { // SBB E
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, state.e, CARRY::UPDATE_CARRY);
		}
	 0x9c=> { // SBB H
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, state.h, CARRY::UPDATE_CARRY);
		}
	 0x9d=> { // SBB L
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, state.l, CARRY::UPDATE_CARRY);
    }

   // ANA portion
   // ANA means AND with accumulator
   	 0xa0=> { // ANA B
		state.accumulator = state.accumulator & state.b;
		logic_flags(state);
		}
	 0xa1=> { // ANA C
		state.accumulator = state.accumulator & state.c;
		logic_flags(state);
		}
	 0xa2=> { // ANA D
		state.accumulator = state.accumulator & state.d;
		logic_flags(state);
		}
	 0xa3=> { // ANA E
		state.accumulator = state.accumulator & state.e;
		logic_flags(state);
		}
	 0xa4=> { // ANA H
		state.accumulator = state.accumulator & state.h;
		logic_flags(state);
		}
	 0xa5=> { // ANA L
		state.accumulator = state.accumulator & state.l;
		logic_flags(state);
		}

	 0xa7=> { // ANA A
		state.accumulator = state.accumulator & state.accumulator;
		logic_flags(state);
		}

   // XRA portion
   // XRA means XOR with accumulator
    0xa8=> { // XRA B
		state.accumulator = state.accumulator ^ state.b;
		logic_flags(state);
		}
	 0xa9=> { // XRA C
		state.accumulator = state.accumulator ^ state.c;
		logic_flags(state);
		}
	 0xaa=> { // XRA D
		state.accumulator = state.accumulator ^ state.d;
		logic_flags(state);
		}
	 0xab=> { // XRA E
		state.accumulator = state.accumulator ^ state.e;
		logic_flags(state);
		}
	 0xac=> { // XRA H
		state.accumulator = state.accumulator ^ state.h;
		logic_flags(state);
		}
	 0xad=> { // XRA L
		state.accumulator = state.accumulator ^ state.l;
		logic_flags(state);
		}
	 0xaf=> { // XRA A
		state.accumulator = state.accumulator ^ state.accumulator;
		logic_flags(state);
		}

   // ORA portion 
   // ORA means OR with accumulator
   
    0xb0=> { // ORA B
		state.accumulator = state.accumulator | state.b;
		logic_flags(state);
		}
	 0xb1=> { // ORA C
		state.accumulator = state.accumulator | state.c;
		logic_flags(state);
		}
	 0xb2=> { // ORA D
		state.accumulator = state.accumulator | state.d;
		logic_flags(state);
		}
	 0xb3=> { // ORA E
		state.accumulator = state.accumulator | state.e;
		logic_flags(state);
		}
	 0xb4=> { // ORA H
		state.accumulator = state.accumulator | state.h;
		logic_flags(state);
		}
	 0xb5=> { // ORA L
		state.accumulator = state.accumulator | state.l;
		logic_flags(state);
		}

	 0xb7=> { // ORA A
		state.accumulator = state.accumulator | state.accumulator;
		logic_flags(state);
		}

   //CMP
   //compare with accumulator
   0xb8=> { // CMP B
        subtract_byte(state, state.accumulator, state.b, CARRY::UPDATE_CARRY);
		}
	 0xb9=> { // CMP C
        subtract_byte(state, state.accumulator, state.c, CARRY::UPDATE_CARRY);
		}
	 0xba=> { // CMP D
        subtract_byte(state, state.accumulator, state.d, CARRY::UPDATE_CARRY);
		}
	 0xbb=> { // CMP E
        subtract_byte(state, state.accumulator, state.e, CARRY::UPDATE_CARRY);
		}
	 0xbc=> { // CMP H
        subtract_byte(state, state.accumulator, state.h, CARRY::UPDATE_CARRY);
		}
	 0xbd=> { // CMP L
        subtract_byte(state, state.accumulator, state.l, CARRY::UPDATE_CARRY);
    }
	 0xbf=> { // CMP A
        subtract_byte(state, state.accumulator, state.accumulator, CARRY::UPDATE_CARRY);
    }
    
   // ADI  data
   // Add immediate data to accumulator
   0xc6 => {
       let rhs = opcode[1];
        state.accumulator = add_byte(state, state.accumulator, rhs, CARRY::UPDATE_CARRY);
   }

   // ACI data 
   // add immediate data to accumulator + carry
   0xce => {
       let rhs = opcode[1];
       state.accumulator = add_byte_with_carry(state, state.accumulator, rhs, CARRY::UPDATE_CARRY);
   }


   // SUI data 
   // subtract immediate data to accumulator
   0xd6 => {
       let rhs = opcode[1];
        state.accumulator = subtract_byte(state, state.accumulator, rhs, CARRY::UPDATE_CARRY);
   }

   // SBI data 
   // subtract immediate data to accumulator with borrow
   0xde => {
       let rhs = opcode[1];
        state.accumulator = subtract_byte_with_borrow(state, state.accumulator, rhs, CARRY::UPDATE_CARRY);
   }

   // ANI data
   // AND immediate with accumulator
   0xe6 => {
       state.accumulator = state.accumulator & opcode[1] as u8;
       logic_flags(state);
   }
   
   // XRI data
   // XOR immediate with accumulator
   0xee => {
       state.accumulator = state.accumulator ^ opcode[1] as u8;
       logic_flags(state);
   }
   // ORI data
   // OR immediate with accumulator
    0xf6 => {
       state.accumulator = state.accumulator | opcode[1] as u8;
       logic_flags(state);
   }

   // CPI Data
   // compare immediate with accumulator
    0xfe => {
        let rhs = opcode[1];
        subtract_byte(state, state.accumulator, rhs, CARRY::UPDATE_CARRY);
   }


    // REGM8 
    

    // RLC 
    // rotate left accumulator 
    // 11101010 becomes 01110101
    // only carry flag is affected
    //https://www.daenotes.com/electronics/digital-electronics/instruction-set-intel-8085
    0x07 => {
        state.flag.carry = if state.accumulator & 0x01 == 1 {true} else {false};
        state.accumulator = (state.accumulator >> 1) | (state.accumulator << 7);
    }
    // RAL 
    // rotate left accumulator through carry flag
    // D7 is placed in carry flag and D0 will hold carry flag
    0x17 => {
        let buffer = state.accumulator;
        let mut carry_data: u8= if state.flag.carry == true {1}  else {0};
        carry_data= carry_data << 7;
        state.accumulator = (state.accumulator >> 1) | carry_data;
        state.flag.carry = if buffer & 0x01 == 1 {true} else {false};
    }

    //RRC 
    //roate accumulator to right
    0x0f => {
        state.flag.carry = if state.accumulator & 0b10000000 == 1 {true} else {false};
        state.accumulator = (state.accumulator << 1) | (state.accumulator >> 7);
    }


    // RAR 
    // rotate accumulator to right but carry flag comes to play 
    0x1f => {
        let buffer = state.accumulator;
        let mut carry_data: u8= if state.flag.carry == true {1}  else {0};
        state.accumulator = (state.accumulator << 1) | carry_data;
        state.flag.carry = if buffer & 0b10000000 == 1 {true} else {false};
    }
    

    // CMA 
    // contents of accumulator is complemented
    0x2f => {
        state.accumulator = !state.accumulator;
    }

    // CMC 
    // carry flag is complemented
    0x3f => {
        state.flag.carry = !state.flag.carry; 
    }
    
    // STC
    // set carry flag. makes carry flag : 1
    0x37 => {
        state.flag.carry = true;
    }

    // INR portion


    // INR B
    0x04 => {
        state.b += 1;
    }
    // INR C
    0x0c => {
        state.c += 1;
    }
    // INR D
    0x14 => {
        state.d +=1; 
    }
    // INR E
    0x1c => {
        state.e += 1;
    } 
    // INR H
    0x24 => {
        state.h += 1;
    }
    // INR L
    0x2c => {
        state.l += 1;
    }
    // INR M (H and L pair)
    0x34 => {
        let buffer: u16 = (state.h as u16) << 8 | state.l as u16;
        state.memory[buffer as usize] = add_byte(state,state.memory[buffer as usize], 1, CARRY::PRESERVE_CARRY);
    }
    // INR A
    0x3c => {
        state.accumulator += 1;
    }

    // DCR portion

   // DCR B
    0x05 => {
        state.b -= 1;
    }
    // DCR C
    0x0d => {
        state.c -= 1;
    }
    // DCR D
    0x15 => {
        state.d -=1; 
    }
    // DCR E
    0x1d => {
        state.e -= 1;
    } 
    // DCR H
    0x25 => {
        state.h -= 1;
    }
    // DCR L
    0x2d => {
        state.l -= 1;
    }
    // DCR M (H and L pair)
    0x35 => {
        let buffer: u16 = (state.h as u16) << 8 | state.l as u16;
        state.memory[buffer as usize] = subtract_byte(state,state.memory[buffer as usize], 1, CARRY::PRESERVE_CARRY);
    }
    // DCR A
    0x3d => {
        state.accumulator -= 1;
    }

    // INX B
    0x03 => {
        state.c += 1;
        if state.c == 0 {
            state.b += 1;
        }
    }

    // INX D
    0x13 => {
        state.d += 1;
        if state.d == 0 {
            state.e += 1;
        }
    }

    // INX H
    0x23 => {
        state.h += 1;
        if state.h  == 0 {
            state.l += 1;
        }
    }

    // INX SP
    0x33 => {
        state.stack_pointer+=1;
    }
    
    // DCX B
    0x0b => {
        state.b -= 1;
        if state.b  == 0 {
            state.c -= 1;
        }
    }

    // DCX D
    0x1b => {
        state.d -= 1;
        if state.d  == 0 {
            state.e -= 1;
        }

    }
    // DCX H
    0x2b => {
        state.h -= 1;
        if state.h  == 0 {
            state.l -= 1;
        }

    }
    // DCX SP
    0x3b => {
        state.stack_pointer -= 1;
    }



    // DAD (Double Register ADD)
    // DAD B
    0x09 => {
        let hl_register_pair_buffer: u16 = (state.h as u16) << 8 | state.l as u16;
        let bc_register_pair_buffer: u16 = (state.b as u16) << 8 | state.c as u16;

        let result: u32 = hl_register_pair_buffer as u32 + bc_register_pair_buffer as u32;
        state.h = ((result & 0xff00) >> 8) as u8;
        state.l = (result & 0x00ff)  as u8;
        state.flag.carry = if (result > 0xffff) {true} else {false};
    }
    // DAD D
    0x19 => {
        let hl_register_pair_buffer: u16 = (state.h as u16) << 8 | state.l as u16;
        let de_register_pair_buffer: u16 = (state.d as u16) << 8 | state.e as u16;

        let result: u32 = hl_register_pair_buffer as u32 + de_register_pair_buffer as u32;
        state.h = ((result & 0xff00) >> 8) as u8;
        state.l = (result & 0x00ff)  as u8;
        state.flag.carry = if (result > 0xffff) {true} else {false};
    }
    // DAD H
    0x29 => {
        let hl_register_pair_buffer: u16 = (state.h as u16) << 8 | state.l as u16;

        let result: u32 = hl_register_pair_buffer as u32 + hl_register_pair_buffer as u32;
        state.h = ((result & 0xff00) >> 8) as u8;
        state.l = (result & 0x00ff)  as u8;
        state.flag.carry = if (result > 0xffff) {true} else {false};
    }
    // DAD SP
    0x39 => {
        let hl_register_pair_buffer: u16 = (state.h as u16) << 8 | state.l as u16;

        let result: u32 = hl_register_pair_buffer as u32 + state.stack_pointer as u32;
        state.h = ((result & 0xff00) >> 8) as u8;
        state.l = (result & 0x00ff)  as u8;
        state.flag.carry = if (result > 0xffff) {true} else {false};
    }

    
    // PUSH portion
    // push B
    0xc5 => {
        state.memory[(state.stack_pointer-1) as usize] = state.b;
        state.memory[(state.stack_pointer-2) as usize] = state.c;
        state.stack_pointer -= 2;
    }
    //PUSH D
    0xd5 =>  {
        state.memory[(state.stack_pointer-1) as usize] = state.d;
        state.memory[(state.stack_pointer-2) as usize] = state.e;
        state.stack_pointer -= 2;
    }

    //PUSH H
    0xe5 =>  {
        state.memory[(state.stack_pointer-1) as usize] = state.h;
        state.memory[(state.stack_pointer-2) as usize] = state.l;
        state.stack_pointer -= 2;
    }

    //PUSH PSW
    0xf5 =>  {
        let psw = bool_to_bin(state.flag.zero) |
                  bool_to_bin(state.flag.sign) << 1 |
                  bool_to_bin(state.flag.parity) << 2 |
                  bool_to_bin(state.flag.carry) << 3 |
                  bool_to_bin(state.flag.auxillary_carry) << 4;
 
                state.memory[(state.stack_pointer-1) as usize] = state.accumulator;
        state.memory[(state.stack_pointer-2) as usize] = psw;
        state.stack_pointer -= 2;

    }

    // POP 
    // POP B
    0xc1 => {
       state.b = state.memory[state.stack_pointer as usize] ;
       state.c = state.memory[(state.stack_pointer + 1) as usize] ;
       state.stack_pointer += 2;
    }
    // POP D
    0xd1 => {
       state.d = state.memory[state.stack_pointer as usize] ;
       state.e = state.memory[(state.stack_pointer + 1) as usize] ;
       state.stack_pointer += 2;

    }
    // POP H
    0xe1 => {
       state.h = state.memory[state.stack_pointer as usize] ;
       state.l = state.memory[(state.stack_pointer + 1) as usize] ;
       state.stack_pointer += 2;

    }
    // POP PSW
    0xf1 => {
        state.accumulator = state.memory[state.stack_pointer as usize];
        let psw= state.memory[(state.stack_pointer + 1) as usize] ;
        state.flag.zero = (0b00000001 & psw) == 1;
        state.flag.sign = (0b00000010 & psw) == 0b10;
        state.flag.parity = (0b00000100 & psw) == 0b100;
        state.flag.carry = (0b00001000 & psw) == 0b1000;
        state.flag.auxillary_carry = (0b00010000 & psw) == 0b10000;
    } 

    // XTHL
    // H and L will hold contents of stack pointer
    // stack pointer will hold the contents of H and L
    // SWAP basically
    0xe3 => {
        let b_sp = state.memory[state.stack_pointer as usize];
        //most significant bit stackpointer buffer
        let b_sp_msb = state.memory[(state.stack_pointer+1) as usize];

        state.memory[state.stack_pointer as usize] = state.l;
        state.memory[(state.stack_pointer+1) as usize] = state.h;

        state.l = b_sp;
        state.h = b_sp_msb;

    }
    // SPHL
    //store contents of H and L on Stack pointer
    0xf9 => {
        state.stack_pointer = ((state.h as u16) << 8) | state.l as u16;
    }

    // STAX B
    // store the contents of accumulator to  
    0x02 => {
        //
    }
    _ => {
        cycles = 0;
    }

    }

   pc += 1;
   pc as usize
}

fn add_byte(state: &mut Processor8085, lhs: u8, rhs: u8, carry: CARRY) -> u8{
    let res: u16 = lhs as u16 + rhs as u16;  
    arithmetic_flag(state, res, carry);
    if (lhs & 0xf) + (rhs & 0xf) > 0xf {
        state.flag.auxillary_carry = true;
    }
    res as u8
}

fn add_byte_with_carry(state: &mut Processor8085, lhs: u8, rhs: u8, carry: CARRY) -> u8 {
    let res: u16 = lhs as u16 + rhs as u16 + (if  state.flag.carry == true { 1 } else {0}) ;  
    arithmetic_flag(state, res, carry);
    if (lhs & 0xf) + (!rhs & 0xf) + 1> 0xf {
        state.flag.auxillary_carry = true;
    }
    res as u8
}

fn subtract_byte_with_borrow(state: &mut Processor8085, lhs: u8, rhs: u8, carry: CARRY) -> u8{
    let res: u16 = lhs as u16 - rhs as u16 - (if state.flag.carry == true {1} else {0});  
    arithmetic_flag(state, res, carry);
    if (lhs & 0xf) + (!rhs & 0xf) + 1> 0xf {
        state.flag.auxillary_carry = true;
    }
    res as u8
}

fn subtract_byte(state: &mut Processor8085, lhs: u8, rhs: u8, carry: CARRY) -> u8{
    let res: u16 = lhs as u16 - rhs as u16;  
    arithmetic_flag(state, res, carry);
    if (lhs & 0xf) + (!rhs & 0xf) + 1> 0xf {
        state.flag.auxillary_carry = true;
    }
    res as u8
}


fn arithmetic_flag(state: &mut Processor8085, res: u16, carry: CARRY)  {
    match carry {
        CARRY::UPDATE_CARRY => state.flag.carry = res > 0xff,
        _ => {}
    }
    state.flag.zero = res & 0xff == 0;
    //128 in hex
    state.flag.sign = 0x80 == (res & 0x80);
    // lower bit or MSB
    state.flag.parity = check_parity((res & 0xff) as u8, 8);
}

fn logic_flags(state:&mut Processor8085) {
    state.flag.carry = false;
    state.flag.auxillary_carry = false;
    state.flag.zero = state.accumulator == 0;
    state.flag.sign = 0x80 == (state.accumulator & 0x80);
    state.flag.parity = check_parity(state.accumulator, 8);
}
