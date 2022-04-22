import { useState, useEffect } from 'react';
import './style.css';

import * as wasm from '../../wasm/wasm_8085';

interface memory {
  position: string,
  value: string
};

let Memory = ({ position, value }: memory) => {
  return (
    <div className="MemoryBox">
      <div className="position">{position}</div>
      <div className="value">{value}</div>
    </div>
  )
}


export default function MemoryView({ emulator, loaded }: { emulator: wasm.Emulator, loaded: boolean }) {
  let [startAddr, setStartAddr] = useState("0");
  let [stopAddr, setStopAddr] = useState("300");
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
    console.log(emulator);
    if (strToNumber(startAddr) < strToNumber(stopAddr)) {
      console.log(startAddr)
      console.log(stopAddr)
      let d = emulator?.watch_memory(strToNumber(startAddr), strToNumber(stopAddr));
      let arrCommon = Array.prototype.slice.call(d);
      setData(arrCommon);
    }
  }, [loaded, startAddr, stopAddr]);

  //TODO: check if start is greater then stop
  //TODO: show error if that happens
  return (
    <div className="mem">
      <h3>{loaded ? "program is loaded" : "program not loaded"}</h3>

      <div className="MemorySearch">
        <span>start  </span>
        <input type="text" className="searchBar" onChange={(e) => setStartAddr(e.target.value)} />
        <span>stop </span>
        <input type="text" className="searchBar" onChange={(e) => setStopAddr(e.target.value)} />
        <button className="searchButton">Set Memory space</button>
      </div>

      <div className="MemoryView">
        {data.map((s, i) => (
          <Memory key={i} position={"0x" + (i + strToNumber(startAddr)).toString(16)} value={"0x" + s.toString(16)} />
        ))}
      </div>
    </div>);

}


