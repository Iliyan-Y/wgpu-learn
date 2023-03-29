use log::error;
use winit::{event::*, window::Window};

pub struct State {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  window: Window,
  color: wgpu::Color,
  click: bool,
  render_pipeline: wgpu::RenderPipeline,
  shader_color: String,
}

impl State {
  // Creating some of the wgpu types requires async code
  pub async fn new(window: Window) -> Self {
    let size = window.inner_size();

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

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
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

    let color = wgpu::Color::BLUE;
    let click = false;

    // +++++++++++
    // SHADER Pipeline
    // +++++++++++
    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

    let shader_color = "main".into();

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: &format!("vs_{}", shader_color), //  shader fn name
        buffers: &[], //  We're specifying the vertices in the vertex shader itself, so we'll leave this empty.
      },
      // fragment is optional
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: &format!("fs_{}", shader_color), //in the shader file
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
      }),

      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList, // every three vertices will correspond to one triangle
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw, // given triangle is facing forward or not
        cull_mode: Some(wgpu::Face::Back), // Triangles that are not considered facing forward are culled (not included in the render

        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::DEPTH_CLIP_CONTROL
        unclipped_depth: false,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },

      depth_stencil: None,
      // Multisampling is ADVANCED topic
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0, // specifies which samples should be active. In this case, we are using all of them..
        alpha_to_coverage_enabled: false, // anti-aliasing
      },
      multiview: None, // how many array layers the render attachments can have
    });

    Self {
      window,
      surface,
      device,
      queue,
      config,
      size,
      color,
      click,
      render_pipeline,
      shader_color,
    }
  }

  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }

  pub fn input(&mut self, event: &WindowEvent) -> bool {
    // if the method returns true, the main loop won't process the event any further.
    // false

    match event {
      WindowEvent::CursorEntered { .. } => {
        self.color = wgpu::Color::GREEN;
        true
      }

      WindowEvent::CursorLeft { .. } => {
        self.click = false;
        self.color = wgpu::Color::BLACK;
        true
      }

      WindowEvent::MouseInput { button, .. } => {
        if MouseButton::Left.eq(button) {
          self.click = true;
        } else {
          self.click = false;
        }

        false
      }

      WindowEvent::CursorMoved { position, .. } => {
        // error!("{:?}", self.click);
        if self.click {
          self.color = wgpu::Color {
            r: position.x as f64 / self.size.width as f64,
            g: position.y as f64 / self.size.height as f64,
            b: 1.0,
            a: 1.0,
          };
          self.click = false;
          true
        } else {
          false
        }
      }

      _ => false,
    }
  }

  pub fn update(&mut self) {
    // todo!()
  }

  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    // the {} block borrows encoder mutably aka &mut self
    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          // This is what @location(0) in the fragment shader targets
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(self.color),
            store: true,
          },
        })],
        depth_stencil_attachment: None,
      });
      // render_pipeline
      render_pass.set_pipeline(&self.render_pipeline);
      // draw something with 3 vertices, and 1 instance. This is where @builtin(vertex_index) comes from.
      render_pass.draw(0..3, 0..1);
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
