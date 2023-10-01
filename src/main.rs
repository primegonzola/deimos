// SPDX-License-Identifier: MIT

#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod app;

#[rustfmt::skip]
fn main() -> Result<()> {

    // initialize logger
    pretty_env_logger::init();

    // create window event loop
    let event_loop = EventLoop::new();

    // create window with title and size, and event loop
    let window = WindowBuilder::new()
        .with_title("D E I M O S")
        .with_inner_size(LogicalSize::new(640, 480))
        .build(&event_loop)?;

    // assume not destroying and not minimized
    let mut minimized = false;
    let mut destroying = false;
    
    // create app
    let mut app = unsafe { app::App::create(&window)? };
    
    // run event loop until destroying
    event_loop.run(move |event, _, control_flow| {

        // the flow assumes polling
        *control_flow = ControlFlow::Poll;

        // check event
        match event {

            // update app if is not being destroyed.
            Event::MainEventsCleared if !destroying && !minimized => unsafe { app.update(&window) }.unwrap(),

            // mark the window as having been resized.
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                if size.width == 0 || size.height == 0 {
                    
                    // is minimized
                    minimized = true;

                } else {
                    
                    // is not minimized
                    minimized = false;

                    // mark window as being resized
                    // app.graphics.resized = true;
                }
            }
            
            // check if close is being requested
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {

                // enter destroying mode
                destroying = true;

                // mark control flow as exit
                *control_flow = ControlFlow::Exit;

                // destroy the app
                unsafe { app.destroy(); }
            }

            // handle keyboard events.
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {

                // check if pressed
                if input.state == ElementState::Pressed {

                    // check key code 
                    // match input.virtual_keycode {
                    //     Some(VirtualKeyCode::Left) if app.data.models > 1 => app.data.models -= 1,
                    //     Some(VirtualKeyCode::Right) if app.data.models < 4 => app.data.models += 1,
                    //     _ => { }
                    // }
                }
            }
            _ => {}
        }
    });
}
