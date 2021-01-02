use fltk::frame::*;
use fltk::{app::*, image::*, group::*, input::*, draw::*, valuator::*};
use std::ops::{Deref, DerefMut};
use std::default::Default;
// use syn::*;
use super::Message;
use super::MutexValuator;
use crate::MValExt; 
use super::{MutIntSlider, MutFloatSlider, MutDial};
//every one has a value display
//every one has a display widget and a valuator widget
//every one updates an item value and triggers a rerender
//
#[derive(Clone)]
pub struct BaseMutexValuator<T: MValExt> {
    pub pack: Pack, 
    pub frame: Frame,
    pub custom_valuator: T,
}




// fn find_custom_valuator(cve: MutexValuator)->{
//     match cve{
//         MutexValuator::DialInput => {MutDial{widg: Dial::new(0,0,100,100,"")}},
//         MutexValuator::IntInput =>{MutInSlider{widg: HorSlider}},
//         _=>()
//     }
// }

impl <T> MValExt for BaseMutexValuator<T> where T: MValExt{}

//  impl <T> BaseMutexValuator <T> where T: MValExt{
 impl <T> BaseMutexValuator <T> where T: MValExt{
    pub fn new(x: i32, y: i32, w: i32, h: i32, custom_valuator_enum: MutexValuator, my_sub_struct: T) -> BaseMutexValuator<T>{
        let mut sb = BaseMutexValuator {
            pack: Pack::new(x,y,w,h,""),
            frame: Frame::new(x,y,w,h,""),
            // custom_valuator: Box::new(T),
            custom_valuator: my_sub_struct, 
            // custom_valuator: my_sub_struct, 
        };
        sb 
     }
}
// implg<T
    // pub fn new(x: i32, y: i32, w: i32, h: i32, custom_valuator_enum: MutexValuator) -> BaseMutexValuator<T>{
    //     let icon_w = 24; 
    //     let input_w = w-icon_w;
    //     let to_get = custom_valuator_enum.get_widg(); 
    //     let mut sb = BaseMutexValuator<T> {
    //         pack: Pack::new(x,y,w,h,""),
    //         frame: Frame::new(x,y,w,h,""),
    //         // custom_valuator: Box::new(T),
    //         custom_valuator: Box::new(to_get),
    //     };
    //     // ColorInput,
    //     // IntInput,
    //     // FloatInput,
    //     // DialInput, 
    //     // DimensionsInput,
    //     match custom_valuator_enum{
    //         MutexValuator::IntInput=>{sb.custom_valuator = MutIntSlider{widg: HorSlider::new(0,0,100,100,"")}}
    //         _=>{}
    //     }
        
    //     sb<T>
    // }
    // pub fn new()->{

    // }
    // pub fn new_int() -> MutIntSlider{
    //     MutIntSlider{widg: HorSlider::new(0,0,100,100,"")}
    // }
// impl <T> Default for BaseMutexValuator<T> where T: MValExt {
//     fn default() -> BaseMutexValuator<T> where T: MValExt{
//         BaseMutexValuator{
//             pack: Pack::new(0,0,100,50,""),
//             frame: Frame::new(0,0,100,50,""),
//             custom_valuator: MutIntSlider{widg: HorSlider::new(0,0,100,50,"")}
//         }
//     }
// }
