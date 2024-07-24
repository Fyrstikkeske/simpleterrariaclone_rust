use std::collections::HashMap;
use macroquad::prelude::*;
use world_stuff::{BlockType, CHUNKSIZE};

mod world_stuff;



#[macroquad::main("Fyrraria mathch")]
async fn main() {
    println!("not so Hello, World!");

    let compacta_font = load_ttf_font("Assets/compacta.ttf")
        .await
        .unwrap();
    
                // FUCK TIS, i will it the more proper way with names and shit, performance what is that, at least it will work... done :thumbsup:

    let mut zoom:f32 = 32.0;
    //writechunkfile(ChunkWithOtherInfo{position: IVec2 { x: 0, y: 0 }, chunk: chunk});
    
    let mut camera_zoom = Vec2{x:1./10.0, y:1./10.0};
    let mut camera_target = Vec2{x:0.0, y:0.0};

    let mut chunks_in_view:HashMap<IVec2,[BlockType; CHUNKSIZE]> = HashMap::new();

loop{
    //stuff that does stuff here

    camera_zoom.y = 1./screen_height();
    camera_zoom.x = 1./screen_width();

    camera_zoom *= zoom;

    let camera = Camera2D {
        zoom: camera_zoom,
        target: camera_target,
        ..Default::default()
    };

    camera_target.x += 0.1;

    let mouse_pos = camera.screen_to_world( mouse_position().into());



    //AHHHH
    clear_background(BLACK);
    set_camera(&camera);
    
    
    world_stuff::chunks_in_view_manager(camera, &mut chunks_in_view);

    //time to draw all those chunks ive summoned REALREALREALREALRELARLEALREALREALREAREARLEARLEARLEALREAL

    world_stuff::draw_world_color_only(&chunks_in_view);

    //ballsexian
    set_default_camera();

    draw_fps(&compacta_font);

    next_frame().await
    }
}



fn draw_fps(compacta_font:&Font){
    draw_text_ex(
        format!("{}", get_fps()).as_str(),
        20.0,
        30.0,
        TextParams {
            font_size: 30,
            font: Some(compacta_font),
            ..Default::default()}
        );

}