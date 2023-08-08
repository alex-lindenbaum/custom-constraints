use ark_ff::Field;
use super::{Instance, Witness};

use crate::arithmetic::{Matrix, dot_prod};

pub fn construct_z<F: Field>(
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
    pub struct RelaxedR1CS<F: Field> {
        pub A: Matrix<F>,
        pub B: Matrix<F>,
        pub C: Matrix<F>,
        error: Vec<F>,
        u: F,
    }

    impl<F: Field> RelaxedR1CS<F> {
        pub fn is_satisfied(
            &self,
            instance: &Instance<F>,
            witness: &Witness<F>
        ) -> bool {
            let z = construct_z(instance, witness, &self.u);
            for m in 0..self.A.data.len() {
                let eval = dot_prod(&self.A.data[m], &z)
                    * dot_prod(&self.B.data[m], &z)
                    - self.u * dot_prod(&self.C.data[m], &z)
                    - self.error[m];
                
                if !eval.is_zero() { return false; }
            }

            true
        }
    }