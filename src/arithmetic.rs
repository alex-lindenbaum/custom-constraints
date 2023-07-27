use ark_ff::Field;

#[derive(Clone)]
pub struct Matrix<F: Field> {
    pub data: Vec<Vec<F>>,
}

impl<F: Field> Matrix<F> {
    pub fn from(data: Vec<Vec<F>>) -> Self {
        Self { data }
    }

    pub fn zero(m: usize, n: usize) -> Self {
        Self { data: vec![vec![F::zero(); n]; m] }
    }

    pub fn mat_vec_prod(&self, v: &Vec<F>) -> Vec<F> {
        let mut result = Vec::new();

        for r in 0..self.data.len() {
            let mut entry = F::zero();
            for i in 0..v.len() {
                entry += self.data[r][i] * v[i];
            }

            result.push(entry);
        }

        result
    }
}

pub fn dot_prod<F: Field>(u: &Vec<F>, v: &Vec<F>) -> F {
    let mut res = F::zero();

    for i in 0..u.len() {
        res += u[i] * v[i];
    }

    res
}