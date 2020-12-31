use fltk::frame::*;
use fltk::{app::*, image::*, button::*, group::*, input::*, draw::*};
use std::ops::{Deref, DerefMut};
use uuid::Uuid; 
// use syn::*;
use super::Message;

use crate::icon_toggle_button::IconToggleButton; 

#[derive(Clone)]
pub struct DragContent {
    pub pack: Pack, 
    pub frame: Frame,
    // input: Input,
    parent_id: Uuid, 
    del_button: Button,
    visable_button: IconToggleButton,
    // visable_button: Button,
    layer_controls: Pack, 
    shuffle_button: Button, 
    // pub input_pack: Pack, 
    
}

impl DragContent {
    pub fn new(p_id: Uuid, label: &str, s: fltk::app::Sender<Message>) -> Self {
        let icon_w = 24; 
        // let input_w = ;
        let mut np = Pack::new(0,0,50,100,"");
        np.set_frame(FrameType::BorderFrame);
        np.set_color(Color::White);
        np.end();

        let layer_controls = Pack::new(0,0,100,100,"");
        layer_controls.end(); 
        
        let mut dc = DragContent {
            parent_id: p_id, 
            pack: np,
            frame: Frame::new(0,0,50,50,label),
            // input: Input::new(50, 0, 30, 15, "test input"),
            del_button:  Button::new(0, 0, 24, 24, ""),
            visable_button: IconToggleButton::new(0,0,24,24,"icons/visibility-24px.svg", "icons/visibility_off-24px.svg"),
            // visable_button: Button::new(0,0,24,24,""),
            layer_controls: layer_controls, 
            shuffle_button: Button::new(0,0,24,24,""),
            // input_pack: input_pack,
        };
        
        dc.frame.set_frame(FrameType::BorderFrame);
        dc.frame.set_color(Color::Black);
        // dc.frame.set_color(Color::Green);
        dc.pack.add(&dc.frame);

        // dc.input.set_type(InputType::Int);
        let vis_icon = SvgImage::load("icons/visibility-24px.svg").unwrap();
        let shuffle_icon = SvgImage::load("icons/shuffle-24px.svg").unwrap();
        let mut del_icon = SvgImage::load("icons/close-24px.svg").unwrap();

        //remove parent message
        let pidc = dc.parent_id.clone();
        let s_delc = s.clone();
        dc.del_button.set_callback(move||{
            s_delc.send(Message::RemoveOperation(pidc))
        });

        //set parent visible message
        let pidc_2 = dc.parent_id.clone();
        let s_vbc = s.clone();
        dc.visable_button.set_callback(move||{
            s_vbc.send(Message::DeactivateOperation(pidc_2))
        });

        //set icon images
        dc.del_button.set_image(Some(del_icon));
        dc.visable_button.set_image(Some(vis_icon));
        dc.shuffle_button.set_image(Some(shuffle_icon));
     
        dc.layer_controls.add(&dc.shuffle_button);
        dc.layer_controls.add(&dc.del_button);
        dc.layer_controls.add(&dc.visable_button.pack);
        
        dc.pack.add(&dc.layer_controls);
        // dc.pack.add(&dc.input_pack);
        // dc.input_pack.add(&dc.input);

        dc.pack.set_type(PackType::Horizontal);
        dc.layer_controls.set_type(PackType::Vertical);
        dc.layer_controls.auto_layout();
        dc.pack.auto_layout();

            
        dc
    }
}

impl Deref for DragContent {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for DragContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}