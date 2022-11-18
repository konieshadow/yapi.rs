use validator::ValidationError;

use super::join_vec;

pub fn valid_one_of<T>(value: T, posible_values: Vec<T>) -> Result<(), ValidationError>
where T: PartialEq + ToString
{
    if posible_values.contains(&value) {
        Ok(())
    } else {
        let mut err = ValidationError::new("one_of");
        let values = join_vec(posible_values, ", ");
        err.message = Some(format!("must be one of {}", values).into());
        Err(err)
    }
}