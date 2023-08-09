use ark_ff::Field;

pub fn dot_prod<F: Field>(u: &Vec<F>, v: &Vec<F>) -> F {
    let mut res = F::zero();
    for i in 0..u.len() {
        res += u[i] * v[i];
    }
    res
}

pub fn add_vec<F: Field>(u: &Vec<F>, v: &Vec<F>) -> Vec<F> {
    let mut sum = Vec::new();
    for i in 0..u.len() {
        sum.push(u[i] + v[i]);
    }
    sum
}

pub fn scale_vec<F: Field>(u: &Vec<F>, k: &F) -> Vec<F> {
    let mut scaled = Vec::new();
    for i in 0..u.len() {
        scaled.push(*k * u[i]);
    }
    scaled
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::UniformRand;
    use ark_bls12_381::Fq;
    use rand::rngs::OsRng;

    #[test]
    fn test_dot_product() {
        let u = vec![Fq::from(1); 20];
        let v = vec![
            Fq::from(1),
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
            Fq::from(5),
            Fq::from(6),
            Fq::from(7),
            Fq::from(8),
            Fq::from(9),
            Fq::from(10),
            Fq::from(11),
            Fq::from(12),
            Fq::from(13),
            Fq::from(14),
            Fq::from(15),
            Fq::from(16),
            Fq::from(17),
            Fq::from(18),
            Fq::from(19),
            Fq::from(20)
        ];

        let prod = dot_prod(&u, &v);

        assert_eq!(
            prod,
            Fq::from(210)
        );
    }

    #[test]
    fn test_mat_vec_prod() {
        let mat_data = vec![
            vec![Fq::from(1), Fq::from(0), Fq::from(0)],
            vec![Fq::from(0), Fq::from(1), Fq::from(0)],
            vec![Fq::from(0), Fq::from(0), Fq::from(1)]
        ];
        let matrix = Matrix::from(mat_data);

        let v = vec![Fq::rand(&mut OsRng {}), Fq::rand(&mut OsRng {}), Fq::rand(&mut OsRng {})];

        assert_eq!(v, matrix.mat_vec_prod(&v));
    }
}