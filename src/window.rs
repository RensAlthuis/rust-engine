
extern crate winit;

use winit::{EventsLoop, Event, WindowEvent};

pub struct Window {
    pub window : winit::Window,
    events_loop : EventsLoop,
}

impl Window {
    pub fn new(title : &str) -> Window {
        let events_loop = winit::EventsLoop::new();
        let window = winit::WindowBuilder::new()
            .with_title(title)
            .build(&events_loop)
            .expect("Could not create window");
        ;
        Window {
            window,
            events_loop,
        }
    }

    pub fn poll_events(&mut self) -> bool {
        let mut running = true;
        self.events_loop.poll_events(| event | {
            match event {
                Event::WindowEvent { event:win_event , ..} => {
                    match win_event {
                        WindowEvent::Resized(winit::dpi::LogicalSize{width, height}) => {
                            println!("[INFO] Resize {}, {}", width, height);
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