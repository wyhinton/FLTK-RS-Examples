/// Simple demonstration of using app::belowmouse() to target nested widgets with DragnDrop Events
use fltk::{
    app, app::*, button::*, enums::*, frame::*, group::*, prelude::*, window::*, widget::*,
};

#[derive(Clone)]
pub enum Message {
    SetDragTarget(String),
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::default();
    let mut win = Window::new(200, 200, 500, 300, "Nested Drag Targets");
    
    //draggable button
    let button_container= Pack::new(50,100,200,200,None);
    let mut drag_button = Button::new(50, 100, 200, 60, "Drag Me!");
    drag_button.set_color(Color::Green);
    let (s, r) = app::channel::<Message>();
    drag_button.handle(move |_, ev| {
        match ev {
            Event::Drag => {
                app::dnd(); // from app module
                true
            }
            _ => false,
        }
    });

    //for displaying our SetDragTarget(String) messages
    let mut display_frame = Frame::new(100, 150, 150, 20, "No Drag Target Yet");
    display_frame.set_frame(FrameType::FlatBox);
    display_frame.set_color(Color::Yellow);
    button_container.end();

    let mut parent_group = Group::new(275, 50, 200, 200, "Parent Group");
    parent_group.set_align(Align::Inside|Align::Top);
    parent_group.handle({
        //for proper Dnd event handling, we need to return true for DndDrag/DndEnter/DndLeave/DndRelease
        let s = s.clone();
        move |widg, ev| match ev {
            Event::DndDrag => true,
            Event::DndEnter => {
                //highlight when entered
                widg.set_color(Color::Yellow);
                widg.redraw();
                true
            },
            Event::DndLeave => {
                //reset color on leave
                widg.set_color(Color::Light1);
                widg.redraw();
                true
            },
            Event::DndRelease => {
                //reset color on leave + send the widgtets label as a message if it's under the Cursor
                widg.set_color(Color::Light1);
                widg.redraw();
                let w = app::belowmouse::<Group>().unwrap();//allows us to distinguish between the parent group and sub group
                s.send(Message::SetDragTarget(String::from(w.label())));
                true
            }
            _ => false,
        }
    });
    parent_group.set_frame(FrameType::FlatBox);
    parent_group.set_color(Color::Light1);

    let mut sub_group= Group::new(parent_group.x()+50, parent_group.y()+50, 100,100, "Sub Group");
    sub_group.set_align(Align::Inside|Align::Center);
    sub_group.set_frame(FrameType::FlatBox);
    sub_group.set_color(Color::Dark3);
    sub_group.handle({
        let s = s.clone();
        move |widg, ev| match ev {
            Event::Unfocus=>{
                widg.set_color(Color::Dark3);
                widg.redraw();
                true
            }
            Event::DndDrag => true,
            Event::DndEnter => {
                widg.set_color(Color::Yellow);
                widg.redraw();
                widg.parent().unwrap().set_color(Color::Light1);
                widg.parent().unwrap().redraw();
                true
            },
            Event::DndLeave => {
                widg.set_color(Color::Dark3);
                widg.parent().unwrap().set_color(Color::Yellow);
                widg.parent().unwrap().redraw();
                true
            },
            Event::DndRelease => {
                widg.set_color(Color::Dark3);
                widg.redraw();
                let w = app::belowmouse::<Group>().unwrap();
                s.send(Message::SetDragTarget(String::from(w.label())));
                true
            }
            _ => false,
        }
    });

    parent_group.end();
    win.end();
    win.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::SetDragTarget(val) => {
                    //update the display frame when we recieve our SetDragTargetMessage
                    display_frame.set_label(&format!("Target:{}", val));
                    display_frame.redraw();
                    println!("setting target to {}", val.clone());
                }
            }
        }
    }
    Ok(())
}