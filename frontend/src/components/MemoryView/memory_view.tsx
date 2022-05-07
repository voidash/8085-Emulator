import { useState, useEffect } from 'react';
import './style.css';

import * as wasm from '../../wasm/wasm_8085';
import { Box, List, TextField } from '@mui/material';


export default function MemoryView({ emulator, loaded }: { emulator: wasm.Emulator, loaded: boolean }) {
  let [startAddr, setStartAddr] = useState("0");
  let [stopAddr, setStopAddr] = useState("20");
  let [data, setData] = useState<number[]>([]);

  function strToNumber(data: string): number {
    if (data.startsWith("0x")) {
      return parseInt(data.substring(2), 16);
    } else if (data.startsWith("0o")) {
      return parseInt(data.substring(2), 8);
    } else if (data.startsWith("0b")) {
      return parseInt(data.substring(2), 2);
    }
    return parseInt(data);
  }

  useEffect(() => {
    console.log(loaded);
    if (strToNumber(startAddr) < strToNumber(stopAddr)) {
      console.log(startAddr)
      console.log(stopAddr)
      let d = emulator?.watch_memory(strToNumber(startAddr), strToNumber(stopAddr));
      let arrCommon = Array.prototype.slice.call(d);
      setData(arrCommon);
    }
  }, [loaded, startAddr, stopAddr]);
  return (
    <Box sx={{
      margin: "auto"
    }}>
      <Box display={'flex'} alignItems={'center'} justifyContent={'space-between'} py={2}>
        <TextField error={strToNumber(startAddr) >= strToNumber(stopAddr)} helperText={strToNumber(startAddr) >= strToNumber(stopAddr) ? "Start can't be greater than stop." : ""} label="Start" variant="filled" inputProps={{ inputMode: 'numeric', }} onChange={(e) => {
          if (e.target.value.trim() == "") {
            setStartAddr('0');
          } else {

            setStartAddr(e.target.value);
          }
        }} />
        <TextField error={strToNumber(startAddr) >= strToNumber(stopAddr)} helperText={strToNumber(startAddr) >= strToNumber(stopAddr) ? "Stop can't be smaller than start." : ""} label="Stop" variant="filled" inputProps={{ inputMode: 'numeric', }} onChange={(e) => {
          if (e.target.value.trim() == "") {
            setStopAddr('300');
          } else {

            setStopAddr(e.target.value);
          }
        }} />
      </Box>

      <List className='memoryView'>
        {data.map((s, i) => (
          <Box key={i} sx={{
            margin: 0.3,
            padding: 0,
            width: 80,
            borderRadius: 0.5,
            backgroundColor: 'primary.dark',
            '&:hover': {
              backgroundColor: 'primary.main',
              opacity: [0.9, 0.8, 0.7],
            },
          }}>
            <Box sx={{
              textAlign: 'center',
              fontSize: 14,
              padding: 1,
              backgroundColor: 'primary.light',

            }}>
              {"0x" + (i + strToNumber(startAddr)).toString(16)}
            </Box>
            <Box sx={{
              textAlign: 'center',
              fontSize: 18,
              color: "white",

              padding: 2,
            }} >
              {s == 0 ? "‎" : "0x" + s.toString(16)}
            </Box>
          </Box>
        ))}
      </List>
    </Box >
  );

}


