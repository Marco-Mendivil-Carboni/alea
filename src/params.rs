use serde::Deserialize;

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

fn check_number_range<T: PartialOrd + std::fmt::Display>(
    number: T,
    name: &str,
    lower: T,
    upper: T,
) -> Result<(), String> {
    if lower <= number && number < upper {
        Ok(())
    } else {
        Err(format!(
            "The {} value {} is out of range ({}-{}).",
            name, number, lower, upper
        ))
    }
}
