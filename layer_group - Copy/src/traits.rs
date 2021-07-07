use crate::{LayerContainer, DroppableButton, CustomEmmiter, DraggableCb, DragObject, DraggableInfo, LayerDraggable};
use fltk::{app, app::*, group::Pack, group::*, widget::Widget, button::*, draw, enums::*, frame::*, draw::*, prelude::*, widget::*, window::*};
use enum_dispatch::enum_dispatch;
//dynclone for easy cloneing of trait objects
use dyn_clone::DynClone;


pub trait DraggableExt: DynClone+Send{
    fn is_dragging(&self)->bool;
    fn draggable_id(&self)->String;
    // fn on_drag_end(&self, cb: Box<dyn FnMut(&mut Self)>) where Self:Sized;
    fn set_parent_id(&mut self, new_parent_id: String);
    fn parent_id(&self)->String;
    // fn with_parent(self, p: &mut Box<dyn DroppableExt>)->Self
    // where Self: Sized;
    fn widg(&self)->Box<dyn WidgetExt>;
    // fn set_push_fn(&mut self, cb: DraggableCb);
}

// impl DraggableExt for Box<dyn DraggableExt>{
    
// }
// trait cannot be made into an object becuase method references Self type rust
#[enum_dispatch(TestDroppable)]
pub trait DroppableExt: DynClone+Send{
    fn droppable_id(&self)->String; 
    fn emmiter(&self)->CustomEmmiter;
    fn get_child(&mut self, child_id: String)->Option<&mut Box<dyn DraggableExt>>;
    fn add_child(&mut self, drag_object: DragObject);
    // fn add_child(&mut self, drag_object: DragObject);
    fn container(&mut self)->Box<dyn GroupExt>; 
    fn take_child(&mut self, from: &mut Box<dyn DroppableExt + Send>, child_id: String);
    fn remove_child(&mut self, child_to_remove_id: String)->Option<(String, DragObject)>;
    fn on_drop(&mut self, drag_source_id: DraggableInfo);
    // fn delete_child(&mut self, child_id: String);
    // fn on_drop(&mut self, drag_source: &mut Box<dyn DroppableExt + Send>);
    // fn on_drop(&mut self, drag_source: &mut Box<dyn DroppableExt + Send>);
}

dyn_clone::clone_trait_object!(DraggableExt);
dyn_clone::clone_trait_object!(DroppableExt);


#[derive(Clone)]
pub enum TestDraggable{
    DragLayer(LayerDraggable),
}

// impl DraggableExt for TestDraggable{
//     fn is_dragging(&self) ->bool {
//         dbg!("do nothign");
//         false
//     }
//     fn draggable_id(&self) ->String {
//         match self {
//             TestDraggable::DragLayer(layer) => {
//                 layer.draggable_id()
//             }
//         }
//     }
//     fn widg(&self) ->Box<dyn WidgetExt> {
//         match self {
//             TestDraggable::DragLayer(layer) => {
//                   Box::new(layer.frame.clone())
//             }
//         }
//     }
//     fn set_parent_id(&mut self, new_parent_id: String) {
//         match self{
//             TestDraggable::DragLayer(layer) => {
//                 // Box::new(layer.frame.clone())
//                 *layer.parent_id.lock().unwrap() = new_parent_id;
//           }  
//         }
//     }



//     fn parent_id(&self)->String{
//         match self{
//             TestDraggable::DragLayer(layer) => {
//                 *layer.parent_id.lock().unwrap()
//           }  
//         }
//     }

// }
#[enum_dispatch]
#[derive(Clone)]
pub enum TestDroppable{
    LayerContainer(LayerContainer),
    Button(DroppableButton)
}

// impl DroppableExt for TestDroppable<D>{
//     match self{
//         LayerContainer(layer_container)=>{
//             layer_con
//         }
//         Button(droppable_button)=>{

//         }
//     }
//     // fn container(&mut self) ->Box<dyn GroupExt>{
//     //     self.container.as_group().unwrap()
//     // }

//     fn droppable_id(&self)->String{
//         match self{
//             LayerContainer(layer_container)=>{
//                 layer_container.droppable_id
//             }
//             Button(droppable_button)=>{
//                 droppable_button.droppable_id
//             }
//         }
//     }
//     fn emmiter(&self)->CustomEmmiter{
//         self.event_manager.clone()
//     }

//     fn get_child(&mut self, id: String)->Option<&mut Box<dyn DraggableExt>>{
//         self.drag_children.get_mut(&id.clone())
//     }

//     fn add_child(&mut self, mut drag_object: DragObject) {
//         dbg!("adding child!");
//         dbg!(self.droppable_id.clone());
//         drag_object.set_parent_id(self.droppable_id.clone());
//         let widg = drag_object.clone().widg();
//         dbg!(drag_object.clone().parent_id());
//         self.drag_children.insert(drag_object.clone().draggable_id(), drag_object);
//         let mut to_add = unsafe{Widget::from_widget_ptr(widg.as_widget_ptr())};
//         self.container.add(&to_add);
//         self.container.redraw();
//         self.container.parent().unwrap().redraw();
//     } 
//     fn take_child(&mut self, from: &mut Box<dyn DroppableExt + Send>, child_id: String){
//         dbg!("Taking Child!");
//         dbg!(child_id.clone());
//     }
//     fn remove_child(&mut self, child_to_remove_id: String) ->Option<(String, DragObject)> {
//         let to_remove_widg = unsafe{Widget::from_widget_ptr(self.drag_children.get(&child_to_remove_id).unwrap().widg().as_widget_ptr())};
//         let index_to_remove = self.container.find(&to_remove_widg);
//         self.container.parent().unwrap().redraw();
//         self.drag_children.remove_entry(&child_to_remove_id.clone())
//     }
//     fn on_drop(&mut self, drag_source: DraggableInfo) {
//     // fn on_drop(&mut self, drag_source: &mut Box<dyn DroppableExt + Send>) {
//         dbg!("got dropped onto layer contiainer");
//     }
// }