use crate::{application::AppExit, events::Events, logger::init_logger, Application};
use crate::{stage, Plugin};
use log::info;
use shipyard::{IntoWorkloadSystem, Workload, WorkloadBuilder, World};
use std::collections::HashMap;

///
/// Configure App using the builder pattern
///
pub struct Builder {
  pub app: Application,
  stages: HashMap<&'static str, WorkloadBuilder>,
}

///
/// Impl default
///
impl Default for Builder {
  fn default() -> Self {
    let mut app_builder = Builder {
      app: Application::default(),
      stages: Default::default(),
    };

    // Init logger
    init_logger();

    app_builder.create_default_stages();
    app_builder.add_event::<AppExit>();

    app_builder
  }
}

///
/// Build each module resources systems etc.
///
impl Builder {
  ///
  /// Add plugin
  ///
  pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
  where
    T: Plugin,
  {
    info!("added plugin: {}", plugin.name());
    plugin.build(self);
    self
  }

  ///
  /// Add resource
  ///
  pub fn add_resource<T: 'static + Send + Sync>(&mut self, resource: T) -> &mut Self {
    self.app.world.add_unique(resource).unwrap();
    self
  }

  ///
  /// Set app runner
  ///
  pub fn set_runner(&mut self, run_fn: impl Fn(Application) + 'static) -> &mut Self {
    self.app.runner = Box::new(run_fn);
    self
  }

  ///
  /// Create app stage
  ///
  pub fn create_stage(&mut self, name: &'static str) -> &mut Self {
    self.stages.insert(name, Workload::builder(name));
    self
  }

  ///
  /// Run specific stage after other stage
  ///
  pub fn run_on_stage(&mut self, stage_name: &'static str, run_after: &'static str) -> &mut Self {
    let stage = self.stages.get_mut(stage_name).unwrap();
    stage.with_workload(run_after);
    self
  }

  ///
  /// Subscribe system on stage
  ///
  pub fn add_to_stage<S, B, R>(&mut self, stage_name: &'static str, system: S) -> &mut Self
  where
    S: IntoWorkloadSystem<B, R>,
  {
    let builder = self.stages.get_mut(stage_name).unwrap();
    builder.with_system(system);
    self
  }

  ///
  /// Add system to update stage
  ///
  pub fn add_system<S, B, R>(&mut self, system: S) -> &mut Self
  where
    S: IntoWorkloadSystem<B, R>,
  {
    self.add_to_stage(stage::UPDATE, system);
    self
  }

  ///
  /// Add system to startup stage, it will be executed once per session
  ///
  pub fn add_startup_system<S, B, R>(&mut self, system: S) -> &mut Self
  where
    S: IntoWorkloadSystem<B, R>,
  {
    self.add_to_stage(stage::STARTUP, system);
    self
  }

  ///
  /// Init default stages
  ///
  fn create_default_stages(&mut self) {
    // Startup systems, called once
    self.create_stage(stage::FIRST);
    self.create_stage(stage::STARTUP);
    self.create_stage(stage::POST_STARTUP);

    // Events system stages, every frame
    self.create_stage(stage::PRE_EVENT);
    self.create_stage(stage::EVENT);

    // Main update system, every frame
    self.create_stage(stage::PRE_UPDATE);
    self.create_stage(stage::UPDATE);
    self.create_stage(stage::POST_UPDATE);

    // Main render system, every frame
    self.create_stage(stage::PRE_RENDER);
    self.create_stage(stage::RENDER);
    self.create_stage(stage::POST_RENDER);
  }

  ///
  /// Register new event type
  ///
  pub fn add_event<T>(&mut self) -> &mut Self
  where
    T: Send + Sync + 'static,
  {
    self
      .add_resource(Events::<T>::default())
      .add_to_stage(stage::EVENT, &Events::<T>::update_system)
  }

  ///
  /// Build and run all systems
  ///
  pub fn run(&mut self) {
    // Add all workloads to the world
    for (_, workload) in self.stages.iter_mut() {
      workload.add_to_world(&self.app.world).unwrap();
    }

    self.stages.clear();

    // Start app
    let app = std::mem::take(&mut self.app);
    app.run();
  }
}
