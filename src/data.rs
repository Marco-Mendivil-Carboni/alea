use crate::params::{check_number, check_vector};
use anyhow::{Context, Result};
use ndarray::Array1;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = fs::read(path).context("failed to read SimData file")?;
        let sim_data = postcard::from_bytes(&data).context("failed to deserialize SimData")?;
        Ok(sim_data)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = postcard::to_allocvec(self).context("failed to serialize SimData")?;
        fs::write(path, data).context("failed to write SimData to file")
    }
}
