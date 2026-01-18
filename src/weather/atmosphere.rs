use bevy::math::Vec3;

use crate::Terrain;
use crate::weather::fluid::{currents, fluid, velocities};
use crate::weather::temperatures::temperature;

pub fn atmosphere(
    points: &Vec<Vec3>,
    half_edges: &Vec<usize>,
    edges: &Vec<Vec<usize>>,
    adjacencies: &Vec<Vec<usize>>,
    terrain: &Vec<Terrain>,
    temperatures: &Vec<f32>,
    iterations: usize,
) -> Vec<f32> {
    let weights = vec![1.0].repeat(terrain.len());
    let starting_velocities = vec![Vec3::ZERO].repeat(terrain.len());
    let starting_pressures = (0..points.len()).map(|p| {
        let temperature = temperatures[p] - 0.5 * temperature(points[p].y);
        let ideal = pressure(points[p].y);
        if ideal.signum() == temperature.signum() {
            0.1 * ideal
        } else {
            ideal * temperature.abs()
        }
    }).collect();

    let starting_currents = fluid(
        points, 
        half_edges, 
        edges, 
        adjacencies, 
        &weights, 
        starting_velocities, 
        &starting_pressures, 
        iterations,
    );
    let mut velocities = velocities(
        points, 
        edges, 
        adjacencies, 
        &starting_currents,
    );

    for p in 0..points.len() {
        velocities[p] = velocities[p].rotate_axis(points[p], -points[p].y.signum());
    }

    return currents(
        points, 
        half_edges, 
        edges, 
        adjacencies, 
        &weights, 
        &velocities,
    );
}

fn pressure(y: f32) -> f32 {
    32.0*y*y*y*y*y*y - 48.0*y*y*y*y + 18.0*y*y - 1.0
}