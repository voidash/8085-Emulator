/*
 * Author: 8085 Project Team
 *
 *
 */
// 8085 is little endian 


pub fn disassemble_data(codebuffer : Vec<u8> ,program_counter: usize) -> (String, u8) {
    let mut opcode_bytes: u8 = 1;
    let mut equivalent_instruction = String::new(); 

    #[allow(unused_parens)]
    match (codebuffer[program_counter]) {
    0x00 => {
        equivalent_instruction =  format!("NOP");
    }
     0x01 => {
        equivalent_instruction = format!("LXI  B, {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x02 => {
		equivalent_instruction = format!("STAX   B");
		}
	 0x03 => {
		equivalent_instruction = format!("INX    B");
		}
	 0x04 => {
		equivalent_instruction = format!("INR    B");
		}
	 0x05 => {
		equivalent_instruction = format!("DCR    B");
		}
	 0x06 => {
		equivalent_instruction = format!("MVI    B,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x07 => {
		equivalent_instruction = format!("RLC");
		}
	 0x08 => {
		equivalent_instruction = format!("NOP");
		}
	 0x09 => {
		equivalent_instruction = format!("DAD    B");
		}
	 0x0a => {
		equivalent_instruction = format!("LDAX   B");
		}
	 0x0b => {
		equivalent_instruction = format!("DCX    B");
		}
	 0x0c => {
		equivalent_instruction = format!("INR    C");
		}
	 0x0d => {
		equivalent_instruction = format!("DCR    C");
		}
	 0x0e => {
		equivalent_instruction = format!("MVI    C,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x0f => {
		equivalent_instruction = format!("RRC");
		}

	 0x10 => {
		equivalent_instruction = format!("NOP");
		}
	 0x11 => {
		equivalent_instruction = format!("LXI    D,{:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x12 => {
		equivalent_instruction = format!("STAX   D");
		}
	 0x13 => {
		equivalent_instruction = format!("INX    D");
		}
	 0x14 => {
		equivalent_instruction = format!("INR    D");
		}
	 0x15 => {
		equivalent_instruction = format!("DCR    D");
		}
	 0x16 => {
		equivalent_instruction = format!("MVI    D,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x17 => {
		equivalent_instruction = format!("RAL");
		}
	 0x18 => {
		equivalent_instruction = format!("NOP");
		}
	 0x19 => {
		equivalent_instruction = format!("DAD    D");
		}
	 0x1a => {
		equivalent_instruction = format!("LDAX   D");
		}
	 0x1b => {
		equivalent_instruction = format!("DCX    D");
		}
	 0x1c => {
		equivalent_instruction = format!("INR    E");
		}
	 0x1d => {
		equivalent_instruction = format!("DCR    E");
		}
	 0x1e => {
		equivalent_instruction = format!("MVI    E,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x1f => {
		equivalent_instruction = format!("RAR");
		}

	 0x20 => {
		equivalent_instruction = format!("NOP");
		}
	 0x21 => {
		equivalent_instruction = format!("LXI    H,{:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x22 => {
		equivalent_instruction = format!("SHLD   {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x23 => {
		equivalent_instruction = format!("INX    H");
		}
	 0x24 => {
		equivalent_instruction = format!("INR    H");
		}
	 0x25 => {
		equivalent_instruction = format!("DCR    H");
		}
	 0x26 => {
		equivalent_instruction = format!("MVI    H,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x27 => {
		equivalent_instruction = format!("DAA");
		}
	 0x28 => {
		equivalent_instruction = format!("NOP");
		}
	 0x29 => {
		equivalent_instruction = format!("DAD    H");
		}
	 0x2a => {
		equivalent_instruction = format!("LHLD   {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x2b => {
		equivalent_instruction = format!("DCX    H");
		}
	 0x2c => {
		equivalent_instruction = format!("INR    L");
		}
	 0x2d => {
		equivalent_instruction = format!("DCR    L");
		}
	 0x2e => {
		equivalent_instruction = format!("MVI    L,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x2f => {
		equivalent_instruction = format!("CMA");
		}

	 0x30 => {
		equivalent_instruction = format!("NOP");
		}
	 0x31 => {
		equivalent_instruction = format!("LXI    SP,{:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x32 => {
		equivalent_instruction = format!("STA    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x33 => {
		equivalent_instruction = format!("INX    SP");
		}
	 0x34 => {
		equivalent_instruction = format!("INR    M");
		}
	 0x35 => {
		equivalent_instruction = format!("DCR    M");
		}
	 0x36 => {
		equivalent_instruction = format!("MVI    M,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x37 => {
		equivalent_instruction = format!("STC");
		}
	 0x38 => {
		equivalent_instruction = format!("NOP");
		}
	 0x39 => {
		equivalent_instruction = format!("DAD    SP");
		}
	 0x3a => {
		equivalent_instruction = format!("LDA    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0x3b => {
		equivalent_instruction = format!("DCX    SP");
		}
	 0x3c => {
		equivalent_instruction = format!("INR    A");
		}
	 0x3d => {
		equivalent_instruction = format!("DCR    A");
		}
	 0x3e => {
		equivalent_instruction = format!("MVI    A,{:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0x3f => {
		equivalent_instruction = format!("CMC");
		}

	 0x40 => {
		equivalent_instruction = format!("MOV    B,B");
		}
	 0x41 => {
		equivalent_instruction = format!("MOV    B,C");
		}
	 0x42 => {
		equivalent_instruction = format!("MOV    B,D");
		}
	 0x43 => {
		equivalent_instruction = format!("MOV    B,E");
		}
	 0x44 => {
		equivalent_instruction = format!("MOV    B,H");
		}
	 0x45 => {
		equivalent_instruction = format!("MOV    B,L");
		}
	 0x46 => {
		equivalent_instruction = format!("MOV    B,M");
		}
	 0x47 => {
		equivalent_instruction = format!("MOV    B,A");
		}
	 0x48 => {
		equivalent_instruction = format!("MOV    C,B");
		}
	 0x49 => {
		equivalent_instruction = format!("MOV    C,C");
		}
	 0x4a => {
		equivalent_instruction = format!("MOV    C,D");
		}
	 0x4b => {
		equivalent_instruction = format!("MOV    C,E");
		}
	 0x4c => {
		equivalent_instruction = format!("MOV    C,H");
		}
	 0x4d => {
		equivalent_instruction = format!("MOV    C,L");
		}
	 0x4e => {
		equivalent_instruction = format!("MOV    C,M");
		}
	 0x4f => {
		equivalent_instruction = format!("MOV    C,A");
		}

	 0x50 => {
		equivalent_instruction = format!("MOV    D,B");
		}
	 0x51 => {
		equivalent_instruction = format!("MOV    D,C");
		}
	 0x52 => {
		equivalent_instruction = format!("MOV    D,D");
		}
	 0x53 => {
		equivalent_instruction = format!("MOV    D.E");
		}
	 0x54 => {
		equivalent_instruction = format!("MOV    D,H");
		}
	 0x55 => {
		equivalent_instruction = format!("MOV    D,L");
		}
	 0x56 => {
		equivalent_instruction = format!("MOV    D,M");
		}
	 0x57 => {
		equivalent_instruction = format!("MOV    D,A");
		}
	 0x58 => {
		equivalent_instruction = format!("MOV    E,B");
		}
	 0x59 => {
		equivalent_instruction = format!("MOV    E,C");
		}
	 0x5a => {
		equivalent_instruction = format!("MOV    E,D");
		}
	 0x5b => {
		equivalent_instruction = format!("MOV    E,E");
		}
	 0x5c => {
		equivalent_instruction = format!("MOV    E,H");
		}
	 0x5d => {
		equivalent_instruction = format!("MOV    E,L");
		}
	 0x5e => {
		equivalent_instruction = format!("MOV    E,M");
		}
	 0x5f => {
		equivalent_instruction = format!("MOV    E,A");
		}

	 0x60 => {
		equivalent_instruction = format!("MOV    H,B");
		}
	 0x61 => {
		equivalent_instruction = format!("MOV    H,C");
		}
	 0x62 => {
		equivalent_instruction = format!("MOV    H,D");
		}
	 0x63 => {
		equivalent_instruction = format!("MOV    H.E");
		}
	 0x64 => {
		equivalent_instruction = format!("MOV    H,H");
		}
	 0x65 => {
		equivalent_instruction = format!("MOV    H,L");
		}
	 0x66 => {
		equivalent_instruction = format!("MOV    H,M");
		}
	 0x67 => {
		equivalent_instruction = format!("MOV    H,A");
		}
	 0x68 => {
		equivalent_instruction = format!("MOV    L,B");
		}
	 0x69 => {
		equivalent_instruction = format!("MOV    L,C");
		}
	 0x6a => {
		equivalent_instruction = format!("MOV    L,D");
		}
	 0x6b => {
		equivalent_instruction = format!("MOV    L,E");
		}
	 0x6c => {
		equivalent_instruction = format!("MOV    L,H");
		}
	 0x6d => {
		equivalent_instruction = format!("MOV    L,L");
		}
	 0x6e => {
		equivalent_instruction = format!("MOV    L,M");
		}
	 0x6f => {
		equivalent_instruction = format!("MOV    L,A");
		}

	 0x70 => {
		equivalent_instruction = format!("MOV    M,B");
		}
	 0x71 => {
		equivalent_instruction = format!("MOV    M,C");
		}
	 0x72 => {
		equivalent_instruction = format!("MOV    M,D");
		}
	 0x73 => {
		equivalent_instruction = format!("MOV    M.E");
		}
	 0x74 => {
		equivalent_instruction = format!("MOV    M,H");
		}
	 0x75 => {
		equivalent_instruction = format!("MOV    M,L");
		}
	 0x76 => {
		equivalent_instruction = format!("HLT");
		}
	 0x77 => {
		equivalent_instruction = format!("MOV    M,A");
		}
	 0x78 => {
		equivalent_instruction = format!("MOV    A,B");
		}
	 0x79 => {
		equivalent_instruction = format!("MOV    A,C");
		}
	 0x7a => {
		equivalent_instruction = format!("MOV    A,D");
		}
	 0x7b => {
		equivalent_instruction = format!("MOV    A,E");
		}
	 0x7c => {
		equivalent_instruction = format!("MOV    A,H");
		}
	 0x7d => {
		equivalent_instruction = format!("MOV    A,L");
		}
	 0x7e => {
		equivalent_instruction = format!("MOV    A,M");
		}
	 0x7f => {
		equivalent_instruction = format!("MOV    A,A");
		}

	 0x80 => {
		equivalent_instruction = format!("ADD    B");
		}
	 0x81 => {
		equivalent_instruction = format!("ADD    C");
		}
	 0x82 => {
		equivalent_instruction = format!("ADD    D");
		}
	 0x83 => {
		equivalent_instruction = format!("ADD    E");
		}
	 0x84 => {
		equivalent_instruction = format!("ADD    H");
		}
	 0x85 => {
		equivalent_instruction = format!("ADD    L");
		}
	 0x86 => {
		equivalent_instruction = format!("ADD    M");
		}
	 0x87 => {
		equivalent_instruction = format!("ADD    A");
		}
	 0x88 => {
		equivalent_instruction = format!("ADC    B");
		}
	 0x89 => {
		equivalent_instruction = format!("ADC    C");
		}
	 0x8a => {
		equivalent_instruction = format!("ADC    D");
		}
	 0x8b => {
		equivalent_instruction = format!("ADC    E");
		}
	 0x8c => {
		equivalent_instruction = format!("ADC    H");
		}
	 0x8d => {
		equivalent_instruction = format!("ADC    L");
		}
	 0x8e => {
		equivalent_instruction = format!("ADC    M");
		}
	 0x8f => {
		equivalent_instruction = format!("ADC    A");
		}

	 0x90 => {
		equivalent_instruction = format!("SUB    B");
		}
	 0x91 => {
		equivalent_instruction = format!("SUB    C");
		}
	 0x92 => {
		equivalent_instruction = format!("SUB    D");
		}
	 0x93 => {
		equivalent_instruction = format!("SUB    E");
		}
	 0x94 => {
		equivalent_instruction = format!("SUB    H");
		}
	 0x95 => {
		equivalent_instruction = format!("SUB    L");
		}
	 0x96 => {
		equivalent_instruction = format!("SUB    M");
		}
	 0x97 => {
		equivalent_instruction = format!("SUB    A");
		}
	 0x98 => {
		equivalent_instruction = format!("SBB    B");
		}
	 0x99 => {
		equivalent_instruction = format!("SBB    C");
		}
	 0x9a => {
		equivalent_instruction = format!("SBB    D");
		}
	 0x9b => {
		equivalent_instruction = format!("SBB    E");
		}
	 0x9c => {
		equivalent_instruction = format!("SBB    H");
		}
	 0x9d => {
		equivalent_instruction = format!("SBB    L");
		}
	 0x9e => {
		equivalent_instruction = format!("SBB    M");
		}
	 0x9f => {
		equivalent_instruction = format!("SBB    A");
		}

	 0xa0 => {
		equivalent_instruction = format!("ANA    B");
		}
	 0xa1 => {
		equivalent_instruction = format!("ANA    C");
		}
	 0xa2 => {
		equivalent_instruction = format!("ANA    D");
		}
	 0xa3 => {
		equivalent_instruction = format!("ANA    E");
		}
	 0xa4 => {
		equivalent_instruction = format!("ANA    H");
		}
	 0xa5 => {
		equivalent_instruction = format!("ANA    L");
		}
	 0xa6 => {
		equivalent_instruction = format!("ANA    M");
		}
	 0xa7 => {
		equivalent_instruction = format!("ANA    A");
		}
	 0xa8 => {
		equivalent_instruction = format!("XRA    B");
		}
	 0xa9 => {
		equivalent_instruction = format!("XRA    C");
		}
	 0xaa => {
		equivalent_instruction = format!("XRA    D");
		}
	 0xab => {
		equivalent_instruction = format!("XRA    E");
		}
	 0xac => {
		equivalent_instruction = format!("XRA    H");
		}
	 0xad => {
		equivalent_instruction = format!("XRA    L");
		}
	 0xae => {
		equivalent_instruction = format!("XRA    M");
		}
	 0xaf => {
		equivalent_instruction = format!("XRA    A");
		}

	 0xb0 => {
		equivalent_instruction = format!("ORA    B");
		}
	 0xb1 => {
		equivalent_instruction = format!("ORA    C");
		}
	 0xb2 => {
		equivalent_instruction = format!("ORA    D");
		}
	 0xb3 => {
		equivalent_instruction = format!("ORA    E");
		}
	 0xb4 => {
		equivalent_instruction = format!("ORA    H");
		}
	 0xb5 => {
		equivalent_instruction = format!("ORA    L");
		}
	 0xb6 => {
		equivalent_instruction = format!("ORA    M");
		}
	 0xb7 => {
		equivalent_instruction = format!("ORA    A");
		}
	 0xb8 => {
		equivalent_instruction = format!("CMP    B");
		}
	 0xb9 => {
		equivalent_instruction = format!("CMP    C");
		}
	 0xba => {
		equivalent_instruction = format!("CMP    D");
		}
	 0xbb => {
		equivalent_instruction = format!("CMP    E");
		}
	 0xbc => {
		equivalent_instruction = format!("CMP    H");
		}
	 0xbd => {
		equivalent_instruction = format!("CMP    L");
		}
	 0xbe => {
		equivalent_instruction = format!("CMP    M");
		}
	 0xbf => {
		equivalent_instruction = format!("CMP    A");
		}

	 0xc0 => {
		equivalent_instruction = format!("RNZ");
		}
	 0xc1 => {
		equivalent_instruction = format!("POP    B");
		}
	 0xc2 => {
		equivalent_instruction = format!("JNZ    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xc3 => {
		equivalent_instruction = format!("JMP    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xc4 => {
		equivalent_instruction = format!("CNZ    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xc5 => {
		equivalent_instruction = format!("PUSH   B");
		}
	 0xc6 => {
		equivalent_instruction = format!("ADI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xc7 => {
		equivalent_instruction = format!("RST    0");
		}
	 0xc8 => {
		equivalent_instruction = format!("RZ");
		}
	 0xc9 => {
		equivalent_instruction = format!("RET");
		}
	 0xca => {
		equivalent_instruction = format!("JZ     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xcb => {
		equivalent_instruction = format!("JMP    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xcc => {
		equivalent_instruction = format!("CZ     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xcd => {
		equivalent_instruction = format!("CALL   {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xce => {
		equivalent_instruction = format!("ACI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xcf => {
		equivalent_instruction = format!("RST    1");
		}

	 0xd0 => {
		equivalent_instruction = format!("RNC");
		}
	 0xd1 => {
		equivalent_instruction = format!("POP    D");
		}
	 0xd2 => {
		equivalent_instruction = format!("JNC    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xd3 => {
		equivalent_instruction = format!("OUT    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xd4 => {
		equivalent_instruction = format!("CNC    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xd5 => {
		equivalent_instruction = format!("PUSH   D");
		}
	 0xd6 => {
		equivalent_instruction = format!("SUI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xd7 => {
		equivalent_instruction = format!("RST    2");
		}
	 0xd8 => {
		equivalent_instruction = format!("RC");
		}
	 0xd9 => {
		equivalent_instruction = format!("RET");
		}
	 0xda => {
		equivalent_instruction = format!("JC     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xdb => {
		equivalent_instruction = format!("IN     {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xdc => {
		equivalent_instruction = format!("CC     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xdd => {
		equivalent_instruction = format!("CALL   {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xde => {
		equivalent_instruction = format!("SBI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xdf => {
		equivalent_instruction = format!("RST    3");
		}

	 0xe0 => {
		equivalent_instruction = format!("RPO");
		}
	 0xe1 => {
		equivalent_instruction = format!("POP    H");
		}
	 0xe2 => {
		equivalent_instruction = format!("JPO    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xe3 => {
		equivalent_instruction = format!("XTHL");
		}
	 0xe4 => {
		equivalent_instruction = format!("CPO    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xe5 => {
		equivalent_instruction = format!("PUSH   H");
		}
	 0xe6 => {
		equivalent_instruction = format!("ANI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xe7 => {
		equivalent_instruction = format!("RST    4");
		}
	 0xe8 => {
		equivalent_instruction = format!("RPE");
		}
	 0xe9 => {
		equivalent_instruction = format!("PCHL");
		}
	 0xea => {
		equivalent_instruction = format!("JPE    {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xeb => {
		equivalent_instruction = format!("XCHG");
		}
	 0xec => {
		equivalent_instruction = format!("CPE     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xed => {
		equivalent_instruction = format!("CALL   {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xee => {
		equivalent_instruction = format!("XRI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xef => {
		equivalent_instruction = format!("RST    5");
		}

	 0xf0 => {
		equivalent_instruction = format!("RP");
		}
	 0xf1 => {
		equivalent_instruction = format!("POP    PSW");
		}
	 0xf2 => {
		equivalent_instruction = format!("JP     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xf3 => {
		equivalent_instruction = format!("DI");
		}
	 0xf4 => {
		equivalent_instruction = format!("CP     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xf5 => {
		equivalent_instruction = format!("PUSH   PSW");
		}
	 0xf6 => {
		equivalent_instruction = format!("ORI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
	 0xf7 => {
		equivalent_instruction = format!("RST    6");
		}
	 0xf8 => {
		equivalent_instruction = format!("RM");
		}
	 0xf9 => {
		equivalent_instruction = format!("SPHL");
		}
	 0xfa => {
		equivalent_instruction = format!("JM     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xfb => {
		equivalent_instruction = format!("EI");
		}
	 0xfc => {
		equivalent_instruction = format!("CM     {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xfd => {
		equivalent_instruction = format!("CALL  {:02x}{:02x}", codebuffer[program_counter+2], codebuffer[program_counter+1]);
		opcode_bytes = 3;
		}
	 0xfe => {
		equivalent_instruction = format!("CPI    {:02x}", codebuffer[program_counter+1]);
		opcode_bytes = 2;
		}
    _ => {
        opcode_bytes = 2;
    }
}

    (equivalent_instruction,opcode_bytes)
}


#[test]
fn check_disassembly_nop() {
    let data = vec![0x00];
    assert_eq!(disassemble_data(data, 0),(String::from("NOP"),1));
}

#[test]
#[allow(non_snake_case)]
fn check_disassembly_MVI_B_23H() {
    let data = vec![0x06,0x23];
    assert_eq!(disassemble_data(data, 0),(String::from("MVI    B,23"),2));
}

#[test]
#[allow(non_snake_case)]
fn check_disassembly() {
    let data = vec![0x01,0x23,0x34];
    assert_eq!(disassemble_data(data, 0),(String::from("LXI  B,3423"),3));
}


