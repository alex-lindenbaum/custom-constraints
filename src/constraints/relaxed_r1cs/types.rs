use ark_ff::Field;
use ark_ec::CurveGroup;

use crate::constraints::{Instance, Witness};

pub struct FoldedR1CSInstance<G: CurveGroup> {
    pub x: Instance<G::ScalarField>,
    pub com_error: G,
    pub com_witness: G,
    pub u: G::ScalarField,
}

pub struct FoldedR1CSWitness<F: Field> {
    pub w: Witness<F>,
    pub error: Vec<F>,
    pub r_error: F,
    pub w_error: F,
}