#![allow(dead_code)]

use wasm_bindgen::prelude::*;
mod console;
mod error;
mod gl;
mod js_loop;
use gl::*;
use js_loop::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    error::report(setup());
}

fn setup() -> Result<(), String> {
    let gl = Gl::init()?;

    let vertices = [
        -0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, -0.5, -0.5, 0.5,
    ];

    let mut mesh = gl.create_mesh(&vertices)?;

    create_draw_loop(move |time| {
        gl.clear_canvas(&[0., 0., 0., 1.]);

        let start = get_current_time()?;

        let margin = 0.95;

        let width = 200;
        let height = 200;
        let half_width = (width as f32 / 2. - 0.5) / margin;
        let half_height = (height as f32 / 2. - 0.5) / margin;

        for x in 0..width {
            for y in 0..height {
                mesh.create_instance(
                    InstanceProperties::new().position(&[
                        x as f32 / half_width - margin,
                        y as f32 / half_height - margin,
                        0.5,
                    ]).
                    color(&[
                        (5. * time.seconds + x as f32 / 8. + y as f32 / 9. + 1.).sin(),
                        // 0.,
                        (10. * time.seconds - x as f32 / 10. + y as f32 / 10.).sin(),
                        -(-6. * time.seconds + x as f32 / 10. + y as f32 / 4.).sin(),
                        // 1.,
                        1.,
                    ])
                    .matrix(&[0.005, 0.0, 0.0, 0.005])
                );
            }
        }

        let after_draw = get_current_time()?;

        mesh.draw();

        if time.frame_count % 60 == 0 {
            console::log!(
                "render: {}ms      setup: {}ms      draw: {}ms",
                time.render_average * 1000.,
                (after_draw - start) * 1000.,
                (get_current_time()? - after_draw) * 1000.,
            );
        }
        Ok(())
    })?;

    Ok(())
}
