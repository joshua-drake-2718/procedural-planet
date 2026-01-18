use bevy::math::Vec3;

use crate::Terrain;
use crate::weather::fluid::fluid;

const SHALLOW_WEIGHT: f32 = 0.3;

pub fn ocean(
    points: &Vec<Vec3>,
    half_edges: &Vec<usize>,
    edges: &Vec<Vec<usize>>, 
    adjacencies: &Vec<Vec<usize>>, 
    terrain: &Vec<Terrain>,
    iterations: usize,
) -> Vec<f32> {
    let weights: Vec<f32> = terrain.iter().map(|terrain| {
        match terrain {
            Terrain::Ocean => {1.0},
            Terrain::Shallow => {SHALLOW_WEIGHT},
            _ => {0.0},
        }
    }).collect();

    let starting_velocities: Vec<Vec3> = (0..points.len()).map(|p| {
        let point = points[p];
        weights[p] * velocity(point.y) * Vec3::new(
            -point.z,
            0.0,
            point.x,
        )
    }).collect();
    let starting_pressures = vec![0.0].repeat(points.len());
    
    return fluid(
        points, 
        half_edges,
        edges, 
        adjacencies, 
        &weights, 
        starting_velocities,
        &starting_pressures,
        iterations,
    );
}

fn velocity(y: f32) -> f32 {
    (1.0 - y*y).sqrt() * (1.0 - 4.0*y*y)
}