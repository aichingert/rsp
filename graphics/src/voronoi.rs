use std::path::Path;
use std::fs::File;

const SEEDS_COUNT: usize = 10;
const HEIGHT: usize = 800;
const WIDTH: usize = 600;

pub struct Voronoi {
    seeds: [u16; SEEDS_COUNT],
    image: Vec<Vec<u32>>
}
    
impl Voronoi {
    pub fn new() -> Self {
        Self {
            seeds: [0;SEEDS_COUNT],
            image: vec![vec![0xFF0000FF; WIDTH]; HEIGHT]
        }
    }

    pub fn create_image<T: AsRef<Path> + std::fmt::Display>(&self, path: T) {
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't create file: [{}] because: [{}]", path, err)
        };
    }
}
