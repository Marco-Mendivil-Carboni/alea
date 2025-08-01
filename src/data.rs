use crate::Params;
use anyhow::{Result, bail};
use ndarray::Array1;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct AgtData {
    phe: usize,
    prob_phe: Array1<f64>,
}

impl AgtData {
    pub fn new(phe: usize, prob_phe: Array1<f64>, n_phe: usize) -> Result<Self> {
        if phe >= n_phe {
            bail!("phe must be in [0, {})", n_phe);
        }
        if prob_phe.len() != n_phe {
            bail!("prob_phe must have length {}", n_phe);
        }
        let sum: f64 = prob_phe.sum();
        if (sum - 1.0).abs() > 1e-6 {
            bail!("prob_phe must sum to 1.0, got {}", sum);
        }
        Ok(Self { phe, prob_phe })
    }

    pub fn phe(&self) -> usize {
        self.phe
    }

    pub fn prob_phe(&self) -> &Array1<f64> {
        &self.prob_phe
    }
}

pub struct SimData {
    par: Params,
    env: usize,
    agt_vec: Vec<AgtData>,
    n_agt_diff: i32,
}

impl SimData {
    pub fn new(par_str: &str) -> Result<Self> {
        let par = Params::new(par_str)?;

        Ok(Self {
            par,
            env: 0,
            agt_vec: Vec::new(),
            n_agt_diff: 0,
        })
    }

    pub fn write_frame(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_all(&self.env.to_le_bytes())?;

        let n_agt = self.agt_vec.len();
        writer.write_all(&(n_agt as u64).to_le_bytes())?;

        for agt in &self.agt_vec {
            writer.write_all(&agt.phe().to_le_bytes())?;
            for val in agt.prob_phe() {
                writer.write_all(&val.to_le_bytes())?;
            }
        }

        writer.write_all(&self.n_agt_diff.to_le_bytes())?;

        Ok(())
    }

    pub fn read_frame(&mut self, reader: &mut impl Read) -> Result<()> {
        let mut buf = [0u8; 8];

        reader.read_exact(&mut buf[..std::mem::size_of::<usize>()])?;
        self.env = usize::from_le_bytes(buf);

        reader.read_exact(&mut buf[..std::mem::size_of::<u64>()])?;
        let n_agt = u64::from_le_bytes(buf) as usize;

        self.agt_vec.clear();
        for _ in 0..n_agt {
            reader.read_exact(&mut buf[..std::mem::size_of::<usize>()])?;
            let phe = usize::from_le_bytes(buf);

            let mut data = vec![0.0f64; self.par.n_phe];
            for i in 0..self.par.n_phe {
                let mut fbuf = [0u8; 8];
                reader.read_exact(&mut fbuf)?;
                data[i] = f64::from_le_bytes(fbuf);
            }
            self.agt_vec
                .push(AgtData::new(phe, Array1::from(data), self.par.n_phe)?);
        }

        let mut diff_buf = [0u8; std::mem::size_of::<i32>()];
        reader.read_exact(&mut diff_buf)?;
        self.n_agt_diff = i32::from_le_bytes(diff_buf);

        Ok(())
    }
}
