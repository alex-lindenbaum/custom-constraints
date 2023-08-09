use ark_ec::CurveGroup;
use rand::rngs::OsRng;

pub struct PedersenCommitment<G: CurveGroup> {
    g: Vec<G>,
    h: G,
    r: G::ScalarField,
    com: G
}

impl<G: CurveGroup> PedersenCommitment<G> {
    pub fn commit(x: Vec<G::ScalarField>, r: G::ScalarField) -> PedersenCommitment<G> {
        let mut g = Vec::new();
        for _ in 0..x.len() {
            g.push(G::rand(&mut OsRng {}));
        }
        let h = G::rand(&mut OsRng {});
        let com = (h * r) + G::msm(&CurveGroup::normalize_batch(&g), &x).unwrap();

        PedersenCommitment { g, h, r, com }
    }
}
