use std::sync::Arc;
use wgpu::{Backends, DeviceDescriptor, Features, InstanceDescriptor, PowerPreference};
use wgpu::{Device, Queue, Surface, TextureFormat};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use crate::misc::Vec2;

pub struct InnerEngine {
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    surface: Arc<Surface<'static>>,
    texture_format: TextureFormat,
    event_loop: EventLoop<()>
}

impl InnerEngine {
    pub async fn new(inner_size: Vec2, title: String) -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(
                inner_size.x, inner_size.y
            ))
            .with_title(title)
            .build(&event_loop)
            .unwrap();


        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let window_arc = Arc::new(window);

        let surface = instance.create_surface(window_arc.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let texture_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        Self {
            texture_format,
            event_loop,
            window: window_arc,
            surface: Arc::new(surface),
            device: Arc::new(device),
            queue: Arc::new(queue),
        }
    }

    pub fn run(self) {
        let ev = self.event_loop;

        ev.run(move |event, control_flow| {
            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { event, window_id } if self.window.id() == window_id => {
                    match event {
                        WindowEvent::ActivationTokenDone { .. } => {}
                        WindowEvent::Resized(_) => {}
                        WindowEvent::Moved(_) => {}
                        WindowEvent::CloseRequested => {
                            control_flow.exit();
                        }
                        WindowEvent::Destroyed => {}
                        WindowEvent::DroppedFile(_) => {}
                        WindowEvent::HoveredFile(_) => {}
                        WindowEvent::HoveredFileCancelled => {}
                        WindowEvent::Focused(_) => {}
                        WindowEvent::KeyboardInput { .. } => {}
                        WindowEvent::ModifiersChanged(_) => {}
                        WindowEvent::Ime(_) => {}
                        WindowEvent::CursorMoved { .. } => {}
                        WindowEvent::CursorEntered { .. } => {}
                        WindowEvent::CursorLeft { .. } => {}
                        WindowEvent::MouseWheel { .. } => {}
                        WindowEvent::MouseInput { .. } => {}
                        WindowEvent::TouchpadMagnify { .. } => {}
                        WindowEvent::SmartMagnify { .. } => {}
                        WindowEvent::TouchpadRotate { .. } => {}
                        WindowEvent::TouchpadPressure { .. } => {}
                        WindowEvent::AxisMotion { .. } => {}
                        WindowEvent::Touch(_) => {}
                        WindowEvent::ScaleFactorChanged { .. } => {}
                        WindowEvent::ThemeChanged(_) => {}
                        WindowEvent::Occluded(_) => {}
                        WindowEvent::RedrawRequested => {}
                    }
                }
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::AboutToWait => {}
                Event::LoopExiting => {}
                Event::MemoryWarning => {},
                Event::WindowEvent { .. } => {}
            }
        }).unwrap()
    }
}