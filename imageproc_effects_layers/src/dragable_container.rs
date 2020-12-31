use fltk::{app, app::*, image::*, frame::*, group::*, input::*, draw::*, menu::*};
use std::{fs::OpenOptions, ops::{Deref, DerefMut}, println};
// use strum::IntoEnumIterator;
// use strum::EnumMessage; 
// use strum_macros::EnumIter;
// use strum::{IntoEnumIterator, EnumMessage};
use strum::IntoEnumIterator; 
use strum::*; 
use strum::EnumMessage;
// use strum::IntoEnumIterator; 
use super::Message;
use super::Operation;
use super::AddConfig;
use super::SubtractConfig;
use super::TestConfig;
use super::ConfigVal; 
// use crate::effect_configurations::*; 

use uuid::Uuid; 

pub mod drag_item;
use drag_item::DragItem;


#[derive(Clone)]
pub struct DragInfo{
    drag_hover: Option<DragItem>,
    drag_source: Option<DragItem>,
    drag_target: Option<DragItem>,
    target_index: Option<usize>,
    source_index: Option<usize>,
    dragging: bool,
    items: Vec<DragItem>, 
}
    
pub struct DragableContainer {
    mpack: Pack, 
    // pub pack: Scroll, 
    pub pack: Pack, 
    items: Vec<DragItem>,
    menu: MenuButton, 
    // pub frame: Frame,
}

fn check(index: Option<usize>) -> bool{
    match index {
        Some(val)=> true,
        None=> false, 
    }
}

fn add_operation(new_op: Operation, s: fltk::app::Sender<Message>){
    s.clone().send(Message::AddOperation(new_op))
}

impl DragableContainer {
    pub fn new(x: i32, y: i32, w: i32, h: i32, s: fltk::app::Sender<Message>) -> Self {
        let icon_w = 24; 
        let input_w = w-icon_w;
        let scrolltest = Scroll::new(x,y,w,h,"");
        scrolltest.end(); 
        let mut dc = DragableContainer {
            mpack: Pack::new(x,y,w,h,""),
            menu: MenuButton::new(x,y,w,15,"Add Layer"),
            pack: Pack::new(x,y,w,h,""),
            items: vec![],
        };
        
        //Add Operations to the Add Operations Menu
        for op in Operation::iter(){
            let smc = s.clone(); 
            dc.menu.add(
                &op.clone().get_message().unwrap(),
                Shortcut::None,
                MenuFlag::Normal,
                move||{
                    add_operation(op.clone(), smc.clone());
                }
            );
        }


        dc.pack.set_frame(FrameType::BorderFrame);
        dc.pack.set_color(Color::Blue);


        dc.pack.end();
        dc.mpack.end();
        let mut ini_items = vec![];
        

        let mut testitems = vec![
            DragItem::new("0", Operation::Add{config: AddConfig::default()}, s.clone()),
            DragItem::new("1", Operation::Add{config: AddConfig::default()}, s.clone()),
            // DragItem::new("2", Operation::Subtract{config: SubtractConfig{a: 1, b: 1}}, s.clone()),
            DragItem::new("3", Operation::Add{config: AddConfig::default()}, s.clone()),
            DragItem::new("3", Operation::Add{config: AddConfig::default()}, s.clone()),
            DragItem::new("4", Operation::TestOperation{config: TestConfig::default()}, s.clone())
        ];
        
        for w in 0..testitems.len(){
            testitems[w].drag_content.frame.set_label(&w.to_string()); 
            dc.items.push(testitems[w].clone());
            dc.pack.add(&testitems[w].pack);   
            ini_items.push(testitems[w].clone());
        }
        
        let mut di = DragInfo{
            drag_hover: None,
            drag_source: None, 
            drag_target: None, 
            target_index: None,
            source_index: None, 
            dragging: false, 
            items: ini_items,
        };
        
        let mut i_cl = dc.items.clone(); 
        let mut mpc = dc.pack.clone();
        dc.pack.handle2(move|t, ev|{
            dnd_handle(t, ev, i_cl.clone(), s.clone());
            true
        });
        dc
    }
    
    pub fn get_ops(&mut self)->Vec<Operation>{
        let mut ops = vec![];
        for y in &self.items{
            ops.push(y.operation.clone());
        }
        ops
    }
    //add an operation layer
    pub fn add_op(&mut self, new_op: Operation, s: fltk::app::Sender<Message>){
        let lb = self.pack.clone().children().to_string();
        let header = vec![lb, new_op.get_message().unwrap().to_string()].join("-");
        let new_op_layer = DragItem::new(&header, new_op, s.clone());
        self.items.push(new_op_layer.clone());
        self.pack.add(&new_op_layer.clone().pack);
        let sic = self.items.clone();
        self.pack.handle2(move|t, ev|{
            dnd_handle(t, ev, sic.clone(), s.clone());
            true
        });
        app::redraw();
    }

    //delete an operation layer
    pub fn delete_op(&mut self, to_delete_id: Uuid, s: fltk::app::Sender<Message>){
        for item in &self.items{
            if item.id == to_delete_id{
                println!("found one to delete {:?}", item);
                self.pack.remove(&item.pack);
                // self.pack.delete()
                app::delete_widget(item.pack.clone());
                
            }
        }
        self.items.retain(|x| x.id != to_delete_id);
        let sic = self.items.clone();
        //update our dnd functionality with the new items
        self.pack.handle2(move|t, ev|{
            dnd_handle(t, ev, sic.clone(), s.clone());
            true
        });
        app::redraw();
    }

    //hide/fold an operation layer
    pub fn hide_op(&mut self, to_hide_id: Uuid, s: fltk::app::Sender<Message>){
        for mut item in self.items.clone(){
            if item.id == to_hide_id{
                println!("found one to hide {:?}", item);
                // item.hidden ^=true;
                println!("hidden is: {}", item.hide_button.is_checked());
                item.set_hidden(item.hide_button.is_checked());
            }
        }
        app::redraw();
    }

    //deactivate an operation layer
    pub fn set_inactive(&mut self, to_hide_id: Uuid, s: fltk::app::Sender<Message>){
        for mut item in self.items.clone(){
            if item.id == to_hide_id{
                println!("found one to inactive {:?}", item);
                item.set_inactive();
            }
        }
        app::redraw();
    }
    
    pub fn set_ops(&mut self, updated_items: Vec<DragItem>){
        self.items = updated_items.clone();
    }
}

// fn dnd_handle(item_container: &mut Scroll, ev: Event, items: Vec<DragItem>, s: fltk::app::Sender<Message>){
fn dnd_handle(item_container: &mut Pack, ev: Event, items: Vec<DragItem>, s: fltk::app::Sender<Message>){

    let mut di = DragInfo{
        drag_hover: None,
        drag_source: None, 
        drag_target: None, 
        target_index: None,
        source_index: None, 
        dragging: false, 
        items: items.clone(),
    };
    // let mut tc = t.clone();
    let mut dh_items = items.clone();
    item_container.handle2(move|t,ev|{
        // println!("my items len {}", dh_items.len());
    // mpc.handle2(move|t,ev|{
        di = drag_info(t.clone(), ev, s.clone(), di.clone(), dh_items.clone());
        let mut at_push = di.drag_source.clone();
        let mut at_hover = di.drag_hover.clone();
        let mut at_release = di.drag_target.clone();
        let t_index = di.target_index; 
        let s_index= di.source_index; 

        match at_release{
            Some(ref mut target_item) => {
                if t_index != s_index {
                    let ind_test = (t_index, s_index);

                    if !di.dragging{
                            match(t_index, s_index){
                            (Some(a), Some(b)) => {
                                dh_items.swap(a, b);
                                // dc.items = dh_items.clone();
                                if t_index.unwrap() > s_index.unwrap() {
                                    t.insert(&at_push.clone().unwrap().pack, t_index.unwrap() as u32);
                                    t.insert(&at_release.clone().unwrap().pack, s_index.unwrap() as u32);
                                }
                                if t_index.unwrap() < s_index.unwrap() {
                                    t.insert(&at_release.clone().unwrap().pack, s_index.unwrap() as u32);
                                    t.insert(&at_push.clone().unwrap().pack, t_index.unwrap() as u32);
                                }
                                di.target_index = None;
                                di.source_index = None;
                                s.send(Message::SetProcessLayers(dh_items.clone()));
                            }
                            _ => ()
                        }
                    }

                } 
            }
            None => ()
        }
        true
    });
}

// fn drag_info(t: Scroll, ev: Event, s: fltk::app::Sender<Message>, mut drg: DragInfo, mut l_items: Vec<DragItem>,) -> DragInfo{
fn drag_info(t: Pack, ev: Event, s: fltk::app::Sender<Message>, mut drg: DragInfo, mut l_items: Vec<DragItem>,) -> DragInfo{
    let (x, y) = app::event_coords(); 
    match ev {
        Event::Push => {       
            // println!("num items {}", l_items.len());
            for z in 0..l_items.len(){
                if app::event_inside(l_items[z].x(), l_items[z].y(), l_items[z].width(), l_items[z].height()){
                    // dbg!(&l_items[z]);
                    drg.drag_source = Some(l_items[z].clone());
                    drg.source_index = Some(z);
                    drg.target_index = None;
                    l_items[z].set_click();
                } else {
                    l_items[z].reset_style();
                }
            }
        }
        Event::Drag =>{
            drg.dragging = true;
            for z in 0..l_items.len(){
                if app::event_inside(l_items[z].x(), l_items[z].y(), l_items[z].width(), l_items[z].height()){
                    drg.drag_hover = Some(l_items[z].clone());
                    l_items[z].set_hover();
                    drg.target_index = Some(z);
                    // println!("item {}, label {}", z, l_items[z].label());
                } else if z != drg.source_index.unwrap_or(1000){
                    // l_items[z].set_color(Color::Yellow);
                    // l_items[z].redraw
                    l_items[z].reset_style();
                }
            }
           
        }
        Event::Released =>{
            drg.dragging = false; 
            for z in 0..l_items.len(){
                // println!("item {}, label {}", z, l_items[z].label());
                l_items[z].reset_style();
                if app::event_inside(l_items[z].x(), l_items[z].y(), l_items[z].width(), l_items[z].height()){
                    drg.drag_target = Some(l_items[z].clone());

                    if !drg.source_index.unwrap() == z {

                    }
                }
            }

        }
        _=> ()
    }   
    drg
}

impl Deref for DragableContainer {
    // type Target = Scroll;
    type Target = Pack;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl DerefMut for DragableContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}

fn is_inside(c: Box<dyn fltk::WidgetExt>) -> bool {
    let x = c.x(); 
    let y = c.y(); 
    let w = c.width(); 
    let h = c.height();
    let tf = app::event_inside(x,y,w,h);
    tf
}


            // match at_hover{
            //     Some(ref mut hover_item) => {
            //         // hover_item.set_hover();
            //         // hover_item.redraw();
            //     }
            //     None => {
            //         // println!("{}", "got none");
            //     }
            // }    
            //         match at_push {
            //     Some(ref mut source_item) => {  
            //         // println!("has targ: {}, targ index: {}, has source: {}, source index: {}", check(t_index), t_index.unwrap_or(1000), check(s_index), s_index.unwrap_or(100));
            //         s.send(Message::Test);
            //     }
            //     None => {
            //         // println!("{}", "got none");
            //     }
            // };