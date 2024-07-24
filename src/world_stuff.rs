use std::{collections::HashMap, fs, io::{self}};
use macroquad::prelude::*;
use rand::srand;

pub const CHUNKSIZE:usize = 1024;


pub struct ChunkWithOtherInfo{
    position: IVec2,
    chunk: [BlockType; CHUNKSIZE],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockType{
    Air,
    Plank,
    Dirt,
    Stone,
    Grass,
}

pub fn readchunkfile(position: IVec2) -> ChunkWithOtherInfo{

    let mut chunk:[BlockType; CHUNKSIZE] = [BlockType::Air; CHUNKSIZE];

    let file_to_read_from = "Chunks/x".to_string() + &position.x.to_string() + "y" + &position.y.to_string();

    let chunkstring = match fs::read_to_string(file_to_read_from){
        Ok(chunkstring) => chunkstring,
        Err(err) => {
            match err.kind() {
                io::ErrorKind::NotFound => {
                    //eprintln!("{} most likely not generated chunks at {} yet", err, position); 
                    return ChunkWithOtherInfo{position: position, chunk: generate_chunk(6, position)};}
                _ => {
                    eprintln!("{} at chunk {}", err, position); 
                    return ChunkWithOtherInfo{position: position, chunk: [BlockType::Air; 1024]};}
            }
        }
    };
    
    let tokens: Vec<&str> = chunkstring.split("+").collect();

    assert_eq!(tokens.len(), 1024);

    for (iteration, &item) in tokens.iter().enumerate(){
        let blocksenum = match item {
            "Air" => BlockType::Air,
            "Plank" => BlockType::Plank,
            "Dirt" => BlockType::Dirt,
            "Stone" => BlockType::Stone,
            "Grass" => BlockType::Grass,
            _ => {eprintln!("could not read the block. Chunk:{} Iter:{}",position , iteration); BlockType::Air},
        };

        chunk[iteration] = blocksenum;
    }

    return ChunkWithOtherInfo{position: position, chunk: chunk};
}

//make functio to generate a world based on what i want, insert in readchunkfile when error gives no file. YES DONE FINALY(not that complicated)

fn generate_chunk(seed:i32, position: IVec2) -> [BlockType; CHUNKSIZE] {
    if position.y < 0{ return [BlockType::Air; CHUNKSIZE]};
    let mut chunk = [BlockType::Air; CHUNKSIZE];
    srand((seed as i128 + position.x as i128 + position.y as i128) as u64);

    let randomnumber = rand::gen_range(0, 100);

    for iter in 0..1024{
        let x:i32 = iter as i32%32;
        let y:i32 = iter as i32/32;
        let chunk_x:f32 = x as f32/32.0 + position.x as f32;
        let chunk_y:f32 = y as f32/32.0 + position.y as f32;

        let sinex:f32 = f32::sin((chunk_x*3.0) + randomnumber as f32/100.);


        if y as f32 > (sinex+1.)*8.0{
            chunk[iter] = BlockType::Stone
        }

    }
    
    chunk
}

pub fn writechunkfile(chunk_info: ChunkWithOtherInfo ){
    let mut chunkstring = "".to_string();

    for i in 0..CHUNKSIZE{
        let blockstring = match chunk_info.chunk[i] {
            BlockType::Air => "Air",
            BlockType::Plank => "Plank",
            BlockType::Dirt => "Dirt",
            BlockType::Stone => "Stone",
            BlockType::Grass => "Grass",
        };

        if i != 1023{
            chunkstring.push_str(&(blockstring.to_owned() + "+"));
            continue;
        }  
        chunkstring.push_str(blockstring)    
    }

    let file_to_write_to = "Chunks/x".to_string() + &chunk_info.position.x.to_string() + "y" + &chunk_info.position.y.to_string();

    match fs::write(file_to_write_to, chunkstring){
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}

pub fn chunks_in_view_manager(camera: Camera2D, chunks_in_view: &mut HashMap<IVec2,[BlockType; CHUNKSIZE]>){
    let search_rectangle = Rect{
		x: ((camera.target.x - 1.0/camera.zoom.x)/32.).floor(),
		y: ((camera.target.y - 1.0/camera.zoom.y)/32.).floor(),
		w: ((camera.target.x + 1.0/camera.zoom.x)/32.).ceil() - ((camera.target.x - 1.0/camera.zoom.x)/32.).floor(),
		h: ((camera.target.y + 1.0/camera.zoom.y)/32.).ceil() - ((camera.target.y - 1.0/camera.zoom.y)/32.).floor(),
	};
    //draw_rectangle(search_rectangle.x, search_rectangle.y, search_rectangle.w, search_rectangle.h, RED);

    let area:usize =(search_rectangle.w * search_rectangle.h).ceil() as usize; 

    let mut chunktoremove = chunks_in_view.clone();

    for i in 0..area{
		let x:i32 = (i as i32%search_rectangle.w as i32) + search_rectangle.x as i32;
		let y:i32 = i as i32 /search_rectangle.w as i32 + search_rectangle.y as i32;
        chunktoremove.remove(&IVec2{x: x, y: y});

        match chunks_in_view.get(&IVec2{x: x, y: y}){
            Some(_)=> {}
            None => {
                chunks_in_view.insert(IVec2{x: x, y: y}, readchunkfile(IVec2{x: x, y: y}).chunk);

            }
        }
    }
    for (key, chunk) in chunktoremove{
        writechunkfile(ChunkWithOtherInfo{position: key, chunk:chunk});
        chunks_in_view.remove(&key);
    }

}

pub fn draw_world_color_only(chunks_in_view: &HashMap<IVec2, [BlockType; 1024]>){
    for chunkinfo in chunks_in_view{
        for index in 0..1024{
            let blockcolor:Color = match chunkinfo.1[index] {
                BlockType::Air => Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0},
                BlockType::Plank => Color { r: 0.5, g: 0.4, b: 0.0, a: 1.0},
                BlockType::Dirt => Color { r: 0.5, g: 0.5, b: 0.1, a: 1.0},
                BlockType::Stone => Color { r: 0.4, g: 0.4, b: 0.45, a: 1.0},
                BlockType::Grass => Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0},
            };

            let x:i32 = index as i32%32;
            let y:i32 = index as i32/32;
            
            draw_rectangle((chunkinfo.0.x*32) as f32 + x as f32, (chunkinfo.0.y*32) as f32 + y as f32, 1.0, 1.0, blockcolor);
        }
        
    }
}