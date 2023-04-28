use common::{events::Events, futures_lite, log, AllStoragesViewMut, UniqueView, UniqueViewMut};
use wgpu::{Adapter, Device, Instance, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration};
use window::{events::WindowInnerEvent, PhysicalSize, WindowContext};

use crate::Pipelines;

pub struct WebGpuState {
  surface: Surface,
  pub device: Device,
  pub queue: Queue,
  pub config: SurfaceConfiguration,
  pub size: PhysicalSize<u32>,
}

impl WebGpuState {
  ///
  /// Startup system. create startup resource
  ///
  pub fn init(all_storages: AllStoragesViewMut) {
    let context = all_storages.borrow::<UniqueView<WindowContext>>().unwrap();
    let state = Self::new(&context);
    all_storages.add_unique::<WebGpuState>(state);
    log::info!("WebGpuState::init() end");
  }

  ///
  /// Resize system
  ///
  pub fn on_resize_event(
    window_event: UniqueView<Events<WindowInnerEvent>>,
    context: UniqueView<WindowContext>,
    mut state: UniqueViewMut<WebGpuState>,
  ) {
    let events = window_event.get_reader().iter(&window_event);

    for event in events {
      match event {
        WindowInnerEvent::Resized(_, _) => {
          state.resize(context.physical_size());
        }
        WindowInnerEvent::ScaleFactorChange(_, _) => {
          state.resize(context.physical_size());
        }
        _ => (),
      }
    }
  }

  ///
  /// Render system
  ///
  pub fn on_render_stage(
    mut state: UniqueViewMut<WebGpuState>,
    context: UniqueView<WindowContext>,
    pipelines: UniqueView<Pipelines>,
  ) {
    match state.render(&pipelines) {
      Ok(_) => {}
      // Reconfigure the surface if lost
      Err(wgpu::SurfaceError::Lost) => state.resize(context.physical_size()),
      // The system is out of memory, we should probably quit
      Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory exit"),
      // All other errors (Outdated, Timeout) should be resolved by the next frame
      Err(e) => log::error!("{:?}", e),
    }
  }

  ///
  ///  Creating some of the wgpu types requires async code
  ///
  pub fn new(context: &WindowContext) -> Self {
    log::info!("WebGpuState::new() start");
    // The instance is a handle to our GPU
    let window = context.window();
    let size = context.physical_size();

    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: Default::default(),
    });

    // # Safety
    //
    // The surface needs to live as long as the window that created it.
    // State owns the window so this should be safe.
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    // Block on async here
    let (adapter, device, queue) = futures_lite::future::block_on(Self::init_render(&instance, &surface));
    let surface_caps = surface.get_capabilities(&adapter);

    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps
      .formats
      .iter()
      .copied()
      .filter(|f| f.describe().srgb)
      .next()
      .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };

    surface.configure(&device, &config);

    log::info!("WebGpuState::new()");

    Self {
      surface,
      device,
      queue,
      config,
      size,
    }
  }

  ///
  /// Create all async stuff here
  ///
  async fn init_render(instance: &Instance, surface: &Surface) -> (Adapter, Device, Queue) {
    let adapter = instance
      .request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          features: wgpu::Features::empty(),
          // WebGL doesn't support all of wgpu's features, so if
          // we're building for the web we'll have to disable some.
          limits: if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults()
          } else {
            wgpu::Limits::default()
          },
          label: None,
        },
        None, // Trace path
      )
      .await
      .unwrap();

    return (adapter, device, queue);
  }

  ///
  /// Surface resize
  ///
  pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }

  /// Method to render experiments
  pub fn render(&mut self, pipelines: &Pipelines) -> Result<(), wgpu::SurfaceError> {
    // get the current frame texture from the surface
    let output = self.surface.get_current_texture()?;
    // create a view into the texture
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    // create a command encoder to record and submit commands to the GPU device
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render Encoder"),
    });

    // begin the render pass process by creating a render pass descriptor
    // this describes how resources will be used in the render pass
    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"), // A human-readable name for the render pass.
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          // Array of color attachments for the render pass. In this case we only have one attachment.
          view: &view, // a reference to the texture view that contains the color data output from the pipeline associated with this attachment slot.
          resolve_target: None, // a target into which samples are explicitly resolved before storage. Used for multisampling features. It is set to none in this code meaning there is no multisampling.
          ops: wgpu::Operations {
            // operations to perform on the texture data(output from the pipeline) before/after rendering and loading/storing it back. `load` specifies the action to take when loading the output while `store` is the action to take when storing the new output.
            load: wgpu::LoadOp::Clear(wgpu::Color {
              //specifies color to clear the attachment with at the start of the rendering pass.
              r: 0.1,
              g: 0.6,
              b: 0.1,
              a: 1.0,
            }),
            store: true,
          },
        })],
        depth_stencil_attachment: None, //no stenci depth attachment specified here
      });

      // set the render pipelines to use for this render pass
      for pipeline in pipelines.get_all() {
        render_pass.set_pipeline(&pipeline.render_pipeline);
        render_pass.set_vertex_buffer(0, pipeline.vertex_buffer.slice(..));
        render_pass.set_index_buffer(pipeline.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..pipeline.num_indices, 0, 0..1);
      }
    }

    // submit the list of recorded commands to the device's command queue
    self.queue.submit(std::iter::once(encoder.finish()));

    // present the final render on the surface
    output.present();

    Ok(())
  }
}
