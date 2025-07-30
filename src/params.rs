use anyhow::{Result, bail};
use serde::Deserialize;
use std::fmt::{Debug, Display};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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

fn number_is_valid<T, R>(val: T, name: &str, range: R) -> Result<()>
where
    T: PartialOrd + Display,
    R: RangeBounds<T> + Debug,
{
    if !range.contains(&val) {
        bail!("`{name}` must be in the range {:?}, but is {val}", range);
    }
    Ok(())
}

macro_rules! number_is_valid {
    ($val:expr, $range:expr) => {{ number_is_valid($val, stringify!($val), $range) }};
}

impl MdlPar {
    // Constructor from serde-compatible data (e.g. JSON, YAML)
    pub fn new(params: &serde_json::Value) -> Result<Self> {
        let mdlpar: MdlPar = serde_json::from_value(params.clone())
            .map_err(|e| anyhow::anyhow!("Failed to deserialize MdlPar: {}", e))?;

        number_is_valid!(mdlpar.n_env, 1..100)?;
        number_is_valid!(mdlpar.n_phe, 1..100)?;

        number_is_valid!(mdlpar.n_agt_init, 1..10_000)?;
        number_is_valid!(mdlpar.std_dev_mut, 0.0..1.0)?;

        Ok(mdlpar)
    }
}
