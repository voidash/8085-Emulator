pub fn check_parity(data: u8 , size: u8) -> bool{
let mut ones = 0;
for i in 0..size {
    if (data >> i) & 0x1 == 1{
        ones += 1;
    }
}
    ones % 2 == 0
}

pub fn bool_to_bin(data:bool) -> u8{
    if data == true {1} else {0}
}

pub fn bin_to_bool(data:u8) -> bool{
    if data == 1 {true} else {false}
}



#[test]
fn parity_of_10() {
    //0b1010
    assert_eq!(check_parity(10,8),true);
}

#[test]
fn parity_of_59() {
    //0b111011
    assert_eq!(check_parity(59,8),false);
}



