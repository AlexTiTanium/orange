use common::{events::Events, futures_lite, AllStoragesViewMut, UniqueView, UniqueViewMut};
use wgpu::*;
use window::{events::WindowInnerEvent, PhysicalSize, WindowContext};

pub struct WebGpuState {
  surface: Surface,
  device: Device,
  queue: Queue,
  config: SurfaceConfiguration,
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
  pub fn on_render_stage(mut state: UniqueViewMut<WebGpuState>, context: UniqueView<WindowContext>) {
    match state.render() {
      Ok(_) => {}
      // Reconfigure the surface if lost
      Err(wgpu::SurfaceError::Lost) => state.resize(context.physical_size()),
      // The system is out of memory, we should probably quit
      Err(wgpu::SurfaceError::OutOfMemory) => panic!("Out of memory exit"),
      // All other errors (Outdated, Timeout) should be resolved by the next frame
      Err(e) => eprintln!("{:?}", e),
    }
  }

  ///
  ///  Creating some of the wgpu types requires async code
  ///
  pub fn new(context: &WindowContext) -> Self {
    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU

    let window = context.window();
    let size = context.physical_size();
    let instance = Instance::new(Backends::all());
    let surface = unsafe { instance.create_surface(window) };

    // Block on async here
    let (adapter, device, queue) = futures_lite::future::block_on(Self::init_render(&instance, &surface));

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    surface.configure(&device, &config);

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

  ///
  /// Render experiments
  ///
  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render Encoder"),
    });

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.1,
              g: 0.1,
              b: 0.1,
              a: 1.0,
            }),
            store: true,
          },
        })],
        depth_stencil_attachment: None,
      });
    }

    // submit will accept anything that implements IntoIter
    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
