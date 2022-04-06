#[derive(Debug)]
pub enum Op {
    OpRR(String, String, String), //opcode register, register
    OpR(String, String),          // opcode register
    OpRA(String, String, i32),    // opcode register, address
    OpA(String, i32),             //opcode address
    Op(String),                   //opcode only
    OpL(String, String),          //opcode and Label
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Opcode {
    pub Label: Option<String>,
    pub Op: Op,
}
