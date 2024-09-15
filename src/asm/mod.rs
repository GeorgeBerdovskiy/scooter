pub mod lower;
pub mod register;

/// Contains lowering logic for all available targets.
pub mod targets {
    /// Contains code for lowering IR to RISC-V.
    pub mod risc_v;
}
