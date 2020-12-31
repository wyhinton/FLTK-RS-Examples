//Demonstrates Photoshop style adjustment layers

use fltk::{app, app::*, frame::*, valuator::*, window::*, widget::*,  image::RgbImage, table::*, button::*, input::*, group::*};
// pub mod effect_configurations; 
// use effect_configurations::*; 
use image::GenericImageView;
use std::default::Default; 
mod dragable_container; 
use dragable_container::DragableContainer;
use dragable_container::drag_item::DragItem;
use uuid::Uuid; 
use strum::EnumMessage;
use strum_macros::*;
use strum_macros::*; 
use strum_macros::EnumIter;
use strum::IntoEnumIterator; 
// use strum_macros::EnumString; 

use strum_macros::EnumMessage;
// use strum::{EnumMessage};
// use strum::*; 
// use strum_macros::*; 


use std::fmt;
use field_types::FieldType;
pub mod icon_toggle_button;
use icon_toggle_button::IconToggleButton;
#[derive(Debug, Clone, EnumIter, EnumString, EnumMessage)]
// #[derive(Clone, EnumIter, EnumString, EnumMessage)]
pub enum Operation{
    #[strum(message = "Add")]
    Add{
        config: AddConfig
    },
    
    // Subtract{
    //     config: SubtractConfig
    // },
    #[strum(message ="Test")]
    TestOperation{
        config: TestConfig,
    }
}

// pub mod::
#[derive(Clone)]
pub enum Message {
    Test,   
    ProcessLayers,
    SetProcessLayers(Vec<DragItem>),
    AddOperation(Operation),
    RemoveOperation(Uuid),
    DeactivateOperation(Uuid),
    HideOperation(Uuid),
}

pub trait OpConfigExt {
    fn as_array(&self)->Vec<ConfigVal>;
    // fn 
}

//CONFIG VAL
#[derive(Clone)]
pub struct ConfigVal{
    pub val_name: String, 
    pub val: f64, 
    pub inp_type: SliderType,
    pub bounds: (f64, f64),
    pub step: f64,
}

impl ConfigVal{
    pub fn get_value(&self) -> f64{
        self.val
    }
}
impl fmt::Debug for ConfigVal{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value Name: {}, value: {}, Input Type: {:?}, bounds: {},{} step: {} ", self.val_name, self.val, self.inp_type, self.bounds.0, self.bounds.1, self.step)
    }
}
//TEST CONFIG
#[derive(Clone)]//input struct
pub struct TestConfig{
    pub val_a: ConfigVal,
    pub val_b: ConfigVal, 
}

impl Default for TestConfig{
    fn default() -> TestConfig{
        TestConfig {
            val_a: ConfigVal{
                val_name: "My Test Value 1".to_string(),
                val: 0.0,
                inp_type: SliderType::HorizontalFill,
                // inp_type: ProcessInputType::HorizontalFill,
                bounds: (0.0, 255.0),
                step: 1.0,
            },
            val_b: ConfigVal{
                val_name: "My Test Value 1".to_string(),
                val: 255.0,
                inp_type: SliderType::Horizontal,
                // inp_type: ProcessInputType::Horizontal,
                bounds: (0.0, 255.0),
                step: 1.0,
            }
            
        }
    }
}

impl OpConfigExt for TestConfig{
    fn as_array(&self) -> Vec<ConfigVal> {
        [self.val_a.clone(), self.val_b.clone()].to_vec()
    }
}

impl fmt::Debug for TestConfig{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "val_a: {:?}, val_b: {:?}", self.val_a, self.val_b)
    }
}

//ADD CONFIG
#[derive(Debug, Clone)]
pub struct AddConfig{
    pub val_a: ConfigVal,
}

impl Default for AddConfig{
    fn default() -> AddConfig{
        AddConfig{
            val_a: ConfigVal{
                val_name: "Add".to_string(), 
                val: 5.0, 
                inp_type: SliderType::HorizontalFill,
                bounds: (0.0, 250.0),
                step: 1.0,
            }
        }
    }
}

impl OpConfigExt for AddConfig{
    fn as_array(&self) -> Vec<ConfigVal> {
        [self.val_a.clone()].to_vec()
    }
}
//SUBTRACT CONFIG
#[derive(Debug, Clone)]
pub struct SubtractConfig{
    pub a: i32,
    pub b: i32,
}

impl Default for SubtractConfig{
    fn default() -> SubtractConfig{
        SubtractConfig{
            a: 0,
            b: 0,
        }
    }
}


// impl Default for Operation {
//     fn default() -> Self{Operation::Add {config: AddConfig::default()}}
// }


fn center_img(wid: &Widget, mut rgb_img: RgbImage){
    rgb_img.draw(wid.x(), wid.y(), wid.width(), wid.height());
}


fn main() {

    // let mut my_funcs = Vec<fn(i32, i32) -> i32 = vec[add(1, 2), substra]
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    win.make_resizable(true);

    
    // let test_icb = IconToggleButton::new(20,20,24,25, ("icons/expand_less-24px.svg"), ("icons/expand_more-24px.svg"));
    // let img = image::open("imgs/clouds.jpeg").unwrap();
    // let (x,y) = img.dimensions();
    // let rgb = RgbImage::new(&img.to_bytes(), x, y, 3).unwrap();

    let mut tb = Button::new((win.width()/2)-50, 800, 100, 50, "");
    
    
    let mut test_img_frame = Frame::new(20, 20, 300, 300, "");
    let img = image::open("imgs/clouds.jpeg").unwrap();
    let (x, y) = img.dimensions();
    let rgb = RgbImage::new(&img.to_bytes(), x, y, 3).unwrap();
    test_img_frame.set_image(Some(rgb));



    let sc1 = s.clone();
    tb.set_callback(move||{
        sc1.send(Message::ProcessLayers);
    });
    let dcw = 300; 
    let dch = 500;
    let dcx = (win.width()/2) - (dcw/2);
    let dcy = 100;
    let mut cont = DragableContainer::new(dcx, dcy, dcw, dch, s.clone());

    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            use Message::*;
            match msg{
                Test => {
                    // println!("{}", "got test message");
                    app::redraw();
                    cont.redraw();
                }
                ProcessLayers => {
                    let co = cont.get_ops();
                    println!("num operations: {}", co.len());
                    let result = pipe_function_array(0.0, co);
                    println!("result is:  {}", result);
                }
                SetProcessLayers(layers) => {
                    cont.set_ops(layers);    
                }
                AddOperation(new_op)=>{
                    cont.add_op(new_op, s.clone());
                    // dbg!(new_op);
                }
                RemoveOperation(drag_item_to_remove) => {
                    println!("should remove at main: {:?}", drag_item_to_remove);
                    cont.delete_op(drag_item_to_remove, s.clone());
                    
                }
                HideOperation(drag_item_to_hide_id) => {
                    println!("should hide at main: {:?}", drag_item_to_hide_id);
                    cont.hide_op(drag_item_to_hide_id, s.clone());
                    
                }
                DeactivateOperation(drag_item_to_deactivate_id) => {
                    println!("should hide at main: {:?}", drag_item_to_deactivate_id);
                    cont.set_inactive(drag_item_to_deactivate_id, s.clone());
                    
                }
            }
        }
    }
}
fn pipe_function_array(mut initial_value: f64, a: Vec<Operation>) -> f64 {
    a.iter()
        .fold(initial_value, |val, operation| match operation {
            Operation::Add { config } => add(val, config),
            // Operation::Subtract { config } => subtract(val, config),
            Operation::TestOperation { config } => test_op(val, config),
        })
}

fn add(val: f64, addcf: &AddConfig)->f64{
    // println!("{} + {} = {}", val, addcf.b, val + addcf.b);
    let to_add = addcf.val_a.get_value(); 
    return val + to_add; 
}
// fn subtract(val: f64, subcf: &SubtractConfig)->f64{
//     println!("{} - {} = {}", val, subcf.b, val - subcf.b as f64);
//     return (val - subcf.b) as f64
// }
fn test_op(val: f64, testcf: &TestConfig)->f64{
    println!("val a: {}, val b: {}",testcf.val_a.get_value(), testcf.val_b.get_value());
    return val
}
