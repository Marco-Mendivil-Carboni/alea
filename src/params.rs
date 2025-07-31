use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::RangeBounds;
// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};

pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MdlPar {
    pub n_env: usize,
    pub n_phe: usize,

    pub prob_env: Matrix,
    pub prob_rep: Matrix,
    pub prob_dec: Matrix,

    pub n_agt_init: usize,

    pub std_dev_mut: f64,
}

fn _check_number<T, R>(val: T, name: &str, range: R) -> Result<()>
where
    T: PartialOrd + Display,
    R: RangeBounds<T> + Debug,
{
    if !range.contains(&val) {
        bail!("{name} must be in the range {:?}, but is {val}", range);
    }
    Ok(())
}

macro_rules! check_number {
    ($val:expr, $range:expr) => {
        _check_number($val, stringify!($val), $range)
    };
}

fn _check_vector(vec: &[f64], name: &str, exp_len: usize, check_prob: bool) -> Result<()> {
    if vec.len() != exp_len {
        bail!("{name} must have {exp_len} elements, but has {}", vec.len());
    }
    if !check_prob {
        return Ok(());
    }
    if vec.iter().any(|&x| x < 0.0) {
        bail!("{name} must have only non-negative elements");
    }
    let sum: f64 = vec.iter().sum();
    let tol = 1e-6;
    if (sum - 1.0).abs() > tol {
        bail!("{name} must sum to 1.0 (tolerance: {tol}), but sums to {sum}");
    }
    Ok(())
}

macro_rules! check_vector {
    ($vec:expr, $expected_len:expr, $check_prob:expr) => {
        _check_vector(&$vec, stringify!($vec), $expected_len, $check_prob)
    };
}

fn _check_matrix(
    mat: &[Vec<f64>],
    name: &str,
    exp_size: (usize, usize),
    check_trans: bool,
) -> Result<()> {
    let (exp_rows, exp_cols) = exp_size;
    if mat.len() != exp_rows {
        bail!("{name} must have {exp_rows} rows, but has {}", mat.len());
    }
    for (i, row) in mat.iter().enumerate() {
        if row.len() != exp_cols {
            bail!(
                "{name} row {} length must be {}, but is {}",
                i,
                exp_cols,
                row.len()
            );
        }
    }
    if !check_trans {
        return Ok(());
    }
    if exp_rows != exp_cols {
        bail!("{name} is not a square matrix");
    }
    for (i, row) in mat.iter().enumerate() {
        _check_vector(row, &format!("{} row {}", name, i), exp_cols, true)?;
    }
    Ok(())
}

macro_rules! check_matrix {
    ($matrix:expr, $expected_size:expr, $check_trans:expr) => {
        _check_matrix(&$matrix, stringify!($matrix), $expected_size, $check_trans)
    };
}

impl MdlPar {
    pub fn new(params: serde_yaml::Value) -> Result<Self> {
        let mdl_par: MdlPar =
            serde_yaml::from_value(params).context("failed to deserialize MdlPar")?;

        check_number!(mdl_par.n_env, 1..100)?;
        check_number!(mdl_par.n_phe, 1..100)?;

        check_matrix!(mdl_par.prob_env, (mdl_par.n_env, mdl_par.n_env), true)?;
        check_matrix!(mdl_par.prob_rep, (mdl_par.n_phe, mdl_par.n_env), false)?;
        check_matrix!(mdl_par.prob_dec, (mdl_par.n_phe, mdl_par.n_env), false)?;

        check_number!(mdl_par.n_agt_init, 1..10_000)?;
        check_number!(mdl_par.std_dev_mut, 0.0..1.0)?;

        Ok(mdl_par)
    }
}
