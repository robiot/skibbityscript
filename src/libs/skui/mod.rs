use super::{get_lib_state, LibFunctions, LibState, Library};
use crate::{error, interpreter::Interpreter, parser::Expr};

use std::collections::HashMap;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

pub mod clock;
pub mod window;

pub const LIBRARY_NAME: &str = "skui";

#[derive(Debug)]
struct WindowInfo {
    width: u32,
    height: u32,
    title: String,
}

#[derive(Default, Debug)]
pub struct SkuiApp {
    window: Option<Window>,
    window_info: Option<WindowInfo>,

    // Stuff
    active_event: Option<WindowEvent>,
}

#[derive(Default, Debug)]
pub struct SkuiState {
    app: Option<SkuiApp>,
    event_loop: Option<EventLoop<()>>,
    clock: Option<Duration>,
}

// https://www.reddit.com/r/rust/comments/1dnaase/rust_and_winit_0303/
pub fn load_skui_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("createWindow".to_string(), window::create_window_builtin);
    functions.insert("pumpEvents".to_string(), window::pump_events_builtin);
    functions.insert("clockTick".to_string(), clock::clock_tick_builtin);

    Library {
        functions,
        state: LibState::SkuiState(SkuiState {
            app: None,
            event_loop: None,
            clock: None,
        }),
    }
}

pub fn load_skui_state(itp: &mut Interpreter) -> Result<&mut SkuiState, error::ParseError> {
    let line = itp.line;
    let libstate = get_lib_state(itp, LIBRARY_NAME);

    let state = if let LibState::SkuiState(state) = libstate {
        state
    } else {
        return Err(error::ParseError::GeneralError {
            line,
            message: "Invalid state".to_string(),
        });
    };

    Ok(state)
}

//
// The state of the skui/ winit wrapper
//
impl SkuiApp {
    fn new(window_info: WindowInfo) -> Self {
        SkuiApp {
            window: None,
            window_info: Some(window_info),
            active_event: None,
        }
    }

    fn get_event(&mut self) -> Expr {
        let event = self.active_event.take().unwrap();

        let event_code = match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                // event_loop.exit();

                "close_requested"
            }
            // WindowEvent::RedrawRequested => {
            //     // Redraw the application.
            //     //
            //     // It's preferable for applications that do not render continuously to render in
            //     // this event rather than in AboutToWait, since rendering in here allows
            //     // the program to gracefully handle redraws requested by the OS.

            //     // Draw.

            //     // Queue a RedrawRequested event.
            //     //
            //     // You only need to call this if you've determined that you need to redraw in
            //     // applications which do not always need to. Applications that redraw continuously
            //     // can render here instead.
            //     // self.window.as_ref().unwrap().request_redraw();
            // }
            _ => "",
        };

        Expr::StringLiteral(event_code.to_string())
    }
}

impl ApplicationHandler for SkuiApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_info = self.window_info.as_ref().unwrap();

        let window_attributes = Window::default_attributes()
            .with_inner_size(winit::dpi::LogicalSize::new(
                window_info.width,
                window_info.height,
            ))
            .with_title(window_info.title.clone()); // Set the window title

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.active_event = Some(event.clone());

        println!("frame");
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            // WindowEvent::RedrawRequested => {
            //     // Redraw the application.
            //     //
            //     // It's preferable for applications that do not render continuously to render in
            //     // this event rather than in AboutToWait, since rendering in here allows
            //     // the program to gracefully handle redraws requested by the OS.

            //     // Draw.

            //     // Queue a RedrawRequested event.
            //     //
            //     // You only need to call this if you've determined that you need to redraw in
            //     // applications which do not always need to. Applications that redraw continuously
            //     // can render here instead.
            //     self.window.as_ref().unwrap().request_redraw();
            // }
            _ => (),
        }
    }
}