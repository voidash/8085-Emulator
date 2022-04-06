import './register_style.css'

import {useState, useEffect} from 'react';

interface register{
  name: string,
  value: string,
  shouldHighlight?: boolean,
  sixteenBit?: boolean
}

let Register = ({name, value, shouldHighlight, sixteenBit} : register) => {
  return (<div className={ sixteenBit? "Register16bit Register": "Register"} >
            <div>{name}</div>  
            <div>{value}</div>  
          </div>);
};


export default function Registers() { 
  return (
    <div>


      <Register name="Accumulator" value ="0x00" sixteenBit={true}/>
      <div className="RegisterPair">
          <Register name = "B" value = "0x00"/>
          <Register name = "C" value = "0x00"/>
        </div>

      <div className="RegisterPair">
          <Register name = "D" value = "0x00"/>
          <Register name = "E" value = "0x00"/>
        </div>

      <div className="RegisterPair">
          <Register name = "H" value = "0x00"/>
          <Register name = "L" value = "0x00"/>
        </div>

      <Register name="Stack Pointer" value="0x00" sixteenBit={true}/>
      <Register name="Program Counter" value="0x00" sixteenBit={true}/>

        </div>

  );
}
