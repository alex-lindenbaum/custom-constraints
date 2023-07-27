use ark_ff::Field;
use ark_poly::polynomial::multivariate::Term;
use super::{Instance, Witness, r1cs::R1CS, plonkish::Plonkish};

use crate::arithmetic::{Matrix, dot_prod};

/// Exists so we save some time instead of cloning x, w, then copying into z.
pub fn construct_z<F: Field>(instance: &Instance<F>, witness: &Witness<F>) -> Vec<F> {
    let mut z = Vec::new();
    
    witness.w.iter().for_each(|&w| z.push(w));
    z.push(F::one());
    instance.x.iter().for_each(|&x| z.push(x));

    z
}

pub struct CCS<F: Field> {
    pub matrices: Vec<Matrix<F>>,
    pub selectors: Vec<Vec<usize>>,
    pub c: Vec<F>,
}

impl<F: Field> CCS<F> {
    pub fn from_r1cs(r: R1CS<F>) -> Self {
        Self {
            matrices: vec![r.A, r.B, r.C],
            selectors: vec![
                vec![0, 1], vec![2]
            ],
            c: vec![F::one(), F::zero() - F::one()]
        }
    }

    pub fn from_plonkish(p: Plonkish<F>) -> Self {
        let mut matrices: Vec<Matrix<F>> = vec![Matrix::zero(p.constraints.len(), p.input_size); p.num_variables];

        for i in 0..p.constraints.len() {
            for j in 0..p.num_variables {
                let k = p.constraints[i][j];
                if k >= p.num_variables {
                    matrices[j].data[i][0] = p.selectors[k - p.num_variables];
                } else {
                    matrices[j].data[i][k] = F::one();
                }
            }
        }
        
        let terms = p.g.terms;
        let mut c = Vec::new();
        let mut selectors = Vec::new();

        terms.iter().for_each(|(coeff, monomial)| {
            c.push(*coeff);
            selectors.push(monomial.vars())
        });

        Self {
            matrices,
            selectors,
            c
        }
    }

    pub fn is_satisfied(
        &self,
        instance: &Instance<F>,
        witness: &Witness<F>
    ) -> bool {
        let z = construct_z(instance, witness);
        for m in 0..self.matrices[0].data.len() {
            let mut sum = F::zero();
            for i in 0..self.selectors.len() {
                let mut hadamard_prod = F::one();
                for j in 0..self.selectors[i].len() {
                    let index = self.selectors[i][j];
                    hadamard_prod *= dot_prod(&self.matrices[index].data[m], &z);
                }
                sum += self.c[i] * hadamard_prod;

                if !sum.is_zero() { return false }
            }
        }

        true
    }
}