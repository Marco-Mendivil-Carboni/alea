use crate::params::{check_number, check_vector};
use anyhow::{Context, Result};
use ndarray::Array1;
use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgtData {
    phe: usize,
    prob_phe: Array1<f64>,
}

impl AgtData {
    pub fn new(phe: usize, prob_phe: Array1<f64>, n_phe: usize) -> Result<Self> {
        check_number(phe, 0..n_phe).context("invalid phenotype")?;
        check_vector(prob_phe.view(), n_phe, true).context("invalid phenotype probabilities")?;
        Ok(Self { phe, prob_phe })
    }

    pub fn phe(&self) -> usize {
        self.phe
    }

    pub fn prob_phe(&self) -> &Array1<f64> {
        &self.prob_phe
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimData {
    pub env: usize,
    pub agt_vec: Vec<AgtData>,
    pub n_agt_diff: i32,
}

impl SimData {
    pub fn new(n_agt_init: usize) -> Self {
        Self {
            env: 0,
            agt_vec: Vec::with_capacity(n_agt_init),
            n_agt_diff: 0,
        }
    }

    pub fn read_frame<P>(file: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        let data = fs::read(file).with_context(|| format!("failed to read {:?}", file))?;
        from_bytes(&data).context("failed to deserialize SimData value from bytes")
    }

    pub fn write_frame<P>(&self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        let data = to_allocvec(self).context("failed to serialize SimData value to bytes")?;
        fs::write(file, data).with_context(|| format!("failed to write {:?}", file))
    }
}
