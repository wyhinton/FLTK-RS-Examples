use fltk::frame::*;
use fltk::image::*;
use fltk::{app::*, image::*, group::*, input::*, draw::*};
use std::ops::{Deref, DerefMut};
use sublime_fuzzy::best_match;
// use syn::*;
use super::Message;

#[derive(Debug, Eq, Clone, Ord, PartialEq, PartialOrd)]
pub struct Result {
    pub score: isize,
    pub val: String,
}

pub struct SearchBar {
    pub pack: Pack, 
    pub input: Input,
    icon: Frame,
    search_items: Vec<String>,
}

fn fuzzy_search(widg: &mut Input, items: Vec<String>, s: fltk::app::Sender<Message>) {
    println!("sc is {:?}", items);
    let mut mc = vec![];
    if !widg.value().is_empty(){
        //remove "Search..." text when adding input
        widg.draw2(move |b|{
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
            //sort our list of results by the their score values
        mc.sort_by(|a, b| b.score.cmp(&a.score));
        s.send(Message::UpdateSearch(mc.clone()));
    } else {
        widg.draw2(move |b|{
            set_draw_color(Color::Black);
            draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
        });
        s.send(Message::ResetTable)
    }

}

impl SearchBar {
    pub fn new(x: i32, y: i32, w: i32, h: i32, s_items: Vec<String>, s: fltk::app::Sender<Message>) -> Self {
        let icon_w = 24; 
        let input_w = w-icon_w;
        let mut sb = SearchBar {
            pack: Pack::new(x,y,w,h,""),
            icon: Frame::new(x,y,icon_w,h,""),
            input: Input::new(x,y,input_w,h,""),
            search_items: s_items.clone(),            
        };
        //icon frame styling       
        let mut search_icon = SvgImage::load("search-24px.svg").unwrap();
        sb.icon.draw2(move |f|{
            search_icon.draw(f.x(), f.y(), 24, 24);    
        });
        sb.icon.set_frame(FrameType::NoBox); 
        
        //pack styling
        sb.pack.end();
        sb.pack.set_type(PackType::Horizontal);
        sb.pack.set_frame(FrameType::BorderFrame);
        sb.pack.set_color(Color::Black);

        //input styling
        sb.input.draw2(move |b|{
            set_draw_color(Color::Black);
            draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
        });

        //On Enter do fuzzy search
        let sc = s_items.clone();
        let sx = s.clone();
        sb.input.set_callback2(move |b| {
            fuzzy_search(b, sc.clone(), sx.clone())
        });

        sb.input.handle2(move |t, ev| {
            match ev {
                Event::KeyDown =>{
                    println!("{}", "got a key down" );
                    t.do_callback();
                    true
                }
                _=>false 
            }
        });
            
        sb
    }
    pub fn set_items(&mut self, new_items: Vec<String>, s: fltk::app::Sender<Message>){
        println!("setting search bar items:  {:?}", new_items);
        self.search_items = new_items.clone();
        self.input.set_callback2(move |b| {
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