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
        //load search icon        
        let mut search_icon = SvgImage::load("search-24px.svg").unwrap();

        //draw the search icon
        sb.icon.draw2(move |f|{
            search_icon.draw(f.x(), f.y(), 24, 24);    
        });

        sb.icon.set_frame(FrameType::NoBox); 
        sb.icon.set_color(Color::Red);
        sb.pack.end();
        sb.pack.set_type(PackType::Horizontal);
        sb.pack.set_frame(FrameType::BorderFrame);
        sb.pack.set_color(Color::Black);
        sb.input.draw2(move |b|{
            set_draw_color(Color::Black);
            // push_clip("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
            draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
        });
        let sc = s_items.clone();
        sb.input.set_callback2(move |b| {
            let mut mc = vec![];
            if !b.value().is_empty(){
                //remove "Search..." text when adding input
                b.draw2(move |b|{
                    set_draw_color(Color::Black);
                    draw_text2("", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
                });

                for x in 0..sc.len(){
                    let m = best_match(&b.value().to_string(), &sc[x]);
                    //best_match returns an Option<Match> so we need to check for Some and None values
                    match m {
                        Some(val) => {
                            let res = Result{
                                score: best_match(&b.value().to_string(), &sc[x]).unwrap().score(),
                                val: sc[x].clone(),
                            };
                            mc.push(res);

                        }
                        None => {
                            let res = Result{
                                score: 0,
                                val: sc[x].clone(),
                            };
                            mc.push(res);
                        }
                    }
                }  
                mc.sort_by(|a, b| b.score.cmp(&a.score));
                s.send(Message::UpdateSearch(mc.clone()));
            } else {
                b.draw2(move |b|{
                    set_draw_color(Color::Black);
                    draw_text2("Search...", b.x() + 10 , b.y(), b.width(), b.height(), Align::Inside | Align::Left );
                });
                s.send(Message::ResetTable)
            }
            //sort our list of results by the their score values


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
    pub fn do_thing(&mut self){
        println!("{}", "say hello");
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