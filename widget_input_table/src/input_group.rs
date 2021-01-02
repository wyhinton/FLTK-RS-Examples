use fltk::frame::*;
use fltk::{app::*, button::*, image::*, group::*, input::*, draw::*, table::*, valuator::*};
use std::ops::{Deref, DerefMut};

use super::InputConfig;
// use syn::*;
use super::Message;
pub mod base_mutex_valuator;
use base_mutex_valuator::BaseMutexValuator;
use super::MutexValuator;
use base_mutex_valuator::*; 
use crate::MValExt; 

#[derive(Clone)]
pub struct InputGroup {
    pub pack: Pack, 
    pub inputs_container: Pack, 
    // pub table: Table,
}

impl InputGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32, input_configs: Vec<InputConfig>, s: fltk::app::Sender<Message> ) -> Self {
        
        let mut it = InputGroup{
            pack: Pack::new(x,y,w,h,""),
            inputs_container: Pack::new(x,y,w,h,""),
            // table: Table::new(0,0,w,h,""),
        }; 
        it.inputs_container.end();
        it.pack.end();
        it.pack.set_frame(FrameType::BorderFrame);
        it.pack.set_color(Color::Black);
        it.pack.make_resizable(true);
    
        let igcl = input_configs.clone(); 
        let mut z = 0;
        //Crate new MutInputs for each Config Val, add that MutInput Val to a Pack, then add that Pack a Master Pack to create an input panel
        for (i, config) in input_configs.iter().enumerate(){
            BaseMutexValuator::new(x,y,w,h, config.inp_type);
            //make a new custom input
            // let new_mut_inp = find_custom_valuator(config.inp_type);
            // new_mut_inp
            // input_container
            // my_new_pack.set_spacing(10);
            // let mutnewbutt = Button::new(x,y,20,my_new_pack.width()/2, &igcl.labels[z]);
            // let mut ns = HorNiceSlider::new(x,y,w,20,"");
            // my_new_pack.set_type(PackType::Horizontal);
            // my_new_pack.end();
            // itpc.add(&my_new_pack);            
            // z = z+1; 
        }
        
        it
    }
}

impl Deref for InputGroup {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for InputGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}

// fn find_custom_valuator(cve: MutexValuator)-><T>{
//     match cve{
//         MutexValuator::DialInput => {MutDial{widg: Dial::new(0,0,100,100,"")}},
//         MutexValuator::IntInput =>{MutInSlider{widg: HorSlider}},
//         _=>()
//     }
// }

