use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    color_buffer: Image,  // RAM
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(
            width as i32, //casting de u32 a i32 porque gen_image_color espera un i32
            height as i32,
            background_color,
        );

        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::CYAN,
        }
    }

    //borrar framebuffer por cada frame
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color
        );
    }

    //Cambiar color del siguiente pixel
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_background_color(&mut self, color: Color){
        self.background_color = color;
    }

    //pintar pixel
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    //pintar pixel sin cambiar de color
    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color){
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, color);
        }
    }

    // Usar buffer en la ventana
    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
       if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);    
            renderer.clear_background(Color::BLACK);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
            
        }
    }
}