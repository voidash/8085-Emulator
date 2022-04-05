let getEquivalentHex = (value: number) => {
  return value.toString(16);
} 

let getEquivalentOctal = (value: number) => {
  return value.toString(8);
} 

let getEquivalentBinary = (value: number) => {
  return value.toString(2);
} 

export {getEquivalentHex, getEquivalentOctal, getEquivalentBinary}
