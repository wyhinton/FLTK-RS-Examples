  
// use fltk::{app, enums::FrameType, frame::Frame, image::SvgImage, prelude::*, window::Window, button::*};
// use hey_listen::{
//     sync::{ParallelDispatchResult, ParallelDispatcher, ParallelListener},
//     RwLock,
// };
// use std::sync::{Arc, Weak};

// #[derive(Clone, Eq, Hash, PartialEq)]
// enum Event {
//     EventVariant,
// }


// struct Listener {}

// #[derive(Default)]
// struct ListenerStruct {
//     number: usize,
// }

// impl ParallelListener<Event> for Arc<RwLock<ListenerStruct>> {
//     fn on_event(&self, _event: &Event) -> Option<ParallelDispatchResult> {
//         println!("{}", self.read().number);
//         self.write().number += 1;

//         // At the end, we have to return an `Option<SyncDispatchResult>` request back to
//         // the dispatcher.
//         // This request gives an instruction back to the dispatcher, here are the variants:
//         // - `ParallelDispatchResult::StopListening` to automatically stop listening.
//         None
//     }
// }

// impl ParallelListener<Event> for Weak<RwLock<ListenerStruct>> {
//     fn on_event(&self, _event: &Event) -> Option<ParallelDispatchResult> {
//         if let Some(strong) = self.upgrade() {
//             println!("hey i'm strong{}", strong.read().number);

//             None
//         } else {
//             Some(ParallelDispatchResult::StopListening)
//         }
//     }
// }

// impl ParallelListener<Event>
//     for Box<dyn Fn(&Event) -> Option<ParallelDispatchResult> + Send + Sync>
// {
//     fn on_event(&self, event: &Event) -> Option<ParallelDispatchResult> {
//         (&self)(&event)
//     }
// }


// impl ParallelListener<Event> for Listener {
//     fn on_event(&self, _event: &Event) -> Option<ParallelDispatchResult> {
//         println!("I'm listening! :)");
//         None
//     }
// }

// struct MyButton{
  
// }

// impl MyButton{
//     pub fn new(x: i32, y: i32, w: i32, h: i32, id: usize, dispatcher: &mut ParallelDispatcher<Event>)->Self{
//         let mut button= Button::new(x,y,w,h,"mybutton");
//         let listener = Arc::new(RwLock::new(ListenerStruct { number: id }));
//         dispatcher.add_listener(Event::EventVariant, listener);
//         button.set_callback(move|widg|{
//             dispatcher.dispatch_event(&Event::EventVariant);
//         });
//         MyButton{
            
//         }
//     }
// }

// fn main() {
//     let app = app::App::default().with_scheme(app::Scheme::Gleam);
//     let mut wind = Window::new(100, 100, 500, 500, "Hello from rust");
//     let mut button= Button::new(0,0,50,20,"mybutton");

//     let mut dispatcher = ParallelDispatcher::<Event>::new(4).expect("Could not construct threadpool");

//     // We add listeners, each assigned with a different number.
//     let listener_1 = Arc::new(RwLock::new(ListenerStruct { number: 0 }));
//     let listener_2 = Arc::new(RwLock::new(ListenerStruct { number: 1 }));

//     // Our closure gets its unique number as well.
//     let listener_3: Box<dyn Fn(&Event) -> Option<ParallelDispatchResult> + Send + Sync> =
//         Box::new(move |_event| {
//             println!("3");

//             // As we did in the `ParallelListener`-implementation:
//             None
//         });

//     // We add some listeners for our only variant.
//     dispatcher.add_listener(Event::EventVariant, Arc::downgrade(&listener_1));
//     dispatcher.add_listener(Event::EventVariant, Arc::clone(&listener_2));
//     dispatcher.add_listener(Event::EventVariant, listener_3);

//     // Let's remember that we gave every listener their own unique number
//     // and added in order of their number.
//     // If we dispatch now, the numbers can be out of order due to
//     // parallel dispatching.
//     // It can help to repeat the dispatch to see the
//     dispatcher.dispatch_event(&Event::EventVariant);
//     button.set_callback(move|widg|{
//       dispatcher.dispatch_event(&Event::EventVariant);
//     });
//     wind.make_resizable(true);
//     wind.end();
//     wind.show();

//     app.run().unwrap();
// }