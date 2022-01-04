// src/ricker2d.rs
// Rust port of 'Program 8.7'.
use fdtd::fdtd::{FDTDSim, Grid};
use fdtd::ricker;

const SIZE_X: usize = 800;
const SIZE_Y: usize = 600;

use crow::{
    glutin::{
        dpi::LogicalSize,
        event::{ElementState, Event, VirtualKeyCode, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    target::Scaled,
    Context, DrawConfig, Texture,
};

fn float_to_rgb(v: f64) -> (f32, f32, f32) {
    (v as f32, v as f32, v as f32)
}

fn mat((r, g, b): (f32, f32, f32)) -> [[f32; 4]; 4] {
    [
        [r, 0.0, 0.0, 0.0],
        [0.0, g, 0.0, 0.0],
        [0.0, 0.0, b, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// Must be greater than 0.0;
const PPW: f64 = 20.0;

fn main() -> Result<(), crow::Error> {
    let cdtds = 1.0 / 2.0f64.sqrt();

    let ez_inc = move |t: usize, g: &mut Grid| {
        let loc = (SIZE_X / 2) * SIZE_Y + (SIZE_Y / 2);
        g.ez[loc] = ricker::ricker(t as f64, 0.0, cdtds, PPW);
    };

    let post_magnetic = move |_t: usize, _g: &mut Grid| {};

    let post_electric = move |t: usize, g: &mut Grid| {
        ez_inc(t, g);
    };

    let mut fdtd_sim = match FDTDSim::new_2d(
        SIZE_X,
        SIZE_Y,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(cdtds),
    ) {
        Ok(e) => e,
        Err(_) => panic!(),
    };

    fdtd_sim.set_post_magnetic(Some(post_magnetic));
    fdtd_sim.set_post_electric(Some(post_electric));

    let event_loop = EventLoop::new();
    let mut ctx = Context::new(
        WindowBuilder::new().with_inner_size(LogicalSize::new(800 as u32, 600 as u32)),
        &event_loop,
    )?;

    let mut texture = Texture::new(&mut ctx, (1, 1))?;
    ctx.clear_color(&mut texture, (1.0, 1.0, 1.0, 1.0));

    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed
                        && input.virtual_keycode == Some(VirtualKeyCode::Space)
                    {
                        fdtd_sim.step();
                    }
                }
                _ => (),
            },

            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                let mut surface = Scaled::new(ctx.surface(), (1, 1));
                ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0));
                for y in 0..fdtd_sim.g.y_sz {
                    for x in 0..fdtd_sim.g.x_sz {
                        let f = fdtd_sim.g.ez[x * fdtd_sim.g.y_sz + y];
                        let color_modulation = mat(float_to_rgb(f));
                        ctx.draw(
                            &mut surface,
                            &texture,
                            (x as i32, y as i32),
                            &DrawConfig {
                                color_modulation,
                                ..Default::default()
                            },
                        );
                    }
                }
                ctx.present(surface.into_inner()).unwrap();
            }
            _ => (),
        },
    );
}
