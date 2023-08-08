use ark_ff::Field;

pub mod r1cs;
pub mod relaxed_r1cs;
pub mod plonkish;
pub mod ccs;

pub struct Instance<F: Field> {
    pub x: Vec<F>,
}

pub struct Witness<F: Field> {
    pub w: Vec<F>
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: add tests for correctness of constraint systems.
}