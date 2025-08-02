use crate::data::{AgtData, SimData};
use crate::params::Params;
use anyhow::{Context, Result};
use ndarray::Array1;
use rand::SeedableRng;
use rand::prelude::*;
use rand_chacha::ChaCha12Rng;
use rand_distr::{LogNormal, weighted::WeightedIndex};
use std::collections::HashSet;
use std::{io::Write, path::Path};

#[derive(Debug)]
pub struct SimEng {
    par: Params,

    sim_data: SimData,

    prng: ChaCha12Rng,

    mut_dist: LogNormal<f64>,

    i_agt_rep: Vec<usize>,
    i_agt_dec: Vec<usize>,
    i_agt_all: Vec<usize>,
}

impl SimEng {
    pub fn new(par: Params) -> Result<Self> {
        let n_agt_init = par.n_agt_init;
        let std_dev_mut = par.std_dev_mut;
        let sim_eng = Self {
            par: par,
            sim_data: SimData {
                env: 0,
                agt_vec: Vec::with_capacity(n_agt_init),
                n_agt_diff: 0,
            },
            prng: ChaCha12Rng::try_from_os_rng()?,
            mut_dist: LogNormal::new(0.0, std_dev_mut)?,
            i_agt_rep: Vec::with_capacity(n_agt_init),
            i_agt_dec: Vec::with_capacity(n_agt_init),
            i_agt_all: Vec::with_capacity(2 * n_agt_init),
        };
        Ok(sim_eng)
    }

    pub fn generate_initial_condition(&mut self) {
        let env_dist = rand::distr::Uniform::new(0, self.par.n_env);
        self.sim_data.env = env_dist.expect("...").sample(&mut self.prng);

        self.sim_data.agt_vec.clear();
        let phe_dist = rand::distr::Uniform::new(0, self.par.n_phe);

        for _ in 0..self.par.n_agt_init {
            let phe = phe_dist.expect("...").sample(&mut self.prng);
            let prob = vec![1.0 / self.par.n_phe as f64; self.par.n_phe];
            let prob_phe = Array1::from(prob);
            self.sim_data
                .agt_vec
                .push(AgtData::new(phe, prob_phe, self.par.n_phe).unwrap());
        }
    }

    pub fn perform_step(&mut self) -> Result<()> {
        // 1. Update environment
        let env_dist = WeightedIndex::new(self.par.prob_env.row(self.sim_data.env))?;
        self.sim_data.env = env_dist.sample(&mut self.prng);

        self.sim_data.n_agt_diff = 0;

        // 2. Create reproduction and death distributions
        let rep_dist_vec: Vec<_> = self
            .par
            .prob_rep
            .outer_iter()
            .map(|v| rand::distr::Bernoulli::new(v[self.sim_data.env]).unwrap()) //PROBABLY WRONG
            .collect();

        let dec_dist_vec: Vec<_> = self
            .par
            .prob_dec
            .outer_iter()
            .map(|v| rand::distr::Bernoulli::new(v[self.sim_data.env]).unwrap()) //PROBABLY WRONG
            .collect();

        // 3. Select replicating and dying agents
        self.i_agt_rep.clear();
        self.i_agt_dec.clear();

        for (i, agt) in self.sim_data.agt_vec.iter().enumerate() {
            let phe = agt.phe();
            if rep_dist_vec[phe].sample(&mut self.prng) {
                self.i_agt_rep.push(i);
            }
            if dec_dist_vec[phe].sample(&mut self.prng) {
                self.i_agt_dec.push(i);
            }
        }

        // 4. Reproduce and mutate
        for &i in &self.i_agt_rep {
            let prob_phe = self.sim_data.agt_vec[i].prob_phe().clone();

            let dist = WeightedIndex::new(prob_phe.iter())?;
            let phe_new = dist.sample(&mut self.prng);

            let mut prob_phe_new: Vec<f64> = prob_phe
                .iter()
                .map(|&x| x * self.mut_dist.sample(&mut self.prng))
                .collect();

            let norm: f64 = prob_phe_new.iter().sum();
            for x in &mut prob_phe_new {
                *x /= norm;
            }

            let new_agt = AgtData::new(phe_new, Array1::from(prob_phe_new), self.par.n_phe)?;
            self.sim_data.agt_vec.push(new_agt);

            self.sim_data.n_agt_diff += 1;
        }

        // 5. Remove dead agents
        self.i_agt_dec.sort_by(|a, b| b.cmp(a)); // reverse sort
        for &i in &self.i_agt_dec {
            self.sim_data.agt_vec.swap_remove(i);
            self.sim_data.n_agt_diff -= 1;
        }

        // 6. Remove excess agents
        let n_agt = self.sim_data.agt_vec.len();
        if n_agt > self.par.n_agt_init {
            let excess = n_agt - self.par.n_agt_init;

            self.i_agt_all.clear();
            self.i_agt_all.extend(0..n_agt);

            let i_agt_rm: HashSet<usize> = self
                .i_agt_all
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

    pub fn run_simulation<P>(&mut self, file: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        for i_save in 0..self.par.saves_per_file {
            let prog_pc = 100.0 * i_save as f64 / self.par.saves_per_file as f64;
            print!("progress: {:06.2}%\r", prog_pc);
            std::io::stdout().flush()?;

            for _ in 0..self.par.steps_per_save {
                self.perform_step()?;
            }

            self.sim_data.write_frame(&file)?;
        }

        log::info!("simulation ended");

        Ok(())
    }
}
