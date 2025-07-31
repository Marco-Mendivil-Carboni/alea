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
        let mp: MdlPar =
            ron::de::from_str(params).context("failed to deserialize MdlPar value from string")?;

        check_number(mp.n_env, 1..100).context("number of environments")?;
        check_number(mp.n_phe, 1..100).context("number of phenotypes")?;

        check_matrix(mp.prob_env.view(), (mp.n_env, mp.n_env), true)
            .context("environment probability matrix")?;
        check_matrix(mp.prob_rep.view(), (mp.n_phe, mp.n_env), false)
            .context("replication probability matrix")?;
        check_matrix(mp.prob_dec.view(), (mp.n_phe, mp.n_env), false)
            .context("decease probability matrix")?;

        check_number(mp.n_agt_init, 1..10_000).context("initial number of agents")?;

        check_number(mp.std_dev_mut, 0.0..1.0).context("mutation standard deviation")?;

        Ok(mp)
    }
}

fn check_number<T, R>(val: T, range: R) -> Result<()>
where
    T: PartialOrd + Display,
    R: RangeBounds<T> + Debug,
{
    if !range.contains(&val) {
        bail!("value must be in the range {:?}, but is {val}", range);
    }

    Ok(())
}

fn check_vector(vec: ArrayView1<f64>, exp_len: usize, prob_vec: bool) -> Result<()> {
    let len = vec.len();
    if len != exp_len {
        bail!("vector length must be {exp_len}, but is {len}");
    }

    if !prob_vec {
        return Ok(());
    }
    if vec.iter().any(|&ele| ele < 0.0) {
        bail!("vector must have only non-negative elements");
    }
    let sum: f64 = vec.iter().sum();
    let tol = 1e-6;
    if (sum - 1.0).abs() > tol {
        bail!("vector must sum to 1.0 (tolerance: {tol}), but sums to {sum}");
    }

    Ok(())
}

fn check_matrix(mat: ArrayView2<f64>, exp_dim: (usize, usize), trans_mat: bool) -> Result<()> {
    let dim = mat.dim();
    if dim != exp_dim {
        bail!("matrix shape must be {:?}, but is {:?}", exp_dim, dim);
    }

    if !trans_mat {
        return Ok(());
    }
    if dim.0 != dim.1 {
        bail!("matrix is not square");
    }
    for (i_row, row) in mat.outer_iter().enumerate() {
        check_vector(row, dim.1, true).with_context(|| format!("row {i_row}"))?;
    }

    Ok(())
}
