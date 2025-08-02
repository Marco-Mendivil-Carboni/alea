use anyhow::{Context, Result, bail};
use ndarray::{Array2, ArrayView1, ArrayView2};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub n_env: usize,
    pub n_phe: usize,

    pub prob_env: Array2<f64>,
    pub prob_rep: Array2<f64>,
    pub prob_dec: Array2<f64>,

    pub n_agt_init: usize,

    pub std_dev_mut: f64,

    pub steps_per_save: usize,
    pub saves_per_file: usize,
}

impl Params {
    pub fn new(par_str: &str) -> Result<Self> {
        let par: Params =
            ron::de::from_str(par_str).context("failed to deserialize Params value from string")?;

        check_number(par.n_env, 1..100).context("invalid number of environments")?;
        check_number(par.n_phe, 1..100).context("invalid number of phenotypes")?;

        check_matrix(par.prob_env.view(), (par.n_env, par.n_env), true)
            .context("invalid environment probabilities")?;
        check_matrix(par.prob_rep.view(), (par.n_phe, par.n_env), false)
            .context("invalid replication probabilities")?;
        check_matrix(par.prob_dec.view(), (par.n_phe, par.n_env), false)
            .context("invalid decease probabilities")?;

        check_number(par.n_agt_init, 1..100_000).context("invalid initial number of agents")?;

        check_number(par.std_dev_mut, 0.0..1.0).context("invalid mutation standard deviation")?;

        check_number(par.steps_per_save, 1..10_000).context("invalid number of steps per save")?;
        check_number(par.saves_per_file, 1..10_000).context("invalid number of saves per file")?;

        Ok(par)
    }
}

pub fn check_number<T, R>(val: T, range: R) -> Result<()>
where
    T: PartialOrd + Display,
    R: RangeBounds<T> + Debug,
{
    if !range.contains(&val) {
        bail!("value must be in the range {:?}, but is {val}", range);
    }

    Ok(())
}

pub fn check_vector(vec: ArrayView1<f64>, exp_len: usize, prob_vec: bool) -> Result<()> {
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
        check_vector(row, dim.1, true).with_context(|| format!("invalid row {i_row}"))?;
    }

    Ok(())
}
