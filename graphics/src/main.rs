mod voronoi;

use voronoi::*;

fn main() {
    let mut voronoi: Voronoi = Voronoi::new();

    voronoi.render_voronoi();
    voronoi.fill_circle(5, 0xFF000000);
    voronoi.save_image("../image/image.ppm");
}
