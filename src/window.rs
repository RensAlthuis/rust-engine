
extern crate winit;

use winit::{EventsLoop, Event, WindowEvent};
use winit::dpi::LogicalSize;

pub struct Window {
    pub window : winit::Window,
    events_loop : EventsLoop,
    pub extent : LogicalSize
}

impl Window {
    pub fn new(title : &str) -> Window {
        let events_loop = winit::EventsLoop::new();
        let window = winit::WindowBuilder::new()
            .with_title(title)
            .build(&events_loop)
            .expect("Could not create window");
        ;

        let extent = window.get_inner_size().unwrap();

        Window {
            window,
            events_loop,
            extent,
        }
    }

    pub fn poll_events(&mut self) -> bool {
        let mut running = true;
        let mut e = self.extent;
        self.events_loop.poll_events(| event | {
            match event {
                Event::WindowEvent { event:win_event , ..} => {
                    match win_event {
                        WindowEvent::Resized(extent) => {
                            println!("[INFO] Resize {}, {}", extent.width, extent.height);
                            e = extent;
                        },

                        WindowEvent::CloseRequested => {
                            running = false;
                        },

                        _ => ()
                    }
                },

                _ => ()
            }
        });

        running
    }

}