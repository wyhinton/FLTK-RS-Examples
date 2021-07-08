use crate::Distance;
use fltk::enums::Color;
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomInputValue {
    CiBool(bool),
    CiInt(i32),
    CiFloat(Distance),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InputType {
    BoolInput,
    IntInput,
    FloatInput,
}

#[derive(Debug, Clone)]
pub struct FloatInputSettings {
    pub min: f64,
    pub max: f64,
    pub default: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct NumInputSettings<T> {
    pub min: T,
    pub max: T,
    pub default: T,
    pub step: T,
}

impl Default for FloatInputSettings {
    fn default() -> FloatInputSettings {
        FloatInputSettings {
            min: 0.0,
            max: 100.0,
            default: 50.0,
        }
    }
}
