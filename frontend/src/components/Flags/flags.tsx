import { Label } from '@mui/icons-material';
import { Box, Checkbox, FormControlLabel, FormGroup, useMediaQuery } from '@mui/material';
import { useState, useEffect } from 'react';


import * as wasm from '../../wasm/wasm_8085';
import './style.css'

function u8ToBool(val: number) {
  return (val == 1)? true: false;
}

export default function Flags({ emulator }: { emulator: wasm.Emulator }) {
  const matches = useMediaQuery('(min-width:600px)');
  return (
    <Box display={'flex'} flexDirection={'row'} flexWrap={'wrap'} justifyContent={'start'} alignItems={'center'} width="100%">
      <h3 className='mobileView' >Flag Registers Status:  </h3>
      <Box width="20px"></Box>
      <FormControlLabel control={<Checkbox checked={u8ToBool(emulator.flags()[0])} size={matches ? "small" : "medium"} />} label="Zero" />
      <FormControlLabel control={<Checkbox checked={u8ToBool(emulator.flags()[1])} size={matches ? "small" : "medium"} />} label="Sign" />
      <FormControlLabel control={<Checkbox checked={u8ToBool(emulator.flags()[2])} size={matches ? "small" : "medium"} />} label="Parity" />
      <FormControlLabel control={<Checkbox checked={u8ToBool(emulator.flags()[3])} size={matches ? "small" : "medium"} />} label="Carry" />
      <FormControlLabel control={<Checkbox checked={u8ToBool(emulator.flags()[4])} size={matches ? "small" : "medium"} />} label="Aux Carry" />
    </Box>
  )
}
