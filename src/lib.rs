use crate::module_a::EnumA;
pub use crate::module_b::module_c::one; // re-exporting nested fn

pub mod module_a {
    pub enum EnumA {
        Void,
        Unit,
    }
}

mod module_b {
    pub mod module_c {
        pub fn one() -> u32 {
            1
        }
    }
}

pub fn inc(a: u32) -> u32 {
    a + 1
}

pub fn create_void() -> EnumA {
    EnumA::Void
}

pub fn two() -> u32 {
    // Option 1
    // crate::module_b::module_c::one() + 1

    // Option 2
    one() + 1
}
