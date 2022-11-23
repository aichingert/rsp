mod voronoi;

use voronoi::*;

fn main() {
    let mut voronoi: Voronoi = Voronoi::new();

    voronoi.create_image("name.ppm");
}
