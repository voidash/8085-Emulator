import { Label } from '@mui/icons-material';
import { Box, Checkbox, FormControlLabel, FormGroup, useMediaQuery } from '@mui/material';
import { useState, useEffect } from 'react';


import * as wasm from '../../wasm/wasm_8085';
import './style.css'


export default function Flags({ emulator }: { emulator: wasm.Emulator }) {
  const matches = useMediaQuery('(min-width:600px)');
  return (
    <Box display={'flex'} flexDirection={'row'} flexWrap={'wrap'} justifyContent={'start'} alignItems={'center'} width="100%">
      <h3 className='mobileView' >Flag Registers Status:  </h3>
      <Box width="20px"></Box>
      <FormControlLabel control={<Checkbox checked={false} size={matches ? "small" : "medium"} />} label="Zero" />
      <FormControlLabel control={<Checkbox checked={false} size={matches ? "small" : "medium"} />} label="Sign" />
      <FormControlLabel control={<Checkbox checked={false} size={matches ? "small" : "medium"} />} label="Parity" />
      <FormControlLabel control={<Checkbox checked={false} size={matches ? "small" : "medium"} />} label="Carry" />
      <FormControlLabel control={<Checkbox checked={false} size={matches ? "small" : "medium"} />} label="Aux Carry" />
    </Box>
  )
}
