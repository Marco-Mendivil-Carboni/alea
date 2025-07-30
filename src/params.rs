use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::fmt::{Debug, Display};
// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};
use std::ops::RangeBounds;

pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug, Deserialize)]
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
        bail!("`{name}` must be in the range {:?}, but is {val}", range);
    }
    Ok(())
}

macro_rules! check_number {
    ($val:expr, $range:expr) => {
        _check_number($val, stringify!($val), $range)
    };
}

impl MdlPar {
    pub fn new(params: serde_json::Value) -> Result<Self> {
        let mdl_par: MdlPar =
            serde_json::from_value(params).context("failed to deserialize MdlPar")?;

        check_number!(mdl_par.n_env, 1..100)?;
        check_number!(mdl_par.n_phe, 1..100)?;

        check_number!(mdl_par.n_agt_init, 1..10_000)?;
        check_number!(mdl_par.std_dev_mut, 0.0..1.0)?;

        Ok(mdl_par)
    }
}
