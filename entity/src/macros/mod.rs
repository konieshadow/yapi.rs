#[macro_export]
macro_rules! set {
    ( $val:expr ) => {
        {
            yapi_entity::traits::OptioniActiveValue::to_active_value($val)
        }
    }
}
