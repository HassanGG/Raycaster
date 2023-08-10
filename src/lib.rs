#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod game;
mod gpu;
mod graphics;
mod player;
mod ray;
mod util;
mod vertex;

use gpu::WGPUState;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::graphics::Graphics;

const WINDOW_SIZE: winit::dpi::PhysicalSize<i32> = winit::dpi::PhysicalSize::new(1400, 700);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Raycaster")
        .with_inner_size(WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("canvas")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
    }

    let state = WGPUState::new(window).await;
    let graphics = Graphics::new(state);
    let mut game = game::Game::new(graphics);

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == game.graphics.gpu_state.window().id() => {
            match game.update() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => {
                    game.graphics.gpu_state.resize(game.graphics.gpu_state.size)
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            game.graphics.gpu_state.window().request_redraw();
        }

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == game.graphics.gpu_state.window().id() => {
            if !game.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        game.graphics.gpu_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        game.graphics.gpu_state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
