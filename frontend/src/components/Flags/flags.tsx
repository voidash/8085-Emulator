import {useState, useEffect} from 'react';
import './style.css'

interface flag {
  name: string, 
  state: boolean
}

let FlagCheckbox = ({name, state}: flag) => {
  return (
    <div className="flagcheckboxes">
      <div className="flagcheckbox_name">{name}</div>
      <input type="checkbox" name="cbx" checked={state} className="flagcheckbox_checkbox"/>
    </div>
  )
}
export default function Flags({}) {
  return (<div className="flags">
            <FlagCheckbox name ="Zero" state={false}/>
            <FlagCheckbox name ="Sign" state={false}/>
            <FlagCheckbox name ="Parity" state={false}/>
            <FlagCheckbox name ="Carry" state={false}/>
            <FlagCheckbox name ="Aux Carry" state={false}/>
          </div>)
}

