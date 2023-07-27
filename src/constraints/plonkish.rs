use ark_ff::Field;
use ark_poly::{multivariate::{SparsePolynomial, SparseTerm}, Polynomial};
use super::{Instance, Witness};

pub fn construct_z<F: Field>(instance: &Instance<F>, witness: &Witness<F>, selectors: &Vec<F>) -> Vec<F> {
    let mut z = Vec::new();
    
    witness.w.iter().for_each(|&w| z.push(w));
    instance.x.iter().for_each(|&x| z.push(x));
    selectors.iter().for_each(|&s| z.push(s));

    z
}

pub struct Plonkish<F: Field> {
    pub num_variables: usize,
    pub input_size: usize,  // |x| + |w|
    pub g: SparsePolynomial<F, SparseTerm>,
    pub selectors: Vec<F>,
    pub constraints: Vec<Vec<usize>>
}

impl<F: Field> Plonkish<F> {
    pub fn is_satisfied(
        &self,
        instance: &Instance<F>,
        witness: &Witness<F>
    ) -> bool {
        let z = construct_z(instance, witness, &self.selectors);
        for i in 0..self.constraints.len() {
            let mut point = Vec::new();
            for j in 0..self.num_variables {
                point.push(z[self.constraints[i][j]]);
            }

            let eval = self.g.evaluate(&point);
            if !eval.is_zero() { return false }
        }

        true
    }
}