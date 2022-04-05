import './register_style.css'

import {useState, useEffect} from 'react';

interface Register{
  name: string,
  value: number,
  shouldHighlight: boolean
}

let Register = ({name, value, shouldHighlight} : Register) => {
  return (<div className="Register">
            <div>{name}</div>  
            <div>{value}</div>  
          </div>);
};


export default function Registers() { 
  return (
    <div>
      <div className="RegisterPair">
          <Register name = "B" value = {22} shouldHighlight={false}/>
          <Register name = "C" value = {34} shouldHighlight={false}/>
        </div>

      <div className="RegisterPair">
          <Register name = "D" value = {22} shouldHighlight={false}/>
          <Register name = "E" value = {34} shouldHighlight={false}/>
        </div>

      <div className="RegisterPair">
          <Register name = "H" value = {22} shouldHighlight={false}/>
          <Register name = "L" value = {34} shouldHighlight={false}/>
        </div>
        </div>


  );
}
