use fltk::{ frame::*, prelude::*, image::*, group::*, input::*, draw::*, enums::*};
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use sublime_fuzzy::best_match;
use super::Message;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, Clone, Ord, PartialEq, PartialOrd)]
pub struct Result {
    pub score: isize,
    pub val: String,
}

pub struct SearchBar {
    pub pack: Pack, 
    pub input: Input,
    search_items: Vec<String>,
    pub value: Rc<RefCell<String>>,
}

fn fuzzy_search(widg: &mut Input, items: Vec<String>, s: fltk::app::Sender<Message>) {
    let mut mc = vec![];
    if !widg.value().is_empty(){
        //remove "Search..." text when adding input
        widg.draw(move |b|{
            set_draw_color(Color::Black);
            draw_text2("", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
        });

        for x in 0..items.len(){
            let m = best_match(&widg.value().to_string(), &items[x]);
            //best_match returns an Option<Match> so we need to check for Some and None values
            match m {
                Some(val) => {
                    let res = Result{
                        score: best_match(&widg.value().to_string(), &items[x]).unwrap().score(),
                        val: items[x].clone(),
                    };
                    mc.push(res);

                }
                None => {
                    let res = Result{
                        score: 0,
                        val: items[x].clone(),
                    };
                    mc.push(res);
                }
            }
        }  
        
        mc.sort_by(|a, b| b.score.cmp(&a.score));
        s.send(Message::UpdateSearch(mc.clone()));
    } else {
        widg.draw(move |b|{
            set_draw_color(Color::Black);
            draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
        });
        s.send(Message::ResetTable)
    }

}

impl SearchBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32, s_items: Vec<String>) -> Self {
        let icon_w = 24; 
        let input_w = w-icon_w;
        let val = Rc::from(RefCell::from("".to_string()));
        let mut pack = Pack::new(x,y,w,h, None);
        
        let mut icon = Frame::new(x,y,icon_w,h, None);
        let mut search_icon = SvgImage::load("search-24px.svg").unwrap();
        icon.draw(move |f|{
            search_icon.draw(f.x(), f.y(), 24, 24);    
        });
        icon.set_frame(FrameType::NoBox); 
        let mut input = Input::new(x,y,input_w,h, None);
        input.draw(move |b|{
            set_draw_color(Color::Black);
            draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
        });
        let val_cl = val.clone();
        input.handle(move |t, ev| {
            match ev {
                Event::KeyDown =>{
                    println!("{}", "got a key down" );
                    dbg!(t.value());
                    *val_cl.borrow_mut() = t.value();
                    true
                }
                _=>false 
            }
        });

        pack.end(); 
        pack.set_type(PackType::Horizontal);
        pack.set_frame(FrameType::BorderFrame);
        pack.set_color(Color::Black);

        let mut sb = SearchBar {
            pack: Pack::new(x,y,w,h,""),
            input: Input::new(x,y,input_w,h,""),
            search_items: s_items.clone(),       
            value:  val,
        };

        sb.pack.end();
        sb
    }
    pub fn set_items(&mut self, new_items: Vec<String>, s: fltk::app::Sender<Message>){
        println!("setting search bar items:  {:?}", new_items);
        self.search_items = new_items.clone();
        self.input.set_callback(move |b| {
            fuzzy_search(b, new_items.clone(), s.clone())
        });
    }
    
}

impl Deref for SearchBar {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl DerefMut for SearchBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input
    }
}