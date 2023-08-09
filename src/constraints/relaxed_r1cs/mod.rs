use ark_ec::CurveGroup;

use crate::{commits::pedersen::PedersenCommitment, arithmetic::{scale_vec, add_vec}};
use self::types::{FoldedR1CSInstance, FoldedR1CSWitness};

use super::{r1cs::R1CS, Instance, Witness};

pub mod types;

#[allow(non_snake_case)]
pub fn fold<G: CurveGroup>(
    R: &R1CS<G::ScalarField>,
    instance_one: &FoldedR1CSInstance<G>,
    instance_two: &FoldedR1CSInstance<G>,
    witness_one: &FoldedR1CSWitness<G::ScalarField>,
    witness_two: &FoldedR1CSWitness<G::ScalarField>,
) {
    todo!()
}

pub fn fold_instances<G: CurveGroup>(
    instance_one: &FoldedR1CSInstance<G>,
    instance_two: &FoldedR1CSInstance<G>,
    cross_terms_commitment: &G,
    r: &G::ScalarField
) -> FoldedR1CSInstance<G> {
    FoldedR1CSInstance {
        x: Instance { 
            x: add_vec(
                &instance_one.x.x, 
                &scale_vec(&instance_two.x.x, r)
            ) 
        },
        com_error: instance_one.com_error + (*cross_terms_commitment * r) + (instance_two.com_error * r * r),
        com_witness: instance_one.com_witness + (instance_two.com_witness * r),
        u: instance_one.u + (instance_two.u * r),
    }
}

pub fn fold_witnesses<G: CurveGroup>(
    witness_one: &FoldedR1CSWitness<G::ScalarField>,
    witness_two: &FoldedR1CSWitness<G::ScalarField>,
    cross_terms: &Vec<G::ScalarField>,
    r_cross_terms: &G::ScalarField,
    r: &G::ScalarField
) -> FoldedR1CSWitness<G::ScalarField> {
    let r_squared = *r * r;

    FoldedR1CSWitness { 
        w: Witness {
            w: add_vec(
                &witness_one.w.w, 
                &scale_vec(&witness_two.w.w, r)) 
        }, 
        error: add_vec(
            &witness_one.error, 
            &add_vec(
                &scale_vec(cross_terms, r),
                &scale_vec(&witness_two.error, &r_squared)
            )
        ), 
        r_error: witness_one.r_error + (*r_cross_terms * r) + (witness_two.r_error * r_squared), 
        w_error: witness_one.w_error + (witness_two.w_error * r) 
    }
}