import {useState} from 'react';
import './style.css';

interface memory {
  position: string,
  value: string
};

let Memory = ({position, value}: memory) => {
  return(
    <div className = "MemoryBox">
      <div className= "position">{position}</div>
      <div className="value">{value}</div>
    </div>
  )
}
export default function MemoryView() {
  return(
    <div className="mem">
    <div className="MemorySearch">
      <input type="text" className="searchBar"/>
      <button className="searchButton">Search</button>
    </div>
      <div className="MemoryView">
    <Memory position="0x22ff" value="0x33"/>
    <Memory position="0x22ff" value="0x33"/>
 </div>
</div>);

}
