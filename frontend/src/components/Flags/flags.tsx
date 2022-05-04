import { Label } from '@mui/icons-material';
import { Box, Checkbox, FormControlLabel, FormGroup } from '@mui/material';
import { useState, useEffect } from 'react';


import * as wasm from '../../wasm/wasm_8085';
import './style.css'


export default function Flags({ emulator }: { emulator: wasm.Emulator }) {
  return (
    <Box display={'flex'} flexDirection={'row'} flexWrap={'wrap'} justifyContent={'start'} alignItems={'center'} width="100%">
      <h3 >Flag Registers Status:  </h3>
      <Box width="20px"></Box>
      <FormControlLabel control={<Checkbox checked={false} />} label="Zero" />
      <FormControlLabel control={<Checkbox checked={false} />} label="Sign" />
      <FormControlLabel control={<Checkbox checked={false} />} label="Parity" />
      <FormControlLabel control={<Checkbox checked={false} />} label="Carry" />
      <FormControlLabel control={<Checkbox checked={false} />} label="Aux Carry" />
    </Box>
  )
}
