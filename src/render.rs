use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

//llenar celdas
fn cell_color(cell: char) -> Color {
    match cell{
        '+' => Color::BLUE,
        '-' => Color::YELLOW,
        '|' => Color::GRAY,
        'g' => Color::RED,
        '#' => Color::BLACK,
        _ => Color::WHITE,   
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell:char) {
    let color = cell_color(cell);
    framebuffer.set_current_color(color);

    for y in yo..(yo + block_size) {
        for x in xo..(xo + block_size) {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize, player: &Player) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    // Dibujar jugador
    framebuffer.set_current_color(Color::CYAN);
    let px = player.pos.x as i32;
    let py = player.pos.y as i32;

    //dibujar circulo
    let dr: i32 = 6; // radio del circulo
    for dy in -dr..=dr {
        for dx in -dr..=dr {
            if dx * dx + dy * dy > dr * dr {
                continue; // fuera del circulo
            }
            let x = px + dx;
            let y = py + dy;
            if x >= 0 && y >= 0 {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}
