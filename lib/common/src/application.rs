pub use crate::Builder;
use crate::{events::Events, stage};
use log::info;
use shipyard::{UniqueViewMut, World};
/// An event that indicates the app should exit. This will fully exit the app process.
#[derive(Debug, Clone)]
pub struct AppExit;

pub struct Application {
  // world
  pub world: World,
  pub runner: Box<dyn Fn(Application)>,
}

impl Default for Application {
  fn default() -> Self {
    Self {
      world: Default::default(),
      runner: Box::new(run_once),
    }
  }
}

fn run_once(mut app: Application) {
  app.initialize();
  app.update();
}

impl Application {
  pub fn build() -> Builder {
    Builder::default()
  }

  pub fn initialize(&mut self) {
    info!("App Initialize");
    self.world.run_workload(stage::FIRST).unwrap();
    self.world.run_workload(stage::STARTUP).unwrap();
    self.world.run_workload(stage::POST_STARTUP).unwrap();
  }

  pub fn exit(&self) {
    self.send(AppExit);
  }

  pub fn send<T>(&self, event: T)
  where
    T: Send + Sync + 'static,
  {
    let mut events = self.world.borrow::<UniqueViewMut<Events<T>>>().unwrap();
    events.send(event);
  }

  pub fn update(&mut self) {
    self.world.run_workload(stage::EVENT).unwrap();
    self.world.run_workload(stage::UPDATE).unwrap();
  }

  pub fn run_stage(&mut self, stage: &'static str) {
    self.world.run_workload(stage).unwrap();
  }

  pub fn run(mut self) {
    let runner = std::mem::replace(&mut self.runner, Box::new(run_once));
    (runner)(self);
  }
}
