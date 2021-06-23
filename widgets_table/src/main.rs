use fltk::{image::*, prelude::*, *, enums::*, button::*, draw::*, group::*};
use std::rc::Rc;
use std::cell::RefCell;
struct MyTable {
    #[allow(dead_code)]
    container: group::Scroll,
    cols: i32,
    rows: i32,
}

impl MyTable {
    pub fn new(cols: i32, rows: i32, col_w: i32, row_h: i32) -> Self {
        let mut img = SvgImage::load("search-24px.svg").unwrap();
        img.scale(col_w, row_h - 20, true, false);
        let mut img = SvgImage::load("search-24px.svg").unwrap();
        let mut container = group::Scroll::default().size_of_parent();
        let col_width = 60;
        let row_height = 60;
        for x in 0..50{
          let mut button= Button::new(0,0,col_width,row_height,None);
          button.set_label(&x.to_string());
          button.set_image(Some(img.clone()));
        };
        let table_redraws = Rc::from(RefCell::from(0));
        

        container.handle(move|widg, ev|{
          match ev{
            Event::Resize=>{
              dbg!("got resize event");
              //acounts children any scroll group starts with
              let true_children = widg.children()-2;
              // let true_children = widg.children()-2;

              //keep track of how many draw calls we're making
              let table_redraws_cl = table_redraws.clone();

              widg.draw(move|widg|{
                dbg!(widg.children());
                //number of columns, number of rows
                let col_count = std::cmp::min(true_children, (widg.width() as f32/col_width as f32).floor() as i32);
                let row_count = (true_children as f32/col_count as f32).ceil();
                // dbg!(col_count, row_count);
                //draw a blue vertical line for every column
                *table_redraws_cl.borrow_mut() += 1;
                set_draw_color(Color::Blue);
                for x in 0..col_count as i32{
                  let t_x = col_width*x;
                  draw_line(t_x, widg.y(), t_x, widg.y()+widg.height());
                };

                //draw a red horiontal line for every row
                set_draw_color(Color::Red);
                for y in 0..row_count as i32{
                  let t_y = row_height*y;
                  draw_line(widg.x(), t_y, widg.x()+widg.width(), t_y);
                };

                //place the children into the grid
                for z in 0..(row_count) as i32{
                  //ypos is row number * row height
                  let t_y = (z)*row_height;
                  //save a starting point to increment by the column number
                  let start = z*col_count as i32; 

                  for q in 0..col_count as i32{
                      let t_x = q*col_width;
                      let child_ind = (start+q);
                      d
                      if child_ind < true_children{
                        // dbg!(child_ind);
                        let mut child = widg.child(child_ind).unwrap();
                        child.resize(t_x, t_y, child.width(), child.height());  
                        child.redraw();
                      }
   
                  }
                }
                widg.draw_children();
              });
            // dbg!("got container resize");
            true
            }
            _=>false              
          }
        });
        container.set_frame(FrameType::FlatBox);
        container.set_color(Color::Yellow);
        container.make_resizable(true);
        container.end();
        Self { 
            container: container,
            cols: cols,
            rows: rows,
        }
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(402, 275);
    let _table = MyTable::new(5, 8, 90, 80);
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}

