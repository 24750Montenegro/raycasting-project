mod framebuffer;
use framebuffer::Framebuffer;
use raylib::prelude::*;


fn main() {
    //tamaño de la ventana
    let window_width = 1200;
    let window_height = 800;

    //iniciar raylib
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster")
        .build();

    let mut framebuffer = Framebuffer::new(
        window_width as u32, 
        window_height as u32, 
        Color::new(50, 50, 100, 255)
    );

    window.set_target_fps(60);

    while !window.window_should_close() {
        //limpiar framebuffer
        framebuffer.clear();

        //dibujo de pruba
        framebuffer.set_current_color(Color::RED);
        for i in 0..300u32 {
            framebuffer.set_pixel(i, i)
        }

        //mostrar
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
