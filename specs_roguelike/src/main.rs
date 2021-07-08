use tcod::colors;
use tcod::console::*;
use specs::{World, Component, VecStorage, System, SystemData, EntityBuilder, ReadStorage, prelude::*};

const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPTS: i32 = 20;

#[derive(Debug, PartialEq)]
pub struct Position{
  x: i32,
  y: i32, 
}

impl Component for Position{
    type Storage = VecStorage<Self>;
}
#[derive(Debug, PartialEq)]
pub struct CharactherGlyph{
  glyph: char
}
impl Component for CharactherGlyph {
  type Storage = VecStorage<Self>;
}

pub struct Render{
  window: Root,
}
impl<'a> System<'a> for Render{
  type SystemData =(
    ReadStorage<'a, CharactherGlyph>,
    ReadStorage<'a, Position>,
  );
  fn run(&mut self, data: Self::SystemData){
    use specs::Join;
    let root = &mut self.window;
    let (sprites, positions) = data;
    root.clear();

    for (sprite, pos) in (&sprites, &positions).join(){
      dbg!(sprite);
      root.put_char(pos.x, pos.y, sprite.glyph, BackgroundFlag::None);
    }
    root.flush();
    root.wait_for_keypress(false);

     
  }

}
struct PrintingSystem;
impl <'a>System<'a> for PrintingSystem{
  type SystemData =( ReadStorage<'a, Position>, ReadStorage<'a, PrintMeTag>);

  fn run(&mut self, data: Self::SystemData){
    println!("Running System Print Me");
    use specs::Join;
    let (position, print_me) = data;
    for (pos, _) in (&position, &print_me).join(){

      println!("{:?}", pos);
    }
  }
}
struct NotPrintingSystem;
impl <'a>System<'a> for NotPrintingSystem{
  type SystemData =( ReadStorage<'a, Position>, ReadStorage<'a, PrintMeTag>);

  fn run(&mut self, data: Self::SystemData){
    use specs::Join;
    let (position, print_me) = data;
    println!("Running System Not Print Me");
    for (pos, _) in (&position, &print_me).join(){
      println!("{:?}", pos);
    }
  }
}

#[derive(Default)]
struct PrintMeTag;
impl Component for PrintMeTag{
  type Storage = specs::NullStorage<Self>;
} 

  

fn main(){
  let mut root = Root::initializer()
  .font("terminal16x16_gs_ro.png", FontLayout::Tcod)
  .font_type(FontType::Greyscale)
  .size(SCREEN_WIDTH, SCREEN_HEIGHT)
  .title("Roguelike using specs")
  .init();
  root.set_default_foreground(colors::WHITE);

  while !root.window_closed(){
    root.clear();
    root.put_char(10,10, '@', BackgroundFlag::None);
    root.flush();
    root.wait_for_keypress(false);
  }

  let mut world = World::new();
  let mut dispatcher = specs::DispatcherBuilder::new()
  .with(PrintingSystem, "print_sys", &[])
  .with(NotPrintingSystem, "not_print_sys", &["print_sys"])
  // .with(NotPrintingSystem, "not_print_sys", &[])
  .with_thread_local(Render{window: root})
  .build();
  dispatcher.dispatch(&mut world);
  // dispatcher.setup(&mut world);

  world.create_entity().with(Position{x: 10, y: 10}).build();
	world
		.create_entity()
		.with(Position { x: 10, y: 10 })
		.with(CharactherGlyph { glyph: 'y' })
		.build();
  world.create_entity().with(Position{x: 50, y: 10}).with(CharactherGlyph{glyph: 'b'}).build();
  world.create_entity().with(PrintMeTag{}).with(Position{x: 10, y: 10}).build();
  world.create_entity().with(Position{x: 10, y: 10}).build();
  world.create_entity().with(Position{x: 10, y: 10}).build();
  world.create_entity().with(PrintMeTag{}).with(Position{x: 100, y: 1000}).build();


  tcod::system::set_fps(LIMIT_FPTS);
  world.maintain()


}