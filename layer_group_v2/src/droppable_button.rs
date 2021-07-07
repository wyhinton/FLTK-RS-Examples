use fltk::{prelude::*, *, *, group::*, enums::*, button::*, image::*, widget::*};
use std::ops::{Deref, DerefMut};
use crate::{traits::*, CustomEmmiter, DraggableInfo, DragAction, DROPPABLES, ROOT_ID, DraggableType, DND_CHANNEL};

#[derive(Clone)]
pub struct DroppableButton {
    container: group::Group,
    droppable_id: String,
    emmiter: CustomEmmiter,
}
    // let mut trash_can = Frame::new(100,100,50,50, "trash");
    // trash_can.set_frame(FrameType::RFlatBox);
    // trash_can.set_color(Color::Blue);
    // trash_can.draw({
    //     let mut img = SvgImage::load(std::path::Path::new("assets/delete_black_24dp.svg")).unwrap();
    //     move |f| {
    //         img.scale(f.w(), f.h(), true, true);
    //         img.draw(
    //             f.x() + f.w() / 2 - img.width() / 2,
    //             f.y() + f.h() / 2 - img.height() / 2,
    //             img.width(),
    //             img.height(),
    //         );
    //     }
    // });
impl DroppableButton {
    pub fn new(w: i32, h: i32, id: &str, icon_path: &str) -> DroppableButton {
        let emmiter = CustomEmmiter::new();
    
        let mut container = Group::new(0, 0, w, h, None);
        container.set_frame(FrameType::RFlatBox);
        container.set_color(Color::from_u32(0x01579b));
        container.set_align(Align::Center);
        let mut btn = Button::new(container.x(), container.y() + 35, w, 15, None);
        btn.set_frame(FrameType::RFlatBox);
        btn.set_color(Color::from_u32(0xf49da9));
        btn.draw({
            let mut img = SvgImage::load(std::path::Path::new(icon_path)).unwrap();
            move |f| {
                img.scale(f.w(), f.h(), true, true);
                img.draw(
                    f.x() + f.w() / 2 - img.width() / 2,
                    f.y() + f.h() / 2 - img.height() / 2,
                    img.width(),
                    img.height(),
                );
            }
        });
        container.end();
        let drag_info = DraggableInfo::new(String::from(ROOT_ID),  String::from(id), DraggableType::Button);
        btn.handle(move|widg, ev|{
            match ev{
                Event::DndDrag=>{
                true
                }
                Event::DndEnter=>{
                true
                }
                Event::DndLeave=>{
                true
                }
                Event::DndRelease=>{
                    dbg!("got DND RELEASE AT BUTTON!");
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::SetDragTarget(drag_info.clone()));
                    DND_CHANNEL.lock().unwrap().0.send(DragAction::DragEnded);
                    true
                }
                _=>false              
              }
        });
        
        DroppableButton { 
            container: container,
            droppable_id: String::from(id),
            emmiter: emmiter,
        }
    }
}

impl Deref for DroppableButton {
    type Target = group::Group;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
impl DerefMut for DroppableButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

impl DroppableExt for DroppableButton {
    fn droppable_id(&self) ->String {
        self.droppable_id.clone()
    }
    // fn emmiter(&self) ->crate::custom_emmiter::CustomEmmiter {
    //     self.emmiter.clone()
    // }
    // fn get_child(&mut self, child_id: String) ->Option<&mut Box<dyn DraggableExt>> {
    //     return None
    // }
    // fn add_child(&mut self, drag_object: crate::DragObject) {
    //     //do nothing
    // }
    // fn container(&mut self) ->Box<dyn GroupExt> {
    //     self.container.as_group().unwrap()
    // }
    // fn take_child(&mut self, from: &mut Box<dyn DroppableExt + Send>, child_id: String) {
    //     //do nothing
    // }
    // fn remove_child(&mut self, child_to_remove_id: String) ->Option<(String, crate::DragObject)> {
    //     None
    // }
    // fn on_drop(&mut self, drag_info: DraggableInfo) {
    //     //allows us to acess to Mutex twice
    //     std::thread::spawn(move ||{
    //         let mut map = DROPPABLES.lock().unwrap();
    //         let mut source_container = map.get_mut(&drag_info.parent).unwrap();
    //         let mut child = source_container.get_child(drag_info.draggable_id.clone()).unwrap();
    //         let widg = unsafe{ Widget::from_widget_ptr(child.widg().as_widget_ptr())};
    //         WidgetBase::delete(widg);
    //         source_container.remove_child(drag_info.draggable_id.clone())
            
    //     });
    // }
}

impl std::fmt::Debug for DroppableButton{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Droppable Button: {}", self.droppable_id)
    }
}