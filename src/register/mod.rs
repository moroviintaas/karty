mod register;
pub use register::Register;

mod hashset_register;
pub use hashset_register::HashSetCardRegister;

mod standard_register;
pub use standard_register::RegisterCardStd;
#[cfg(feature = "register")]
mod sampling;

//#[cfg(feature = "register")]
//pub use crate::random::random_sampling_register;
