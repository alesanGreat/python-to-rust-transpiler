use crate::types::DrawType;
use crate::types::State;

pub fn generate_rust(state: &State) -> String {
    let mut rust_code = String::new();

    // Importaciones
    rust_code.push_str("use sdl2::pixels::Color;\n");
    rust_code.push_str("use sdl2::event::Event;\n");
    rust_code.push_str("use sdl2::keyboard::Keycode;\n");
    rust_code.push_str("use sdl2::rect::Rect;\n");
    rust_code.push_str("use std::time::Duration;\n\n");

    // Constantes
    rust_code.push_str("const ANCHO: u32 = 800;\n");
    rust_code.push_str("const ALTO: u32 = 600;\n\n");

    // Función draw_circle corregida
    rust_code.push_str(r#"
fn draw_circle(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: Color, center: (i32, i32), radius: i32) {
    let (cx, cy) = center;
    canvas.set_draw_color(color);
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let _ = canvas.draw_point((cx + dx, cy + dy));
            }
        }
    }
}
"#);

    // Función main
    rust_code.push_str("\nfn main() -> Result<(), String> {\n");
    
    // Colores
    rust_code.push_str("    // Definición de colores\n");
    rust_code.push_str("    let _BLANCO: Color = Color::RGB(255, 255, 255);\n");
    rust_code.push_str("    let NEGRO: Color = Color::RGB(0, 0, 0);\n");
    rust_code.push_str("    let _ROJO: Color = Color::RGB(255, 0, 0);\n\n");
    


// Variables de usuario (evitando duplicados y variables sin inicializar)
rust_code.push_str("    // Variables de usuario\n");
let mut used_vars = std::collections::HashSet::new();
let skip_vars = vec!["window", "size", "renderer", "cy", "type", "rect", "center"];

for var in &state.variables {
    if !used_vars.contains(&var.name) && 
       !["ANCHO", "ALTO", "_BLANCO", "NEGRO", "_ROJO"].contains(&var.name.as_str()) &&
       !skip_vars.contains(&var.name.as_str()) {
        match var.tipo.as_str() {
            "bool" => {
                let bool_value = if var.value.contains("true") { "true" } else { "false" };
                rust_code.push_str(&format!("    let mut {} = {};\n", var.name, bool_value));
            },
            "tuple" => {
                if !var.value.contains("undefined") && !var.value.is_empty() {
                    // Extraer los valores numéricos de la tupla
                    let values: String = var.value
                        .chars()
                        .filter(|c| c.is_numeric() || *c == ',' || *c == ' ')
                        .collect();
                    rust_code.push_str(&format!("    let mut {} = {};\n", var.name, values));
                }
            },
            _ => {
                // Para valores numéricos y otros tipos
                if var.value.contains("Number(") {
                    let num = var.value
                        .replace("Number(", "")
                        .replace(")", "")
                        .trim()
                        .to_string();
                    rust_code.push_str(&format!("    let mut {} = {};\n", var.name, num));
                } else if var.value.contains("BinaryOp") {
                    // Manejar operaciones binarias
                    if var.name == "centro_x" {
                        rust_code.push_str(&format!("    let mut {} = (ANCHO / 2) as i32;\n", var.name));
                    } else if var.name == "centro_y" {
                        rust_code.push_str(&format!("    let mut {} = (ALTO / 2) as i32;\n", var.name));
                    }
                } else if !var.value.contains("undefined") && !var.value.contains("Ignore") {
                    let value = var.value.trim_matches('"');
                    rust_code.push_str(&format!("    let mut {} = {};\n", var.name, value));
                }
            }
        }
        used_vars.insert(var.name.clone());
    }
}

    rust_code.push_str("\n");

    // Inicialización de SDL2
    rust_code.push_str(r#"    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL Window", ANCHO, ALTO)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(NEGRO);
        canvas.clear();
"#);

    // Dibujar elementos
    for draw_call in &state.draw_calls {
        rust_code.push_str(&format!("        println!(\"Debug: Processing draw call: {{:?}}\", {:?});\n", draw_call));
        match draw_call.draw_type {
            DrawType::Rect => {
               rust_code.push_str(&format!(r#"
                {{
                   let rect_x = {};
                   let rect_y = {};
                   let rect_w = {};
                   let rect_h = {};
                   let rect = Rect::new(rect_x, rect_y, rect_w as u32, rect_h as u32);
                   let color = {};
                   canvas.set_draw_color(color);
                   canvas.fill_rect(rect)?;
                 }}
               "#,
                    draw_call.x, draw_call.y, 
                    draw_call.w.as_ref().unwrap_or(&"50".to_string()), 
                    draw_call.h.as_ref().unwrap_or(&"50".to_string()),
                    draw_call.color
                ));
            }
            DrawType::Circle => {
                rust_code.push_str(&format!(r#"
        draw_circle(&mut canvas, {}, ({}, {}), {});
"#,
                    draw_call.color, draw_call.x, draw_call.y, 
                    draw_call.radius.as_ref().unwrap_or(&"50".to_string())
                ));
            }
        }
    }
    
    // Final del bucle principal y función main
    rust_code.push_str(r#"
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
"#);
    
    rust_code
}