use sea_orm::{ActiveValue, Value};


pub trait AutoTimestamp {
    fn default_add() -> Self;
    fn default_up() -> Self;
}

pub trait OptioniActiveValue<V>
where V: Into<Value>
{
    fn to_active_value(self) -> ActiveValue<V>;
}

impl <V> OptioniActiveValue<V> for V
where V: Into<Value>
{
    fn to_active_value(self) -> ActiveValue<V> {
        sea_orm::Set(self)
    }
}

impl <V> OptioniActiveValue<V> for Option<V>
where V: Into<Value>
{
    fn to_active_value(self) -> ActiveValue<V> {
        self.map(sea_orm::Set).unwrap_or(sea_orm::ActiveValue::NotSet)
    }
}