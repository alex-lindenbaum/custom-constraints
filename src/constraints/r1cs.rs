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

pub fn construct_z_relaxed<F: Field>(
    instance: &Instance<F>,
    witness: &Witness<F>,
    u: &F
) -> Vec<F> {
    let mut z = Vec::new();

    witness.w.iter().for_each(|&w| z.push(w));
    z.push(*u);
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
        let z = construct_z(instance, witness);
        for m in 0..self.A.data.len() {
            let eval = dot_prod(&self.A.data[m], &z)
                * dot_prod(&self.B.data[m], &z)
                - dot_prod(&self.C.data[m], &z);

            if !eval.is_zero() { return false; }
        }

        true
    }

    pub fn is_relaxed_satisfied(
        &self,
        instance: &Instance<F>,
        witness: &Witness<F>,
        error: &Vec<F>,
        u: &F
    ) -> bool {
        let z = construct_z_relaxed(instance, witness, &u);
        for m in 0..self.A.data.len() {
            let eval = dot_prod(&self.A.data[m], &z)
                * dot_prod(&self.B.data[m], &z)
                - *u * dot_prod(&self.C.data[m], &z)
                - error[m];
                
            if !eval.is_zero() { return false; }
        }

        true
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;
    use ark_ff::{One, Zero};

    #[test]
    fn test_is_satisfied_valid() {
        let instance = Instance {
            x: vec![Fr::zero() - Fr::one()],
        };
        let witness = Witness {
            w: vec![],
        };

        let A = Matrix::from(vec![vec![Fr::one(), Fr::zero()], vec![Fr::zero(), Fr::one()]]);
        let B = Matrix::from(vec![vec![Fr::one(), Fr::one()], vec![Fr::zero(), Fr::one()]]);
        let C = Matrix::from(vec![vec![Fr::zero(), Fr::zero()], vec![Fr::one(), Fr::zero()]]);

        let r1cs = R1CS { A, B, C };

        assert!(r1cs.is_satisfied(&instance, &witness));
    }

    #[test]
    fn test_is_satisfied_invalid() {
        let instance = Instance {
            x: vec![Fr::one()],
        };
        let witness = Witness {
            w: vec![],
        };

        let A = Matrix::from(vec![vec![Fr::one(), Fr::zero()], vec![Fr::zero(), Fr::one()]]);
        let B = Matrix::from(vec![vec![Fr::one(), Fr::one()], vec![Fr::zero(), Fr::one()]]);
        let C = Matrix::from(vec![vec![Fr::zero(), Fr::zero()], vec![Fr::one(), Fr::zero()]]);

        let r1cs = R1CS { A, B, C };

        assert!(!r1cs.is_satisfied(&instance, &witness));
    }
}