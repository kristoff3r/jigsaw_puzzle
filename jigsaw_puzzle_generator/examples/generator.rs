use env_logger::{Builder, Env};
use jigsaw_puzzle_generator::JigsawGenerator;
use std::env;
use std::fs::create_dir_all;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    let env = Env::default();
    Builder::from_env(env).format_timestamp_millis().init();
    let image_path = env::args().nth(1).unwrap_or("raw.jpg".to_string());
    let template = JigsawGenerator::from_path(&image_path, 9, 6)
        .generate()
        .expect("Failed to generate puzzle");
    create_dir_all("images").expect("Failed to create images directory");
    template
        .origin_image
        .save("images/origin_image.png")
        .expect("Failed to save image");

    for piece in template.pieces.iter() {
        template
            .crop(piece)
            .save(format!("images/puzzle_piece_{}.png", piece.index))
            .expect("Failed to save image");
    }
}