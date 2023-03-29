use wgpu::{Device, RenderPipeline, SurfaceConfiguration};

pub fn render_pipe(
  device: &Device,
  config: &SurfaceConfiguration,
  shader_color: String,
) -> RenderPipeline {
  let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
  let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("Render Pipeline Layout"),
    bind_group_layouts: &[],
    push_constant_ranges: &[],
  });

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
  render_pipeline
}
