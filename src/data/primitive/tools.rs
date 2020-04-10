use crate::data::ast::Interval;
use crate::error_format::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

pub enum Integer {
    Int(i64),
    Float(f64),
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_integer(text: &str) -> Result<Integer, ErrorInfo> {
    match (text.parse::<i64>(), text.parse::<f64>()) {
        (Ok(int), _) => Ok(Integer::Int(int)),
        (_, Ok(float)) => Ok(Integer::Float(float)),
        (..) => Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_OPS.to_owned(),
        )),
    }
}

pub fn check_division_by_zero_i64(lhs: i64, rhs: i64) -> Result<i64, ErrorInfo> {
    if lhs == 0 || rhs == 0 {
        return Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_OPS_DIV_INT.to_owned(),
        ));
    }

    Ok(lhs)
}

pub fn check_division_by_zero_f64(lhs: f64, rhs: f64) -> Result<f64, ErrorInfo> {
    if lhs == 0.0 || rhs == 0.0 {
        return Err(gen_error_info(
            Interval { column: 0, line: 0 },
            ERROR_OPS_DIV_FLOAT.to_owned(),
        ));
    }

    Ok(lhs)
}
