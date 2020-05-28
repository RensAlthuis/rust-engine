
extern crate winit;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit::dpi::PhysicalSize;
use winit::window::WindowBuilder;

pub struct Window {
    pub window : winit::window::Window,
    events_loop : Option<EventLoop<()>>,
    pub extent : PhysicalSize<u32>
}

impl Window {
    pub fn new(title : &str) -> Window {
        let events_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&events_loop)
            .expect("Could not create window");

        let extent = window.inner_size();
        let events_loop = Some(events_loop);
        Window {
            window,
            events_loop,
            extent,
        }
    }

    pub fn run<F>(&mut self, main_loop : F )
    where F: 'static + Fn() {
        let mut e = self.extent;
        if let Some(events_loop) = self.events_loop.take() {

            events_loop.run(move | event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
                match event {
                    Event::WindowEvent { event:win_event , ..} => {
                        match win_event {
                            WindowEvent::Resized(extent) => {
                                println!("[INFO] Resize {}, {}", extent.width, extent.height);
                                e = extent;
                            },

                            WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                            },

                            _ => ()
                        }
                    },

                    _ => ()
                }
                main_loop();
            });

        };

    }

}