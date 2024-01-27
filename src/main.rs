mod maze;

use std::collections::HashSet;

use maze::{new_maze, random_depth_first};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use crate::maze::EdgeNode;

const MAZE_SIZE: (u32, u32) = (800, 800);
const SQUARE_SIZE: u32 = 32;
const INITIAL: u32 = 0;

fn main () -> Result<(), String> {
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Game of Life",
            MAZE_SIZE.0,
            MAZE_SIZE.1,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut event_pump = sdl_context.event_pump()?;

    let mut maze = new_maze(SQUARE_SIZE);
    random_depth_first(&mut maze, INITIAL);

    let start = INITIAL;
    let mut stack = Vec::new();

    stack.push(start);
    let mut visited: HashSet<u32> = HashSet::new();

    while let Some(vertex) = stack.pop() {

        if let Some(edges) = maze.adjacency.get(&vertex) {

            'edges: for edge in edges {

                match edge.1 {
                    EdgeNode::Some => {

                        if !visited.insert(edge.0) {
                            break 'edges;
                        }

                        stack.push(edge.0);

                        let next_x = edge.0 % SQUARE_SIZE;
                        let next_y = edge.0 / SQUARE_SIZE;
                        let next_x = (next_x * (MAZE_SIZE.0 / SQUARE_SIZE)) + ((MAZE_SIZE.0 / SQUARE_SIZE) / 2);
                        let next_y =(next_y * (MAZE_SIZE.1 / SQUARE_SIZE)) + ((MAZE_SIZE.1 / SQUARE_SIZE) / 2);

                        let x = vertex % SQUARE_SIZE;
                        let y = vertex / SQUARE_SIZE;
                        let x = (x * (MAZE_SIZE.0 / SQUARE_SIZE)) + ((MAZE_SIZE.0 / SQUARE_SIZE) / 2);
                        let y =(y * (MAZE_SIZE.1 / SQUARE_SIZE)) + ((MAZE_SIZE.1 / SQUARE_SIZE) / 2);

                        let center_x = (next_x + x) / 2;
                        let center_y = (next_y + y) / 2;

                        let w: u32;
                        let h: u32;

                        if center_x != x {
                            
                            w = MAZE_SIZE.0 / (SQUARE_SIZE * 2);
                            h = MAZE_SIZE.1 / (SQUARE_SIZE * 6);
                        } else {
                            w = MAZE_SIZE.0 / (SQUARE_SIZE * 6);
                            h = MAZE_SIZE.1 / (SQUARE_SIZE * 2);
                        }

                        canvas.fill_rect(Rect::from_center((center_x as i32, center_y as i32), w + 1, h + 1))?;

                    },
                    EdgeNode::None => (),
                }

            }

        }
        
    }

    for y in 0..SQUARE_SIZE {
        for x in 0..SQUARE_SIZE {

            let x = (x * (MAZE_SIZE.0 / SQUARE_SIZE)) + ((MAZE_SIZE.0 / SQUARE_SIZE) / 4);
            let y =(y * (MAZE_SIZE.1 / SQUARE_SIZE)) + ((MAZE_SIZE.1 / SQUARE_SIZE) / 4);

            let w = MAZE_SIZE.0 / (SQUARE_SIZE * 2);
            let h = MAZE_SIZE.1 / (SQUARE_SIZE * 2);

            canvas.fill_rect(Rect::new(x as i32, y as i32, w, h))?;
        }
    }

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    println!("space pressed")
                },
                _ => (),
            }
        }
        
        canvas.present();
    }

    println!("quit game");

    Ok(())

}