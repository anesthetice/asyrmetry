use angle_sc::{self, Angle, Radians};
use core::f32::consts::E;
use core::f64::consts::PI;
use dasp::Signal;
use itertools::Itertools;

type Meters = f32;
type MetersPerSeconds = f32;

const DISTANCE_BETWEEN_L_AND_R: Meters = 0.2;
const SPEED_OF_SOUND_IN_AIR: MetersPerSeconds = 343.0;

pub fn process_signal(
    raw_signal: Vec<f32>,
    sample_rate: u32,
) -> eyre::Result<(Vec<f32>, Vec<f32>, Vec<i32>)> {
    let mut sound_source = Coord2D::new(1.4, PI / 2.2);
    let left = Coord2D::new(0.4, PI);
    let right = Coord2D::new(0.4, 0.0);

    let period: f64 = 5.0;
    let radial_pulse = 2.0 * PI / period;
    let radial_step = radial_pulse / sample_rate as f64;
    Ok(raw_signal
        .into_iter()
        .map(|amp| {
            sound_source.1 = sound_source.1 + Angle::from(Radians(radial_step));

            let left_dist = left.dist(&sound_source);
            let right_dist = right.dist(&sound_source);
            let delta_t = (left_dist - right_dist) / SPEED_OF_SOUND_IN_AIR;
            (
                amp * E.powf(-left_dist),
                amp * E.powf(-right_dist),
                (delta_t * sample_rate as f32) as i32,
            )
        })
        .multiunzip::<(Vec<f32>, Vec<f32>, Vec<i32>)>())
}

#[derive(Debug)]
struct Coord2D(Meters, Angle);

impl Coord2D {
    pub fn new(radius: f32, radians: f64) -> Self {
        Self(radius, Angle::from(Radians(radians)))
    }

    pub fn dist(&self, other: &Coord2D) -> Meters {
        (self.0.powi(2) + other.0.powi(2)
            - 2.0 * self.0 * other.0 * (other.1 - self.1).cos().0 as f32)
            .sqrt()
    }
}
