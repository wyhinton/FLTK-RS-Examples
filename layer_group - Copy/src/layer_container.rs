use fltk::{app, app::*, group::Pack, group::*, widget::Widget, button::*, draw, enums::*, frame::*, draw::*, prelude::*, widget::*, window::*};
use crate::{CustomEmmiter, traits::*, LayerDraggable, DraggableCb, DraggableEvent, DraggableInfo, DND_CHANNEL, enums::*, DragObject, cdbg};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use colored::Colorize;

#[derive(Clone)]
pub struct LayerContainer{
// pub struct LayerContainer<D: DraggableExt+Clone>{
    container: Pack,
    pub droppable_id: String,
    event_manager: CustomEmmiter,
    drag_children: HashMap<String, DragObject>,
    // test_children: HashMap<String, D>
}

impl LayerContainer{
// impl <D: DraggableExt+Clone> LayerContainer<D>{
    pub fn new(x: i32, y: i32, w: i32, h: i32, id: &'static str)->Self{
        let parent_id = Rc::from(RefCell::from("__ROOT__".to_string()));
        let index = Rc::from(RefCell::from(0));

        let mut group= Group::new(x,y,w,h,"mygroup");
        let mut container= Pack::new(x,y,w,h,None);
        let emmiter = CustomEmmiter::new();
        emmiter.on(DraggableEvent::DRAGGABLE_PUSH, move|val: DraggableInfo|{
            dbg!("handling drag info");
            dbg!(val);
        });
        container.end();
        group.end();
        group.set_frame(FrameType::FlatBox);
        group.set_color(Color::Blue);
        let parent_id_cl = parent_id.clone();
        let index_cl = index.clone();
        container.handle(move|widg, ev|{
            match ev{
                Event::DndDrag=>{
                    // dbg!("got drag at group");
                true
                }
                Event::DndEnter=>{
                    // dbg!("got enter at group ");
                true
                }
                Event::DndLeave=>{
                    // dbg!("got leave at group");
                true
                }
                Event::DndRelease=>{
                    // dbg!("got release at group");
                true
                }
                _=>false              
              }
        });
        group.handle(move|widg, ev|{
            let drag_info = DraggableInfo::new(parent_id_cl.borrow().to_string(), *index_cl.borrow(), id.to_string(), DraggableType::Container);
          match ev{
            Event::DndDrag=>{
                // dbg!("got dnd drag AT LAYER CONTAINER");
                true
            }
            Event::DndEnter=>{
                // dbg!("got dnd enter AT LAYER CONTAINER");
                // DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(drag_info.clone()));
                // widg.set_color(Color::Red);
                // widg.redraw();
                true
            }
            Event::DndLeave=>{
                true
            }
            Event::DndRelease=>{
                // dbg!("got DND RELEASE AT LAYER CONTAINER!");
                let is_same = app::belowmouse::<Group>().unwrap();
                let under = app::belowmouse::<Group>().unwrap();
                // cdbg!(under.is_same(widg));
                DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(drag_info.clone()));
                DND_CHANNEL.lock().unwrap().0.send(DragAction::DragEnded);
                widg.set_color(Color::Red);
                widg.redraw();
                true
            }

            _=>false              
          }
        });
        let source_id = id;

        // let mut test_children: HashMap<String, D> = HashMap::new();
        // let l = TestDraggable::DragLayer(LayerDraggable::new(100,100,100,100,"TESTESTTEST", None));
        // let test = DraggableEnum::from(l);
        // test_children.insert(String::from("test child 1"), l);
        LayerContainer{
            container: container,
            droppable_id: id.to_string(),
            event_manager: emmiter,
            drag_children: HashMap::new(),
            // test_children: test_children,
        }
    }
}


// impl DroppableExt for LayerContainer<D>{
impl DroppableExt for LayerContainer{
// impl<D: DraggableExt+Clone> DroppableExt for LayerContainer<D>{
    fn container(&mut self) ->Box<dyn GroupExt>{
        self.container.as_group().unwrap()
    }

    fn droppable_id(&self)->String{
        self.droppable_id.clone()
    }
    fn emmiter(&self)->CustomEmmiter{
        self.event_manager.clone()
    }

    fn get_child(&mut self, id: String)->Option<&mut Box<dyn DraggableExt>>{
        self.drag_children.get_mut(&id.clone())
    }

    fn add_child(&mut self, mut drag_object: DragObject) {
        dbg!("adding child!");
        dbg!(self.droppable_id.clone());
        drag_object.set_parent_id(self.droppable_id.clone());
        let widg = drag_object.clone().widg();
        dbg!(drag_object.clone().parent_id());
        self.drag_children.insert(drag_object.clone().draggable_id(), drag_object);
        let mut to_add = unsafe{Widget::from_widget_ptr(widg.as_widget_ptr())};
        self.container.add(&to_add);
        self.container.redraw();
        self.container.parent().unwrap().redraw();
    } 
    fn take_child(&mut self, from: &mut Box<dyn DroppableExt + Send>, child_id: String){
        dbg!("Taking Child!");
        dbg!(child_id.clone());
    }
    fn remove_child(&mut self, child_to_remove_id: String) ->Option<(String, DragObject)> {
        let to_remove_widg = unsafe{Widget::from_widget_ptr(self.drag_children.get(&child_to_remove_id).unwrap().widg().as_widget_ptr())};
        let index_to_remove = self.container.find(&to_remove_widg);
        self.container.parent().unwrap().redraw();
        self.drag_children.remove_entry(&child_to_remove_id.clone())
    }
    fn on_drop(&mut self, drag_source: DraggableInfo) {
    // fn on_drop(&mut self, drag_source: &mut Box<dyn DroppableExt + Send>) {
        dbg!("got dropped onto layer contiainer");
    }
}