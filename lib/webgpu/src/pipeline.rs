use std::collections::HashMap;

pub struct Pipeline {
  pub num_indices: u32,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub pipeline_layout: wgpu::PipelineLayout,
  pub render_pipeline: wgpu::RenderPipeline,
}

#[derive(Default)]
pub struct Pipelines {
  dict: HashMap<String, Pipeline>,
}

/// Pipeline dictionary
impl Pipelines {
  /// Add a pipeline to the pipeline dictionary
  pub fn add(&mut self, name: &str, pipeline: Pipeline) {
    self.dict.insert(name.to_string(), pipeline);
  }

  /// Get a pipeline from the pipeline dictionary
  pub fn get(&self, name: &str) -> &Pipeline {
    self.dict.get(name).unwrap()
  }

  /// Get a mutable pipeline from the pipeline dictionary
  pub fn get_mut(&mut self, name: &str) -> &mut Pipeline {
    self.dict.get_mut(name).unwrap()
  }

  /// Get all pipelines from the pipeline dictionary
  pub fn get_all(&self) -> Vec<&Pipeline> {
    self.dict.values().collect()
  }
}
