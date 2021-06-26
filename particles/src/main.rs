//template for creating simple examples
use fltk::*;
use fltk::{app::*, frame::*, window::*, image::*, table::*, button::*, input::*, group::*, prelude::*};
use rapier2d::prelude::*;

#[derive(Debug, Clone)]
pub enum Message {
    Test,   
}

fn main() {

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
  
    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
    collider_set.insert(collider);
  
    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(vector![0.0, 10.0])
            .build();
    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
  
    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    
    let app = App::default();
    let (s, r) = channel::<Message>();
    let mut win = Window::new(200, 200, 1000, 1000, "Template");
    
    let mut button = Button::new(0,0,50,20,"CLICK ME");

    win.end();
    win.show();

    while app.wait() {
            physics_pipeline.step(
              &gravity,
              &integration_parameters,
              &mut island_manager,
              &mut broad_phase,
              &mut narrow_phase,
              &mut rigid_body_set,
              &mut collider_set,
              &mut joint_set,
              &mut ccd_solver,
              &physics_hooks,
              &event_handler,
            );
        
            let ball_body = &rigid_body_set[ball_body_handle];
            println!(
              "Ball altitude: {}",
              ball_body.translation().y*10.0
            );
            button.set_pos((ball_body.translation().x*10.0) as i32, (ball_body.translation().y*20.0) as i32);
        win.redraw();
        app::sleep(0.016);
    }
}
