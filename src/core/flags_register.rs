/* F register is called the Flag register.
 * The lower four bits of the register ar always 0s
 * The CPU  writes automatically on the upper 4-bits:
 *
 * 7: "Z"  -> ZERO
 * 6: "S"  -> SUBSTRACTION
 * 5: "HC" -> HALF CARRY
 * 4: "C"  -> CARRY
 *
 *    ┌-> Carry
 * ┌-+> Subtraction
 * | |
 * 1111 0000
 * | |
 * └-+> Zero
 *   └-> Half Carry
 * 
 */

use std;

const ZERO_FLAG_BYTE_POS: u8 = 7;
const SUB_FLAG_BYTE_POS: u8 = 6
const HC_FLAG_BYTE_POS: u8 = 5
const CARRY_FLAG_BYTE_POS: u8 = 4

pub struct FlagsRegister
{
    // Set if the last operation results in a 0
    pub zero: bool,
    // Set if the last operation was a substraction
    pub substract: bool,
    // Set if lower half of the result overflowed
    pub half_carry: bool,
    // Set if the result overflowed
    pub carry: bool,
}

impl FlagsRegister
{
    pub fn new() -> FlagsRegister
    {
        FlagsRegister
        {
            zero: false,
            substract: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 
{
    fn from(flag: FlagsRegister) -> u8
    {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POS       |
        (if flag.substract  { 1 } else { 0 }) << SUB_FLAG_BYTE_POS        |
        (if flag.half_carry { 1 } else { 0 }) << HC_FLAG_BYTE_POS |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POS      |
    }
}

impl std::convert::From<u8> for FlagsRegister 
{
    fn from(byte: u8) -> Self 
    {
        let zero        =((byte >> ZERO_FLAG_BYTE_POS)  & 0b1) != 0;
        let substract   =((byte >> SUB_FLAG_BYTE_POS)   & 0b1) != 0;
        let half_carry  =((byte >> HC_FLAG_BYTE_POS)    & 0b1) != 0;
        let carry       =((byte >> CARRY_FLAG_BYTE_POS) & 0b1) != 0;

        FlagsRegister
        {
            zero,
            substract,
            half_carry,
            carry,
        }
    }
}