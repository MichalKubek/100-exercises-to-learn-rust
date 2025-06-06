// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
//

use std::{ops::Add, u8};

#[derive(Debug, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}


impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 { value: value.into() }
    }
}

impl From<u16> for SaturatingU16  {
   fn from(value: u16) -> Self {
        SaturatingU16 { value }
    } 
}


impl<T> From<&T> for SaturatingU16
where
    SaturatingU16: From<T>,
    T: Copy,
{
    fn from(value: &T) -> Self {
        SaturatingU16::from(*value)
    }
}
//impl From<&T> for SaturatingU16
//    where
//        SaturatingU16: From<T>,
//        T: Copy,
//{
//   fn from(value: &T) -> Self {
//        SaturatingU16::from(*value)
//    } 
//}

//impl From<&SaturatingU16> for SaturatingU16  {
//    fn from(value: &SaturatingU16) -> Self {
//        SaturatingU16 { value: (*value).value }
//    }
//}

impl Add for SaturatingU16  {
    type Output = Self;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        let value = match self.value.checked_add(rhs.value) {
                Some(value) => value,
                None => u16::MAX,
            };
        SaturatingU16 {
            value
        }
    }
}

impl Add<u16> for SaturatingU16  {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        let value = match self.value.checked_add(rhs) {
                Some(value) => value,
                None => u16::MAX,
            };
        SaturatingU16 {
            value
        }
    }
}
impl Add<&SaturatingU16> for SaturatingU16  {
    type Output = Self;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        let value = match self.value.checked_add(rhs.value) {
                Some(value) => value,
                None => u16::MAX,
            };
        SaturatingU16 {
            value
        }
    }
}



impl PartialEq<SaturatingU16> for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
    
}

