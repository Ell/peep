use std::collections::HashMap;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder, WindowId};

use crate::renderer::Renderer;

#[derive(Debug)]
pub struct Application {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Window>,
    main_window: Window,
    renderer: Renderer,
}

impl Application {
    pub async fn create() -> Self {
        let event_loop = EventLoop::new();
        let main_window = WindowBuilder::new().build(&event_loop).unwrap();

        let windows = HashMap::new();

        let renderer = Renderer::new(&main_window).await;

        Self {
            event_loop,
            windows,
            main_window,
            renderer,
        }
    }

    pub fn run(self) {
        let event_loop = self.event_loop;
        let main_window = self.main_window;
        let mut renderer = self.renderer;

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == main_window.id() && !renderer.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            renderer.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                renderer.update();

                match renderer.render() {
                    Ok(_) => {}
                    Err(wgpu::SwapChainError::Lost) => renderer.resize(renderer.size),
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                main_window.request_redraw();
            }
            _ => {}
        });
    }
}
