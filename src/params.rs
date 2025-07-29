
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
