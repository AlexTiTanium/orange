pub use crate::Builder;
use crate::{events::Events, stage};
use log::info;
use shipyard::{UniqueViewMut, World};

pub struct Application {
  // ECS world
  pub world: World,

  // Event loop
  pub runner: Box<dyn Fn(Application)>,
}

///
/// Application support default initialization
///
impl Default for Application {
  fn default() -> Self {
    Self {
      world: Default::default(),
      runner: Box::new(Application::run_once),
    }
  }
}

///
/// Application implementation
///
impl Application {
  ///
  /// Default runner
  ///
  pub fn run_once(mut app: Application) {
    app.initialize();
    app.update();
    app.render();
  }

  ///
  /// Application builder
  ///
  pub fn build() -> Builder {
    Builder::default()
  }

  ///
  /// Application initialization
  ///
  pub fn initialize(&mut self) {
    info!("App Initialize");

    self.run_stage(stage::FIRST);
    self.run_stage(stage::STARTUP);
    self.run_stage(stage::POST_STARTUP);
  }

  ///
  /// Send application exit event
  ///
  pub fn exit(&self) {
    self.send(AppExit);
  }

  ///
  /// Send event
  ///
  pub fn send<T>(&self, event: T)
  where
    T: Send + Sync + 'static,
  {
    let mut events = self.world.borrow::<UniqueViewMut<Events<T>>>().unwrap();
    events.send(event);
  }

  ///
  /// Send update
  ///
  pub fn update(&mut self) {
    // Read input
    self.run_stage(stage::PRE_EVENT);
    self.run_stage(stage::EVENT);

    // Run logic
    self.run_stage(stage::PRE_UPDATE);
    self.run_stage(stage::UPDATE);
    self.run_stage(stage::POST_UPDATE);
  }

  ///
  /// Render one frame
  ///
  pub fn render(&mut self) {
    self.run_stage(stage::PRE_RENDER);
    self.run_stage(stage::RENDER);
    self.run_stage(stage::POST_RENDER);
  }

  ///
  /// Run stage stage
  ///
  pub fn run_stage(&mut self, stage: &'static str) {
    self.world.run_workload(stage).unwrap();
  }

  ///
  /// Start application
  ///
  pub fn run(mut self) {
    let runner = std::mem::replace(&mut self.runner, Box::new(Application::run_once));
    (runner)(self);
  }
}

///
/// An event that indicates the app should exit. This will fully exit the app process
///
#[derive(Debug, Clone)]
pub struct AppExit;
