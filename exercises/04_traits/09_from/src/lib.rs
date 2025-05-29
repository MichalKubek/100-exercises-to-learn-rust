// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[allow(dead_code)]
pub struct WrappingU32 {
    value: u32,
}


impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

impl Into<u32> for WrappingU32 {
    fn into(self) -> u32 {
        self.value
    }
}



#[allow(dead_code)]
fn example() {
    let _wrapping: WrappingU32 = 42.into();
    let _wrapping = WrappingU32::from(42);
    let a: u32 = _wrapping.into();
}
