use bevy::math::Vec3;

use crate::terrain::Terrain;
use crate::weather::atmosphere::atmosphere;
use crate::weather::fluid::velocities;
use crate::weather::ocean::ocean;
use crate::weather::precipitation::precipitation;
use crate::weather::temperatures::{temperature, temperatures, temperature_advection};

mod advection;
mod atmosphere;
mod fluid;
mod ocean;
pub mod temperatures;
mod precipitation;

const OCEAN_ITERATIONS: f32 = 5.0;
const ATMOSPHERE_ITERATIONS: f32 = 1.0;
const OCEAN_ADVECTION: f32 = 0.3;
const ATMOSPHERE_ADVECTION: f32 = 0.2;
const PRECIPITATION_ITERATIONS: f32 = 0.5;

pub fn weather(
    points: &Vec<Vec3>,
    half_edges: &Vec<usize>,
    terrain: &Vec<Terrain>,
    edges: &Vec<Vec<usize>>, 
    adjacencies: &Vec<Vec<usize>>, 
) -> (Vec<Vec3>, Vec<f32>, Vec<Vec3>, Vec<f32>, Vec<f32>) {
    let iteration_multiplier = (points.len() as f32).sqrt();
    let ocean_iterations = (OCEAN_ITERATIONS * iteration_multiplier) as usize;
    let atmosphere_iterations = (ATMOSPHERE_ITERATIONS * iteration_multiplier) as usize;
    let ocean_advection = (OCEAN_ADVECTION * iteration_multiplier) as usize;
    let atmosphere_advection = (ATMOSPHERE_ADVECTION * iteration_multiplier) as usize;
    let precipitation_iterations = (PRECIPITATION_ITERATIONS * iteration_multiplier) as usize;

    let ocean_currents = ocean(
        points,
        half_edges, 
        edges, 
        adjacencies, 
        terrain,
        ocean_iterations,
    );
    let mut ocean_temperatures = temperatures(points, terrain);
    temperature_advection(
        &mut ocean_temperatures, 
        edges, 
        adjacencies, 
        &ocean_currents, 
        ocean_advection
    );

    let atmospheric_currents = atmosphere(
        points, 
        half_edges, 
        edges, 
        adjacencies, 
        terrain, 
        &ocean_temperatures,
        atmosphere_iterations,
    );
    let mut atmospheric_temperatures = (0..points.len()).map(|p| {
        0.5 * (ocean_temperatures[p] + temperature(points[p].y))
    }).collect();
    temperature_advection(
        &mut atmospheric_temperatures, 
        edges, 
        adjacencies, 
        &atmospheric_currents, 
        atmosphere_advection
    );

    let precipitation = precipitation(
        terrain, 
        edges, 
        adjacencies, 
        &atmospheric_currents, 
        precipitation_iterations,
    );

    let ocean_currents = velocities(points, edges, adjacencies, &ocean_currents);
    let atmospheric_currents = velocities(points, edges, adjacencies, &atmospheric_currents);

    return (
        ocean_currents,
        ocean_temperatures,
        atmospheric_currents,
        atmospheric_temperatures,
        precipitation,
    );
}