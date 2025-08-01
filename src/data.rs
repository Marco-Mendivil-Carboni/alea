use crate::params::{check_number, check_vector};
use anyhow::{Context, Result};
use ndarray::Array1;
use serde::{Deserialize, Serialize};

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
