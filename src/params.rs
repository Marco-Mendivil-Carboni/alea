use anyhow::{Context, Result, bail};
use ndarray::{Array2, ArrayView1, ArrayView2};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::RangeBounds;
// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct MdlPar {
    pub n_env: usize,
    pub n_phe: usize,

    pub prob_env: Array2<f64>,
    pub prob_rep: Array2<f64>,
    pub prob_dec: Array2<f64>,

    pub n_agt_init: usize,

    pub std_dev_mut: f64,
}

impl MdlPar {
    pub fn new(params: &str) -> Result<Self> {
        let mdl_par: MdlPar =
            ron::de::from_str(params).context("failed to deserialize MdlPar value from string")?;

        check_number(mdl_par.n_env, "number of environments", 1..100)?;
        check_number(mdl_par.n_phe, "number of phenotypes", 1..100)?;

        check_matrix(
            mdl_par.prob_env.view(),
            "environment probability matrix",
            (mdl_par.n_env, mdl_par.n_env),
            true,
        )?;
        check_matrix(
            mdl_par.prob_rep.view(),
            "replication probability matrix",
            (mdl_par.n_phe, mdl_par.n_env),
            false,
        )?;
        check_matrix(
            mdl_par.prob_dec.view(),
            "decease probability matrix",
            (mdl_par.n_phe, mdl_par.n_env),
            false,
        )?;

        check_number(mdl_par.n_agt_init, "initial number of agents", 1..10_000)?;

        check_number(mdl_par.std_dev_mut, "mutation standard deviation", 0.0..1.0)?;

        Ok(mdl_par)
    }
}

fn check_number<T, R>(value: T, value_name: &str, range: R) -> Result<()>
where
    T: PartialOrd + Display,
    R: RangeBounds<T> + Debug,
{
    if !range.contains(&value) {
        bail!(
            "{value_name} must be in the range {:?}, but is {value}",
            range
        );
    }

    Ok(())
}

fn check_vector(
    vector: ArrayView1<f64>,
    vector_name: &str,
    expected_len: usize,
    check_prob: bool,
) -> Result<()> {
    let len = vector.len();
    if len != expected_len {
        bail!("{vector_name} length must be {expected_len}, but is {len}");
    }

    if !check_prob {
        return Ok(());
    }
    if vector.iter().any(|&element| element < 0.0) {
        bail!("{vector_name} must have only non-negative elements");
    }
    let sum: f64 = vector.iter().sum();
    let tol = 1e-6;
    if (sum - 1.0).abs() > tol {
        bail!("{vector_name} must sum to 1.0 (tolerance: {tol}), but sums to {sum}");
    }

    Ok(())
}

fn check_matrix(
    matrix: ArrayView2<f64>,
    matrix_name: &str,
    expected_dim: (usize, usize),
    check_trans: bool,
) -> Result<()> {
    let dim = matrix.dim();
    if dim != expected_dim {
        bail!(
            "{matrix_name} shape must be {:?}, but is {:?}",
            expected_dim,
            dim
        );
    }

    if !check_trans {
        return Ok(());
    }
    if dim.0 != dim.1 {
        bail!("{matrix_name} is not a square matrix");
    }
    for (i_row, row) in matrix.outer_iter().enumerate() {
        check_vector(row, &format!("row {i_row} of {matrix_name}"), dim.1, true)?;
    }

    Ok(())
}
