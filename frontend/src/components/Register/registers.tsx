import './register_style.css'

import { useState, useEffect } from 'react';
import * as wasm from '../../wasm/wasm_8085';
import { Box } from '@mui/material';
interface register {
  name: string,
  value: string,
  shouldHighlight?: boolean,
  sixteenBit?: boolean
}

let Register = ({ name, value, shouldHighlight, sixteenBit }: register) => {
  return (
    <div className={sixteenBit ? "Register16bit Register" : "Register"} >
      <div>{name}</div>
      <div>{value}</div>
    </div>);
};


export default function Registers({ emulator }: { emulator: wasm.Emulator }) {
  return (
    <Box>
      <Register name="Accumulator" value={"0x" + emulator.accumulator().toString(16)} sixteenBit={true} />
      <div className="RegisterPair">
        <Register name="B" value={"0x" + emulator.b().toString(16)} />
        <Register name="C" value={"0x" + emulator.c().toString(16)} />
      </div>
      <div className="RegisterPair">
        <Register name="D" value={"0x" + emulator.d().toString(16)} />
        <Register name="E" value={"0x" + emulator.e().toString(16)} />
      </div>
      <div className="RegisterPair">
        <Register name="H" value={"0x" + emulator.h().toString(16)} />
        <Register name="L" value={"0x" + emulator.l().toString(16)} />
      </div>
      <Register name="Stack Pointer" value={"0x" + emulator.stack_pointer().toString(16)} sixteenBit={true} />
      <Register name="Program Counter" value={"0x" + emulator.program_counter().toString(16)} sixteenBit={true} />

    </Box>

  );
}
