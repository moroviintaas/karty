mod register_trait;
pub use register_trait::Register;

mod generic_register;
pub use generic_register::GenericRegister;

mod standard_register;
pub use standard_register::RegisterCardStd;

#[cfg(feature = "random")]
mod sampling;

#[cfg(feature = "random")]
pub use sampling::{RandomSamplingRegister, RandomSamplingRegisterCompl};

#[cfg(feature = "random")]
mod generic_sampling_register;

#[cfg(feature = "random")]
pub use generic_sampling_register::{GenericSamplingRegister, GenericSamplingRegisterCompl};

//#[cfg(feature = "register")]
//pub use crate::random::random_sampling_register;
