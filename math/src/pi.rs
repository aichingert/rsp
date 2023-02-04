use crate::random::Rand;

pub fn approximate_pi(precision: u32) -> f32 {
    let mut rand = Rand::new(9.32).into_iter();
    let mut p_circle: f32 = 0.;
    let mut p_total: f32 = 0.;

    for _ in 0..precision {
        let x: f32 = rand.next().unwrap();
        let y: f32 = rand.next().unwrap();

        if x*x + y*y <= 1. {
            p_circle += 1.;
        }
        p_total += 1.;
    }

    // Pi
    4.0 * (p_circle / p_total)
}
