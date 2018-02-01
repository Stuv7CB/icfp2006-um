extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use operator::{Operator, Data, Ort};
use std::ops::{Add, Mul, Div, Not, BitAnd};
use std::cmp::{PartialEq};

#[derive(Debug, Copy, Clone)]
pub struct Plate {
    plate: u32
}

impl Plate {
    pub fn new(buf: [u8;4]) -> Plate {
        let mut reader = Cursor::new(buf);
        Plate {
            plate: reader.read_u32::<BigEndian>().expect("Cannot parse to u32")
        }
    }

    pub fn from(value: u32) -> Plate {
        Plate {
            plate: value
        }
    }

    pub fn get_operator_data(&self) -> Operator {
        let operator = (self.plate & 0xF000_0000) >> 28;
        let data = Data {
                    a: (self.plate & 0b1_1100_0000) >> 6,
                    b: (self.plate & 0b11_1000) >> 3,
                    c: self.plate & 0b111,
                    };
        match operator {
            0 => Operator::ConditionalMove(data),
            1 => Operator::ArrayIndex(data),
            2 => Operator::ArrayAmendment(data),
            3 => Operator::Addition(data),
            4 => Operator::Multiplication(data),
            5 => Operator::Division(data),
            6 => Operator::NotAnd(data),
            7 => Operator::Halt,
            8 => Operator::Allocation(data),
            9 => Operator::Abandonment(data),
            10 => Operator::Output(data),
            11 => Operator::Input(data),
            12 => Operator::LoadProgram(data),
            13 => Operator::Orthography(Ort {
                    a: (self.plate & 0xE00_0000) >> 25,
                    data: self.plate & 0x1FF_FFFF
                }),
            _ => panic!("Wrong instruction!")
        }
    }

    pub fn unwrap(&self) -> u32 {
        self.plate
    }
}

use std::num::Wrapping;

impl<'a, 'b> Add<&'b Plate> for &'a Plate {
    type Output = Plate;

    fn add(self, other: &'b Plate) -> Plate {
        Plate::from((Wrapping(self.plate) + Wrapping(other.plate)).0)
    }
}

impl<'a, 'b> Mul<&'b Plate> for &'a Plate {
    type Output = Plate;

    fn mul(self, other: &'b Plate) -> Plate {
        Plate::from((Wrapping(self.plate) * Wrapping(other.plate)).0)
    }
}

impl<'a, 'b> Div<&'b Plate> for &'a Plate {
    type Output = Plate;

    fn div(self, other: &'b Plate) -> Plate {
        Plate::from((Wrapping(self.plate) / Wrapping(other.plate)).0)
    }
}

impl<'a> Not for &'a Plate {
    type Output = Plate;

    fn not(self) -> Plate {
        Plate::from(!self.plate)
    }
}

impl<'a, 'b> BitAnd<&'b Plate> for &'a Plate {
    type Output = Plate;

    fn bitand(self, other: &'b Plate) -> Plate {
        Plate::from(self.plate & other.plate)
    }
}

impl PartialEq<u32> for Plate {
    fn eq(&self, other: &u32) -> bool {
        self.plate == *other
    }
}

#[cfg(test)]
mod tests {
    use super::Plate;

    #[test]
    fn plate_not() {
        let plate_a = Plate::from(0xFFFFFFFF);
        assert_eq!((!&plate_a).unwrap(), 0);
    }

    #[test]
    fn plate_and() {
        let plate_a = Plate::from(0x0F0F0F0F);
        let plate_b = Plate::from(0xF0F0F0F0);
        assert_eq!((&plate_a & &plate_b).unwrap(), 0);
    }
}