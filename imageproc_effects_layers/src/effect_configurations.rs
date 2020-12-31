// use fltk::{frame::*, valuator::*};
// use image::GenericImageView;
// use std::{default::Default, ops::Add}; 
// use uuid::Uuid; 
// use strum::{IntoEnumIterator, EnumMessage};
// use strum_macros::EnumMessage;
// use std::fmt;
// use strum::*;
// use strum_macros::*; 


// pub trait OpConfigExt {
//     fn as_array(&self)->Vec<ConfigVal>;
// }

// //CONFIG VAL
// #[derive(Clone)]
// pub struct ConfigVal{
//     pub val_name: String, 
//     pub val: f64, 
//     pub inp_type: SliderType,
//     pub bounds: (f64, f64),
//     pub step: f64,
// }

// impl ConfigVal{
//     pub fn get_value(&self) -> f64{
//         self.val
//     }
// }
// impl fmt::Debug for ConfigVal{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Value Name: {}, value: {}, Input Type: {:?}, bounds: {},{} step: {} ", self.val_name, self.val, self.inp_type, self.bounds.0, self.bounds.1, self.step)
//     }
// }
// //TEST CONFIG
// #[derive(Clone)]//input struct
// pub struct TestConfig{
//     pub val_a: ConfigVal,
//     pub val_b: ConfigVal, 
// }

// impl Default for TestConfig{
//     fn default() -> TestConfig{
//         TestConfig {
//             val_a: ConfigVal{
//                 val_name: "My Test Value 1".to_string(),
//                 val: 0.0,
//                 inp_type: SliderType::HorizontalFill,
//                 // inp_type: ProcessInputType::HorizontalFill,
//                 bounds: (0.0, 255.0),
//                 step: 1.0,
//             },
//             val_b: ConfigVal{
//                 val_name: "My Test Value 1".to_string(),
//                 val: 255.0,
//                 inp_type: SliderType::Horizontal,
//                 // inp_type: ProcessInputType::Horizontal,
//                 bounds: (0.0, 255.0),
//                 step: 1.0,
//             }
            
//         }
//     }
// }

// impl OpConfigExt for TestConfig{
//     fn as_array(&self) -> Vec<ConfigVal> {
//         [self.val_a.clone(), self.val_b.clone()].to_vec()
//     }
// }

// impl fmt::Debug for TestConfig{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "val_a: {:?}, val_b: {:?}", self.val_a, self.val_b)
//     }
// }

// //ADD CONFIG
// #[derive(Debug, Clone)]
// pub struct AddConfig{
//     pub val_a: ConfigVal,
// }

// impl Default for AddConfig{
//     fn default() -> AddConfig{
//         AddConfig{
//             val_a: ConfigVal{
//                 val_name: "Add".to_string(), 
//                 val: 5.0, 
//                 inp_type: SliderType::HorizontalFill,
//                 bounds: (0.0, 250.0),
//                 step: 1.0,
//             }
//         }
//     }
// }

// impl OpConfigExt for AddConfig{
//     fn as_array(&self) -> Vec<ConfigVal> {
//         [self.val_a.clone()].to_vec()
//     }
// }
// //SUBTRACT CONFIG
// #[derive(Debug, Clone)]
// pub struct SubtractConfig{
//     pub a: i32,
//     pub b: i32,
// }

// impl Default for SubtractConfig{
//     fn default() -> SubtractConfig{
//         SubtractConfig{
//             a: 0,
//             b: 0,
//         }
//     }
// }

// //OPERATION
// #[derive(Debug, Clone, EnumIter::, EnumString, EnumMessage)]
// pub enum Operation{
//     #[strum(message = "Add")]
//     Add{
//         config: AddConfig
//     },
//     #[strum(message ="Subtract")]
//     // Subtract{
//     //     config: SubtractConfig
//     // },
//     #[strum(message ="Test")]
//     TestOperation{
//         config: TestConfig,
//     }
// }
// impl Default for Operation {
//     fn default() -> Self{Operation::Add {config: AddConfig::default()}}
// }
// // impl strum::EnumMessage for Operation{
// //     strum::
// // }




