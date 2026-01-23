use bevy::math::Vec3;

use crate::terrain::Terrain;
use crate::weather::advection::advection;

const LAND_TEMPERATURE: f32 = 0.5;
const LAND_VARIATION: f32 = 0.5;

pub fn temperature(
    y: f32,
) -> f32 {
    1.0 - 4.0*y*y + 2.0*y*y*y*y
}

pub fn temperatures(
    points: &Vec<Vec3>,
    terrain: &Vec<Terrain>,
) -> Vec<f32> {
    (0..points.len()).map(|p| {
        let temperature = temperature(points[p].y);
        match terrain[p] {
            Terrain::Ocean => temperature,
            Terrain::Shallow => temperature,
            _ => LAND_TEMPERATURE + LAND_VARIATION * temperature,
        }
    }).collect()
}

pub fn temperature_advection(
    temperatures: &mut Vec<f32>,
    edges: &Vec<Vec<usize>>,
    adjacencies: &Vec<Vec<usize>>, 
    currents: &Vec<f32>,
    iterations: usize,
) {
    advection(temperatures, edges, adjacencies, currents, iterations, |_| {});
}