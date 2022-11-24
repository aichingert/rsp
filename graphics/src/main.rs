mod voronoi;

use voronoi::*;

fn main() {
    let mut voronoi: Voronoi = Voronoi::new();

    voronoi.save_image("name.ppm");
}
