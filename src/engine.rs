use crate::data::{AgtData, SimData};
use crate::params::Params;
use anyhow::{Context, Result};
use ndarray::Array1;
use rand::prelude::*;
use rand_chacha::ChaCha12Rng;
use rand_distr::{Bernoulli, LogNormal, Uniform, weighted::WeightedIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimEng {
    sim_data: SimData,
    prng: ChaCha12Rng,
    par: Params,
}

impl SimEng {
    pub fn new(par: Params) -> Result<Self> {
        Ok(Self {
            sim_data: SimData::new(par.n_agt_init),
            prng: ChaCha12Rng::try_from_os_rng()?,
            par,
        })
    }

    pub fn generate_initial_condition(&mut self) -> Result<()> {
        let env_dist = Uniform::new(0, self.par.n_env)?;
        self.sim_data.env = env_dist.sample(&mut self.prng);

        self.sim_data.agt_vec.clear();
        self.sim_data.agt_vec.reserve(self.par.n_agt_init);

        let phe_dist = Uniform::new(0, self.par.n_phe)?;
        for _ in 0..self.par.n_agt_init {
            let phe = phe_dist.sample(&mut self.prng);
            let prob = vec![1.0 / self.par.n_phe as f64; self.par.n_phe];
            let prob_phe = Array1::from(prob);
            self.sim_data
                .agt_vec
                .push(AgtData::new(phe, prob_phe, self.par.n_phe).context("...")?);
        }

        Ok(())
    }

    fn perform_step(
        &mut self,
        mut_dist: &LogNormal<f64>,
        i_agt_rep: &mut Vec<usize>,
        i_agt_dec: &mut Vec<usize>,
        i_agt_all: &mut Vec<usize>,
    ) -> Result<()> {
        // 1. Update environment
        let env_dist = WeightedIndex::new(self.par.prob_env.row(self.sim_data.env))?;
        self.sim_data.env = env_dist.sample(&mut self.prng);

        self.sim_data.n_agt_diff = 0;

        // 2. Create reproduction and death distributions
        let rep_dist_vec: Vec<_> = self
            .par
            .prob_rep
            .outer_iter()
            .map(|v| Bernoulli::new(v[self.sim_data.env]).unwrap()) //PROBABLY WRONG
            .collect();

        let dec_dist_vec: Vec<_> = self
            .par
            .prob_dec
            .outer_iter()
            .map(|v| Bernoulli::new(v[self.sim_data.env]).unwrap()) //PROBABLY WRONG
            .collect();

        // 3. Select replicating and dying agents
        i_agt_rep.clear();
        i_agt_dec.clear();

        for (i, agt) in self.sim_data.agt_vec.iter().enumerate() {
            let phe = agt.phe();
            if rep_dist_vec[phe].sample(&mut self.prng) {
                i_agt_rep.push(i);
            }
            if dec_dist_vec[phe].sample(&mut self.prng) {
                i_agt_dec.push(i);
            }
        }

        // 4. Reproduce and mutate
        for i in i_agt_rep {
            let prob_phe = self.sim_data.agt_vec[*i].prob_phe().clone();

            let dist = WeightedIndex::new(prob_phe.iter())?;
            let phe_new = dist.sample(&mut self.prng);

            let mut prob_phe_new: Vec<f64> = prob_phe
                .iter()
                .map(|&x| x * mut_dist.sample(&mut self.prng))
                .collect();

            let norm: f64 = prob_phe_new.iter().sum();
            for x in &mut prob_phe_new {
                *x /= norm;
            }

            let new_agt =
                AgtData::new(phe_new, Array1::from(prob_phe_new), self.par.n_phe).context("...")?;
            self.sim_data.agt_vec.push(new_agt);

            self.sim_data.n_agt_diff += 1;
        }

        // 5. Remove dead agents
        i_agt_dec.sort_by(|a, b| b.cmp(a)); // reverse sort
        for i in i_agt_dec {
            self.sim_data.agt_vec.swap_remove(*i);
            self.sim_data.n_agt_diff -= 1;
        }

        // 6. Remove excess agents
        let n_agt = self.sim_data.agt_vec.len();
        if n_agt > self.par.n_agt_init {
            let excess = n_agt - self.par.n_agt_init;

            i_agt_all.clear();
            i_agt_all.extend(0..n_agt);

            let i_agt_rm: HashSet<usize> = i_agt_all
                .choose_multiple(&mut self.prng, excess)
                .cloned()
                .collect();

            let mut i_agt_rm: Vec<_> = i_agt_rm.into_iter().collect();

            i_agt_rm.sort_by(|a, b| b.cmp(a));
            for i in i_agt_rm {
                self.sim_data.agt_vec.swap_remove(i);
            }
        }

        Ok(())
    }

    pub fn run_simulation<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        let mut_dist = LogNormal::new(0.0, self.par.std_dev_mut)?;

        let mut i_agt_rep: Vec<usize> = Vec::with_capacity(self.par.n_agt_init);
        let mut i_agt_dec: Vec<usize> = Vec::with_capacity(self.par.n_agt_init);
        let mut i_agt_all: Vec<usize> = Vec::with_capacity(2 * self.par.n_agt_init);

        let file = file.as_ref();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file)
            .with_context(|| format!("failed to open {:?}", file))?;

        let mut writer = BufWriter::new(file);

        for i_save in 0..self.par.saves_per_file {
            for _ in 0..self.par.steps_per_save {
                self.perform_step(&mut_dist, &mut i_agt_rep, &mut i_agt_dec, &mut i_agt_all)
                    .context("...")?;
            }

            self.sim_data.write_frame(&mut writer)?;
            // log::info!("SimData: {:#?}", self.sim_data);

            let prog_pc = 100.0 * (i_save + 1) as f64 / self.par.saves_per_file as f64;
            log::info!("progress: {:06.2}%", prog_pc);
        }

        writer.flush()?;

        log::info!("simulation ended");

        Ok(())
    }
}
