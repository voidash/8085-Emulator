mod utils;
use emulator::core::Processor8085;
use emulator::generate_assembly_code;
use emulator::emulator::emulate_8085;


use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Emulator { 
    state: Processor8085,  
    offset: usize 
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(offset: usize) -> Emulator {
        Emulator {
            state: Processor8085::new(),  
            offset
        }
    }

    pub fn b(&self) -> u8 {
        self.state.b
    }

    pub fn c(&self) -> u8 {
        self.state.c
    }

    pub fn d(&self) -> u8 {
        self.state.d
    }

    pub fn e(&self) -> u8 {
        self.state.e
    }

    pub fn h(&self) -> u8 {
        self.state.h
    }

    pub fn l(&self) -> u8 {
        self.state.l
    }

    pub fn flags(&self) -> Box<[u8]>  {
        vec![self.state.flag.zero as u8, self.state.flag.sign as u8, self.state.flag.parity as u8, self.state.flag.carry as u8 ,self.state.flag.auxillary_carry as u8].into_boxed_slice()
    }

    pub fn accumulator(&self) -> u8 {
        self.state.accumulator
    }

    pub fn stack_pointer(&self) -> u16 {
        self.state.stack_pointer
    }

    pub fn program_counter(&self) -> u16 {
        self.state.program_counter
    }

    pub fn set_program_counter(&mut self, offset:u16) {
        self.state.program_counter = offset;
    }

//    pub fn assemble(&self,data: &str) -> Box<[u8]> {
       // generate_assembly_code(vec![String::from(data)]);
       // my_vec.into_boxed_slice()
 //   }

    pub fn get_assembly(&mut self, data: Box<[JsValue]>) -> Result<Box<[u8]>, JsValue> {
       let formatted_data = data.iter().map(|s| { s.as_string().unwrap().to_lowercase() }).collect();
       match generate_assembly_code(formatted_data) {
           Ok(data) => {
               let (assembled_code,meta) = data;
               return Ok(assembled_code.into_boxed_slice());
           }
           Err(error_signal) => {
            let error = JsValue::from(format!("{error}", error=error_signal.1));
            return Err(error);
           }
       }
    }

    pub fn load_program(&mut self,offset:usize, data: Box<[JsValue]>) -> Result<Box<[usize]>, JsValue> {
       let start_address = self.offset;
       let formatted_data = data.iter().map(|s| { s.as_string().unwrap().to_lowercase() }).collect();

       match generate_assembly_code(formatted_data) {
           Ok(data) =>{
               let (assembled_code,meta) = data;
               let mut position = start_address+offset;
               for (counter,&hex_code) in assembled_code.iter().enumerate() {
                   self.state.memory[start_address + offset + counter]  = hex_code;
                   position = position + counter;
               } 
               return Ok(meta.into_boxed_slice());
           } 
           Err(error_signal) => {
           self.state.memory[0] = 0;
           let error = JsValue::from(format!("{linenumber}; {error}", linenumber=error_signal.0, error=error_signal.1));
           return Err(error);
           }
       }
    }

    pub fn watch_memory(&self, start: usize, stop : usize ) -> Box<[u8]> {
        self.state.memory[start..stop].into()
    }

    pub fn emulate_line_by_line(&mut self) -> u16{
        if let Some(program_counter) = emulate_8085(&mut self.state, self.offset) {
            return program_counter;
        }
        return 0;
    }

    pub fn emulate(&mut self) -> u16{
        while let Some(program_counter) = emulate_8085(&mut self.state, self.offset) {
        }
        return 0;
    }

}




