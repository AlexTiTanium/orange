use crate::events::GameEvent;
use crate::stage;
use shipyard::{Workload, WorkloadBuilder, World};
use std::cell::RefCell;

pub struct State {
  // world
  pub world: World,

  // Event system
  events: RefCell<Vec<Box<dyn Fn(&GameEvent)>>>,

  // Workload builders
  pub render: RefCell<WorkloadBuilder>,
  pub post_render: RefCell<WorkloadBuilder>,
  pub pre_render: RefCell<WorkloadBuilder>,
}

impl State {
  pub fn new() -> Self {
    let world = World::new();

    let pre_render = RefCell::new(Workload::builder(stage::PRE_RENDER));
    let render = RefCell::new(Workload::builder(stage::RENDER));
    let post_render = RefCell::new(Workload::builder(stage::POST_RENDER));

    // Event system
    let events = RefCell::new(Vec::new());

    Self {
      world,
      events,
      render,
      post_render,
      pre_render,
    }
  }

  /// Add resources to the world
  pub fn add_resource<T: 'static + Send + Sync>(&self, resource: T) -> &Self {
    self.world.add_unique(resource);
    &self
  }

  /// Add none sync resource
  pub fn add_none_sync_resource<T: 'static>(&self, component: T) -> &Self {
    self.world.add_unique_non_send_sync(component);
    &self
  }

  /// Create workload
  pub fn create_workload(&self) -> WorkloadBuilder {
    WorkloadBuilder::default()
  }

  /// Create new  empty workload
  pub fn workload(&self) -> shipyard::WorkloadBuilder {
    WorkloadBuilder::default()
  }

  /// Execute workload
  pub fn run_workload(&self, stage: &str) {
    self.world.run_workload(stage);
  }

  /// Build all known workloads, must be called after all modules initialization
  pub fn build(&self) {
    self.pre_render.borrow_mut().add_to_world(&self.world).unwrap();
    self.render.borrow_mut().add_to_world(&self.world).unwrap();
    self.post_render.borrow_mut().add_to_world(&self.world).unwrap();
  }

  /// Listen specific game event
  pub fn listen(&self, listener: Box<dyn Fn(&GameEvent)>) {
    let events = &mut self.events.borrow_mut();
    events.push(listener);
  }

  /// Send game event to listener
  pub fn send(&self, event: GameEvent) {
    let events = &self.events.borrow();
    for listener in events.iter() {
      listener(&event);
    }
  }

  pub fn create_resources(&self) {
    // self.world.add_unique(Window::default());
    // self.world.add_unique(Camera::default());
    // self.world.add_unique(Input::default());
    // self.world.add_unique(Time::default());
    // self.world.add_unique(FPS::default());
    // self.world.add_unique(Assets::new());
  }

  // pub fn create_game_object(&self) -> EntityId {
  //   //self.world.run(entities::create_game_object)
  // }

  //pub fn handle_window_events(&self, event: &WindowEvent) {
  // handle_keyboard_input(&self.world, event);
  // handle_window_resize(&self.world, event);
  // handle_camera_resize(&self.world, event);
  //}

  // pub fn attach_window(&self, window: &winit::window::Window) {
  //   let scale_factor = window.scale_factor(); // TODO: round?
  //   let logical_size = window.inner_size().to_logical::<f32>(scale_factor);
  //   let physical_size = logical_size.to_physical::<f32>(scale_factor);

  //   update_window_sizes(&self.world, logical_size, physical_size, scale_factor);
  //   handle_camera_update(&self.world);
  // }

  // pub fn update_time(&self) {
  //   self.world.run(time::update);
  // }

  // pub fn update_fps(&self) {
  //   self.world.run(time::update_fps);
  // }
}
