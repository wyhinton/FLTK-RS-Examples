use fltk::{app, button::*, group::*, frame::*, draw::*};
use fltk::*;
// use crate::{SLOTWIDTH, PALETTE, Message, IconButton};
// mod scroll_header;
// use scroll_header::ScrollHeader;

#[derive(Clone)]
pub struct ScrollGroup {
    pub master: Group,
    // pub master: Scroll,
    pub pack: Group, 
    pub inner_group: Pack, 
}


impl ScrollGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32, mut inner_group: Pack) -> Self {
        
        let mut win = group::Group::new(x, y, w, h-25, "");
        win.set_frame(FrameType::FlatBox);
        win.set_color(Color::Red);
        win.make_resizable(false);
        let mut scroll = Group::new(0,0,300, h-25,"");

        scroll.set_frame(FrameType::FlatBox);
        let mut pack = group::Pack::new(x,0,w,h,"");

        pack.end();
        scroll.end();
        win.end();
        pack.add(&inner_group);
        let pack_cl = pack.clone();
        let pack_cl2 = pack.clone();
        let mut ig_c = inner_group.clone();
        win.draw2(move|widg|{
            push_clip(widg.x(),widg.y(), widg.width(), widg.height());
            pop_clip();
        });

        scroll.handle2({let mut win = win.clone(); move |s, ev| match ev {
            Event::MouseWheel => {
                let dy = app::event_dy();
                
                if pack_cl.height() > pack_cl.parent().unwrap().parent().unwrap().height() {
                    if dy > 0 {
                        dbg!(pack.height()-s.height());
                        dbg!(pack.y(), s.y(), pack.height());
                        if pack.y() > s.y()-(pack.height()-s.height()) {
                            pack.set_pos(pack.x(), pack.y() - 10);
                            s.set_damage_type(Damage::Scroll);
                            win.set_damage_type(Damage::Scroll);
                            // ig_c.set_damage_type(Damage::Scroll);
                            
                        }
            
                    } else {
                        dbg!(pack.y(), s.y());
                        pack.set_pos(pack.x(), pack.y() + 10);
                        s.set_damage_type(Damage::Scroll);
                        win.set_damage_type(Damage::Scroll);
                        // ig_c.set_damage_type(Damage::Scroll);
                    }
                };
                true
            }
            _ => false,
        }});

        let sg = ScrollGroup{
            master: win, 
            pack: scroll, 
            inner_group: pack_cl2,
        };
    
        sg
    }
}

impl std::ops::Deref for ScrollGroup {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        &self.pack
    }
}

impl std::ops::DerefMut for ScrollGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pack
    }
}

