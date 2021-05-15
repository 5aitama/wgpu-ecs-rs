pub mod components;

use std::ops::Deref;

use ecs_rust::core::{
    managers::{
        entity_manager::EntityManager, 
        component_manager::ComponentManager, 
        system_manager::SystemManager,
        system_manager::Sys
    },
    entity_query::EntityQuery,
    world::World
};
use futures::executor::block_on;

use crate::components::{tags::Cube, xyz::XYZ};

use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};


struct MySystem;

impl Sys for MySystem {

    fn run(&self, world: &mut World) {
        // Query entities that must have XYZ & Cube components.
        let entities = <(XYZ, Cube)>::all(world);

        // If there is no entities, create one, and add it the required components.
        if entities.len() == 0 {
            let entity = EntityManager::create_entity(world).unwrap();

            ComponentManager::add_component(XYZ::new(0, 1, 2), entity, world);
            ComponentManager::add_component(Cube, entity, world);

            return;
        }

        for entity in entities {
            // Modify the component...
            let xyz = ComponentManager::get_component_mut::<XYZ>(entity, world).unwrap();
            xyz.set_y(4);

            // Print it!
            println!("xyz: {:?}", xyz);
        }
    }
    
}
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            }, None
        ).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let color = wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        
        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline =device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        });

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            color,
            render_pipeline,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.color),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });

            _render_pass.set_pipeline(&self.render_pipeline);
            _render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

fn main() {
    
    // let mut world = World::new(2, 1);
    // let mut sys_manager = SystemManager::new(1);

    // sys_manager.register(MySystem);

    // loop {
    //     sys_manager.run(&mut world);
    // }

    env_logger::init();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let mut state = block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        }
        
        if window_id == window.id() => if !state.input(event) { 
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { 
                    input, 
                    .. 
                } => {
                    match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    }
                },

                WindowEvent::CursorMoved {
                    position,
                    ..
                } => {
                    let r = position.x / state.size.width as f64;
                    let g = position.y / state.size.height as f64;
                    let b = r;
                    let a = 1.0;

                    state.color = wgpu::Color { r, g, b, a };
                },

                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                },


                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                },

                _ => {}
            }
        }

        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprint!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            window.request_redraw();
        }

        _ => {}
    });
}