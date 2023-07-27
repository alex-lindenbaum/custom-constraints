use ark_ff::Field;
use super::{Instance, Witness};

use crate::arithmetic::{Matrix, dot_prod};

/// Exists so we save some time instead of cloning x, w, then copying into z.
pub fn construct_z<F: Field>(instance: &Instance<F>, witness: &Witness<F>) -> Vec<F> {
    let mut z = Vec::new();
    
    witness.w.iter().for_each(|&w| z.push(w));
    z.push(F::one());
    instance.x.iter().for_each(|&x| z.push(x));

    z
}

#[allow(non_snake_case)]
pub struct R1CS<F: Field> {
    pub A: Matrix<F>,
    pub B: Matrix<F>,
    pub C: Matrix<F>,
}

impl<F: Field> R1CS<F> {
    pub fn is_satisfied(
        &self,
        instance: &Instance<F>,
        witness: &Witness<F>
    ) -> bool {
        for m in 0..self.A.data.len() {
            let z = construct_z(instance, witness);
            let eval = dot_prod(&self.A.data[m], &z)
                * dot_prod(&self.B.data[m], &z)
                - dot_prod(&self.C.data[m], &z);

            if !eval.is_zero() { return false }
        }

        true
    }
}