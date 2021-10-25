fn arg_validator_positive_f64_impl(v: String, zero_ok: bool) -> Result<(), String> {
    match v.parse::<f64>() {
        Ok(i) => {
            if i < 0.0 + (if zero_ok { 0.0 } else { f64::EPSILON }) {
                Err(String::from("Value too small"))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(String::from("Value must be a float")),
    }
}

pub fn arg_validator_positive_f64(v: String) -> Result<(), String> {
    arg_validator_positive_f64_impl(v, false)
}

pub fn arg_validator_positive_or_zero_f64(v: String) -> Result<(), String> {
    arg_validator_positive_f64_impl(v, true)
}

pub fn arg_validator_isize(v: String) -> Result<(), String> {
    match v.parse::<isize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Value must be an integer")),
    }
}

pub fn arg_validator_usize(v: String) -> Result<(), String> {
    match v.parse::<isize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Value must be a positive integer")),
    }
}
