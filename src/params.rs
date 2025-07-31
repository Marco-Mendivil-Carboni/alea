use anyhow::{Context, Result, bail};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
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

macro_rules! check_number {
    ($value:expr, $range:expr) => {
        _check_number($value, stringify!($value), $range)
    };
}

macro_rules! check_vector {
    ($vector:expr, $expected_len:expr, $check_prob:expr) => {
        _check_vector(&$vector, stringify!($vector), $expected_len, $check_prob)
    };
}

macro_rules! check_matrix {
    ($matrix:expr, $expected_shape:expr, $check_trans:expr) => {
        _check_matrix($matrix, stringify!($matrix), $expected_shape, $check_trans)
    };
}

impl MdlPar {
    pub fn new(params: &str) -> Result<Self> {
        let mdl_par: MdlPar =
            ron::de::from_str(params).context("failed to deserialize MdlPar value from string")?;

        check_number!(mdl_par.n_env, 1..100)?;
        check_number!(mdl_par.n_phe, 1..100)?;

        check_matrix!(
            mdl_par.prob_env.view(),
            &[mdl_par.n_env, mdl_par.n_env],
            true
        )?;
        check_matrix!(
            mdl_par.prob_rep.view(),
            &[mdl_par.n_phe, mdl_par.n_env],
            false
        )?;
        check_matrix!(
            mdl_par.prob_dec.view(),
            &[mdl_par.n_phe, mdl_par.n_env],
            false
        )?;

        check_number!(mdl_par.n_agt_init, 1..10_000)?;
        check_number!(mdl_par.std_dev_mut, 0.0..1.0)?;

        Ok(mdl_par)
    }
}

fn _check_number<T, R>(value: T, name: &str, range: R) -> Result<()>
where
    T: PartialOrd + Display,
    R: RangeBounds<T> + Debug,
{
    if !range.contains(&value) {
        bail!("{name} must be in the range {:?}, but is {value}", range);
    }
    Ok(())
}

fn _check_vector(
    vector: ArrayView1<f64>,
    name: &str,
    expected_len: usize,
    check_prob: bool,
) -> Result<()> {
    let len = vector.len();
    if len != expected_len {
        bail!("{name} must have length {expected_len}, but has {len}");
    }
    if !check_prob {
        return Ok(());
    }
    if vector.iter().any(|&element| element < 0.0) {
        bail!("{name} must have only non-negative elements");
    }
    let sum: f64 = vector.iter().sum();
    let tol = 1e-6;
    if (sum - 1.0).abs() > tol {
        bail!("{name} must sum to 1.0 (tolerance: {tol}), but sums to {sum}");
    }
    Ok(())
}

fn _check_matrix(
    matrix: ArrayView2<f64>,
    name: &str,
    expected_shape: &[usize],
    check_trans: bool,
) -> Result<()> {
    let shape = matrix.shape();
    if shape != expected_shape {
        bail!(
            "{name} must have shape {:?}, but has {:?}",
            expected_shape,
            shape
        );
    }
    if !check_trans {
        return Ok(());
    }
    if shape[0] != shape[1] {
        bail!("{name} is not a square matrix");
    }
    for (i_row, row) in matrix.outer_iter().enumerate() {
        _check_vector(row, &format!("row {i_row} of {name}"), shape[1], true)?;
    }
    Ok(())
}
