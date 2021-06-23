  
use fltk::{app, enums::FrameType, frame::Frame, image::SvgImage, prelude::*, window::Window, button::*};
// use hey_listen::{
//     sync::{ParallelDispatchResult, ParallelDispatcher, ParallelListener},
//     RwLock,
// };
// use hey_listen::rc::{Dispatcher, DispatcherRequest, Listener};
use hey_listen::RwLock::{Dispatcher, DispatcherRequest, Listener};
use std::sync::{Arc, Weak};
use event_emitter_rs::EventEmitter;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum EventEnum {
    EventVariantA,
    EventVariantB,
    EventVariantC,
}
struct ListenerStruct {}

// This implements the `Listener`-trait, enabling the struct above (`ListenerStruct`)
// to become a trait-object when starting listening.
impl Listener<EventEnum> for ListenerStruct {
    fn on_event(&self, _event: &EventEnum) -> Option<DispatcherRequest> {
        println!("I'm listening! :)");

        // At the end, we have to return an `Option<DispatcherRequest>` request back to
        // the dispatcher.
        // This request gives an instruction back to the dispatcher, here are the variants:
        //
        // - `DispatcherRequest::StopListening` to automatically
        // stop listening.
        //
        // - `DispatcherRequest::StopPropagation` stops the dispatcher from dispatching
        // to other listeners in this instance of dispatching.
        //
        // - `DispatcherRequest::StopListeningAndPropagation` combines the first and second.
        //
        // - `None`, equals to sending no request to the dispatcher, everything will stay as it is.
        None
    }
}
impl Listener<EventEnum> for Box<dyn Fn(&EventEnum) -> Option<DispatcherRequest>> {
    fn on_event(&self, event: &EventEnum) -> Option<DispatcherRequest> {
        (self)(&event)
    }
}

struct MyButton{
  
}

impl MyButton{
    pub fn new(x: i32, y: i32, w: i32, h: i32, id: usize, dispatcher: &mut ParallelDispatcher<Event>)->Self{
        let mut button= Button::new(x,y,w,h,"mybutton");
        let listener = Arc::new(RwLock::new(ListenerStruct { number: id }));
        dispatcher.add_listener(Event::EventVariant, listener);
        button.set_callback(move|widg|{
            dispatcher.dispatch_event(&Event::EventVariant);
        });
        MyButton{
            
        }
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");
        // Create your listener.
        let listener = ListenerStruct {};

        // Create your dispatcher and define the generic type what the dispatcher
        // shall accept as dispatchable type, it's our declared `EventEnum` in this
        // example.
        let mut dispatcher: Dispatcher<EventEnum> = Dispatcher::new();
    
        // Make your listener start listening.
        dispatcher.add_listener(EventEnum::EventVariantA, listener);
    
        // Dispatches our events to all listeners.
        dispatcher.dispatch_event(&EventEnum::EventVariantA);
        dispatcher.dispatch_event(&EventEnum::EventVariantB);
    
        // If you want to work with a closure, you can do the following:
        let listening_closure: Box<dyn Fn(&EventEnum) -> Option<DispatcherRequest>> =
            Box::new(move |event: &EventEnum| {
                // Be aware, since enum's variants are no types,
                // whenever you want to work with the enum,
                // you need to pattern-match it of if-let-bind in order to find its variant,
                // even if you listen to only one variant.
                let event_name = match *event {
                    EventEnum::EventVariantA => "A".to_string(), // we won't listen to this ...
                    EventEnum::EventVariantB => "B".to_string(), // ... nor to this.
                    EventEnum::EventVariantC => "C, as in closure".to_string(),
                };
    
                println!("Received event-variant: {}!", event_name);
    
                // As we did in the `Listener`-implementation:
                None
            });
    
        dispatcher.add_listener(EventEnum::EventVariantB, listening_closure);
    
        dispatcher.dispatch_event(&EventEnum::EventVariantC);
    
    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}