use std::collections::HashMap;

use log::info;
use shipyard::{IntoWorkloadSystem, Workload, WorkloadBuilder};

use crate::{application::AppExit, events::Events, logger::init_logger, Application};
use crate::{stage, Plugin};

/// Configure [App]s using the builder pattern
pub struct Builder {
  pub app: Application,
  stages: HashMap<&'static str, WorkloadBuilder>,
}

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

impl Builder {
  pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
  where
    T: Plugin,
  {
    info!("added plugin: {}", plugin.name());
    plugin.build(self);
    self
  }

  pub fn add_resource<T: 'static + Send + Sync>(&mut self, resource: T) -> &mut Self {
    self.app.world.add_unique(resource).unwrap();
    self
  }

  pub fn set_runner(&mut self, run_fn: impl Fn(Application) + 'static) -> &mut Self {
    self.app.runner = Box::new(run_fn);
    self
  }

  pub fn create_stage(&mut self, name: &'static str) -> &mut Self {
    self.stages.insert(name, Workload::builder(name));
    self
  }

  pub fn add_to_stage<S, B, R>(&mut self, stage_name: &'static str, system: S) -> &mut Self
  where
    S: IntoWorkloadSystem<B, R>,
  {
    let builder = self.stages.get_mut(stage_name).unwrap();
    builder.with_system(system);
    self
  }

  pub fn add_system<S, B, R>(&mut self, system: S) -> &mut Self
  where
    S: IntoWorkloadSystem<B, R>,
  {
    self.add_to_stage(stage::UPDATE, system);
    self
  }

  pub fn add_startup_system<S, B, R>(&mut self, system: S) -> &mut Self
  where
    S: IntoWorkloadSystem<B, R>,
  {
    self.add_to_stage(stage::STARTUP, system);
    self
  }

  pub fn create_default_stages(&mut self) {
    // Startup systems, called once
    self.create_stage(stage::FIRST);
    self.create_stage(stage::STARTUP);

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

  pub fn add_event<T>(&mut self) -> &mut Self
  where
    T: Send + Sync + 'static,
  {
    self
      .add_resource(Events::<T>::default())
      .add_to_stage(stage::EVENT, &Events::<T>::update_system)
  }

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
