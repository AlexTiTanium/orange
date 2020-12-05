use crate::resources::Window;
use crate::resources::*;
use crate::stage;
use crate::systems::*;
use crate::*;
use std::{cell::RefCell, collections::HashMap};
use winit::event::WindowEvent;

pub fn create_state() -> State {
  let state = State::new();
  state.create_resources();
  state
}

pub struct State {
  // world
  pub world: World,

  // Workload builders
  workloads: RefCell<HashMap<String, Vec<WorkloadBuilder>>>,
}

impl State {
  pub fn new() -> Self {
    let world = World::new();

    // Store workloads builders, after self.build() them will be cleared
    let workloads = RefCell::new(HashMap::new());

    Self { world, workloads }
  }

  /// Adds resources to the world
  pub fn add_resource<T: 'static + Send + Sync>(&self, resource: T) -> &Self {
    self.world.add_unique(resource);
    &self
  }

  /// Add systems to UPDATE workload
  pub fn on_update(&self, workload: WorkloadBuilder) {
    self.add_workload(stage::UPDATE, workload);
  }

  /// Add new workloads
  pub fn add_workload(&self, stage: &str, workload: WorkloadBuilder) {
    let workloads = &mut self.workloads.borrow_mut();
    let workloads_vec = workloads.entry(stage.to_string()).or_insert(vec![]);
    workloads_vec.push(workload);
  }

  /// Execute workload
  pub fn run_workload(&self, stage: &str) {
    self.world.run_workload(stage);
  }

  /// Build all known workloads, must be called after all modules initialization
  pub fn build(&self) {
    let workloads = &mut self.workloads.borrow_mut();

    for (stage, builders) in workloads.iter_mut() {
      let mut workload = WorkloadBuilder::new(stage.to_string());

      for mut builder in builders.iter_mut() {
        workload.append(&mut builder);
      }

      let info = workload.add_to_world_with_info(&self.world).unwrap();
      println!("{:?}", info)
    }

    workloads.clear();
  }

  pub fn create_resources(&self) {
    self.world.add_unique(Window::default());
    self.world.add_unique(Camera::default());
    self.world.add_unique(Input::default());
    self.world.add_unique(Time::default());
    self.world.add_unique(FPS::default());
    self.world.add_unique(Assets::new());
  }

  pub fn create_game_object(&self) -> EntityId {
    self.world.run(entities::create_game_object)
  }

  pub fn handle_window_events(&self, event: &WindowEvent) {
    handle_keyboard_input(&self.world, event);
    handle_window_resize(&self.world, event);
    handle_camera_resize(&self.world, event);
  }

  pub fn attach_window(&self, window: &winit::window::Window) {
    let scale_factor = window.scale_factor(); // TODO: round?
    let logical_size = window.inner_size().to_logical::<f32>(scale_factor);
    let physical_size = logical_size.to_physical::<f32>(scale_factor);

    update_window_sizes(&self.world, logical_size, physical_size, scale_factor);
    handle_camera_update(&self.world);
  }

  pub fn update_time(&self) {
    self.world.run(time::update);
  }

  pub fn update_fps(&self) {
    self.world.run(time::update_fps);
  }
}
