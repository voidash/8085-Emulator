## How assembler works

![assembler_output](assets/assember_output.png)

- object file
		Object file is what is loaded to ROM. 8085 is 40 pinned because  the intellec developement system  only supported 40 pins.

- program listing
		record for source program and object code. Think of it as table 

- cross reference listing
		another record that is one of the diagnostic tools provided by the assembler.  

## 8085 

- memory
- Program counter
- Work registers
- Condition flags
- Stack and stack pointer
- IO ports
- instruction set

### Registers 8085

![internal registers of 8085](assets/architecture.png)
Internal registers of 8085

![sybmolic name of registers](assets/sybmolic_name_of_registers.png)


#### Condition flags

Carry , sign , zero , parity and Auxillary Carry.

#### Addressing Modes

- *Implied Addressing* : STC, DAA instructions are implied by the instruction function
- *Register Addressing*: CMP E. specify one regiser, another one is by default accumulator so it deals with 8-bit values. Except some like `PCHL` which exchanges the contents of program counter with H and L register.
- *Immediate Addressing*: MVI D, OFFH, LXI SP,30FFH (16bit load). Specify register , then what to load.
- *Direct Addressing*: JMP 1000H  
- *Register Indirect addressing* : references memory by a register pair.  MOV M,C moves the contents of the C register to memory address stored in the H and L pair.

#### Instruction Naming conventions

![instruction_conventions](assets/instruction_conventions.png)
![register_pair_name_convention](assets/register_pair_name_convention.png)
![arithmetic_name_convention](assets/arithmetic_name_convention.png)
![conditional](assets/conditional.png)


![hardware_instruction_summary](assets/harware_isntruction_summary.png)

## Concepts of Assembly Language

![source_format](assets/source_format.png)
- character set
- Delimiters: characters with special meaning to the assembler. Delimiters define the end of source statement, a field or component.


#### Operand field information
![operand field information](assets/operand_field_information.png)

#### Two's complement representation of data 

#### Symbolic Addressing
