use fltk::{app, button::*, frame::*, app::*, image::*, group::*, valuator::*, input::*, draw::*};
use std::ops::{Deref, DerefMut};
use std::fmt;
use std::default::Default; 
// use strum_macros::EnumIter;
// use strum::IntoEnumIterator;
use strum_macros::EnumMessage;
// use strum::EnumMessage;
use strum::IntoEnumIterator;
use strum::EnumMessage;
use super::Message;


use uuid::Uuid; 
use super::AddConfig;
use super::SubtractConfig;
use super::TestConfig; 
use super::ConfigVal;
use super::Operation;
use crate::OpConfigExt; 

mod drag_content;
use drag_content::DragContent;
// use crate::effect_configurations::*; 
use crate::{icon_toggle_button::IconToggleButton}; 
// use::icon_toggle_button::IconToggleButton; 

#[derive(Clone)]
pub struct DragItem {
    pub operation: Operation,
    pub drag_content: DragContent,
    pub pack: Pack,
    header_pack: Pack,
    body_pack: Pack, 
    pub input_pack: Pack, 
    pub id: Uuid, 
    pub hidden: bool,
    pub active: bool, 
    pub hide_button: IconToggleButton,
    pub input_configs: Vec<ConfigVal>,
    // pub 
}

impl fmt::Debug for DragItem{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi: {}, active: {}, hidden: {}, configs: {:?} ", self.id, self.active, self.hidden, self.input_configs)
    }
}



impl DragItem{
    pub fn new(label: &str, op: Operation, s: fltk::app::Sender<Message>) -> Self{

        //match operation to a config?
        
        //unique id
        let id = Uuid::new_v4();

        //containerpack
        let mut container_pack = Pack::new(0,0,300,200,"");
        container_pack.set_frame(FrameType::BorderFrame);
        container_pack.end(); 

        //headerpack
        let mut header_pack = Pack::new(0,0,300,25,"");
        header_pack.end(); 
        header_pack.set_type(PackType::Horizontal);
        header_pack.make_resizable(true);

        //input pack
        let mut input_pack = Pack::new(0,0,300,100,"");
        input_pack.end(); 

        //body pack
        let mut body_pack = Pack::new(0,0,300,100,"");
        body_pack.end(); 
        
        let s_hbc = s.clone();
        let id_hbc = id.clone();
        let mut hide_button = IconToggleButton::new(0,0,24,24, "icons/expand_less-24px.svg", "icons/expand_more-24px.svg");
        hide_button.set_callback(move||{
            s_hbc.send(Message::HideOperation(id_hbc));
        });


        let mut di = DragItem{
            pack: container_pack,
            header_pack: header_pack,
            body_pack: body_pack,
            id: id,
            drag_content: DragContent::new(id, label, s.clone()),
            operation: op.clone(),
            input_pack: input_pack.clone(), 
            hidden: false,
            active: true, 
            hide_button: hide_button, 
            input_configs: vec![], 
        };

        // let mut def = op

        // let opc 
        let mut input_clone = input_pack.clone();
        let testdef: TestConfig = TestConfig::default();
        let si_clone = s.clone();
        let si_idc= id.clone(); 
        match op {
            Operation::TestOperation{..}=>{
                let test_def = TestConfig::default();
                let op_configs = test_def.inputs;
                di.input_configs = op_configs.clone(); 
                inputs_from_config(op_configs, input_clone, si_clone, si_idc);
            }

            Operation::Add{..}=>{
                let def_add_opp = AddConfig::default();
                let op_configs = def_add_opp.inputs;
                di.input_configs = op_configs.clone(); 
                inputs_from_config(op_configs, input_clone, si_clone, si_idc);
            }
            _=>println!("{}", "not test config")
        }

        
        //add header and body to container pack
        di.pack.set_frame(FrameType::BorderFrame);
        di.pack.set_color(Color::Green);
        di.pack.add(&di.header_pack);
        di.pack.add(&di.body_pack);

        //add the main content to the body
        di.body_pack.set_type(PackType::Horizontal);
        di.body_pack.add(&di.drag_content.pack);


        let test_button = Button::new(0,0,300-25,25, &op.get_message().unwrap());
        // let test_button = Button::new(0,0,300-25,25, &label);
        di.header_pack.add(&di.hide_button.pack);
        // di.header_pack.add(&hide_button.pack);
        di.header_pack.add(&test_button);

        //add the inputs to the body
        di.body_pack.add(&di.input_pack);
        di.body_pack.make_resizable(true);
        let ti1 = Input::new(50, 0, 200, 15, "");


        di.input_pack.add(&ti1);
        di.input_pack.auto_layout();
        di.input_pack.make_resizable(true);

        // di.pack.auto_layout();
        di.pack.make_resizable(true);
        di.pack.end();
        di
    }
    //click style
    pub fn set_click(&mut self){
        println!("{}", "set push style" );
        self.drag_content.frame.set_color(Color::Blue);
        self.drag_content.frame.redraw();
        self.drag_content.redraw();
    }
    //drag hover style
    pub fn set_hover(&mut self){
        self.drag_content.frame.draw2(move |b|{
            // draw_rect_fill(b.x(), b.y(), b.width(), 2, Color::Black);
            set_draw_color(Color::Yellow);
            draw_rect(b.x(), b.y(), b.width(), b.height());
        });
        self.drag_content.frame.redraw();
        self.drag_content.redraw();
    }
    //default style
    pub fn reset_style(&mut self){
        self.drag_content.frame.draw2(move |b|{
            // draw_rect_fill(b.x(), b.y(), b.width(), 2, Color::Red);
            // set_draw_color(Color::Red);
            draw_rect(b.x(), b.y(), b.width(), b.height());
        });
        self.drag_content.frame.set_color(Color::Red);
        self.drag_content.frame.set_frame(FrameType::FlatBox);
        self.drag_content.frame.redraw();
        app::redraw();
        self.drag_content.redraw();
    }
    
    pub fn set_item_color(&mut self, col: Color){
        self.drag_content.frame.set_color(col);
        self.drag_content.frame.redraw();
    }
    //inactive style
    pub fn set_inactive(&mut self){
        println!("{}", "should set to an inactive style");
        // self.drag_content.del
        // self.drag_content()
        // self.drag_content.frame.set_color(col);
        // self.drag_content.frame.redraw();
    }
    
    pub fn set_hidden(&mut self, is_checked: bool){
        println!("{}", "should set to a hidden/folded style");
        if is_checked{
            self.body_pack.show()
        } else {
            self.body_pack.hide();
        }
        app::redraw();
        // self.drag_content.del
        // self.drag_content.frame.set_color(col);
        // self.drag_content.frame.redraw();
    }

    // pub fn update
}

impl Deref for DragItem {
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for DragItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}

//make an input widget from the ConfigVal settings
fn make_input(cf: ConfigVal)-> Slider{
    let mut new_slider = Slider::new(0,0,50, 50, "");
    new_slider.set_bounds(cf.bounds.0, cf.bounds.1);
    new_slider.set_value(cf.val); 
    new_slider.set_label(&cf.val_name);
    new_slider.set_type(cf.inp_type);
    new_slider
}

//iterate through the inputs in a config generating input widgets and add them to a group
fn inputs_from_config(cf: Vec<ConfigVal>, parent: Pack, s: fltk::app::Sender<Message>, drag_item_id: Uuid){
    // let t_tuple = (0,1,2,3,4,5); 
    let mut x = 0; 
    // let sc = s.clone(); 
    for xyz in cf.clone(){
        // println!("{}", t_tuple.x); 
        let sc = s.clone(); 
        println!("name of value is {}, id is: {}", xyz.val_name, xyz.id); 
        let mut new_input = make_input(xyz.clone());
        new_input.set_callback2(move|widg|{
            // s.clone().send(Message::UpdateInputValue(xyz.clone().id))
            sc.clone().send(Message::UpdateInputValue(drag_item_id.clone(), xyz.clone().id, widg.value()))
            
        });
        parent.clone().add(&new_input);
        x = x + 1; 
    }
}