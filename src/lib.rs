use bevy::prelude::*;

use crate::template::template;
use crate::terrain::biomes::{Biome, biomes};
use crate::terrain::tectonics::tectonics;
use crate::terrain::borders::borders;
use crate::terrain::{terrain, Terrain};
use crate::weather::weather;

mod render;
mod template;
mod terrain;
mod weather;

#[derive(Resource)]
pub struct Planet {
    pub points: Vec<Vec3>,
    pub adjacencies: Vec<Vec<usize>>,
    pub mesh: Mesh,
    pub terrain: Vec<Terrain>,
    pub biomes: Vec<Biome>,
    pub borders: Vec<Vec<Vec3>>,
    pub ocean_currents: Vec<Vec3>,
    pub ocean_temperatures: Vec<f32>,
    pub atmospheric_currents: Vec<Vec3>,
    pub atmospheric_temperatures: Vec<f32>,
    pub precipitation: Vec<f32>,
}

impl Planet {
    pub fn new(num_points: usize) -> Planet {
        let (
            mut template, 
            edges, 
            adjacencies
        ) = template(num_points);

        let mut heights = tectonics(
            &template.points, 
            &adjacencies, 
        );
        let terrain = terrain(&mut heights);

        let (
            ocean_currents, 
            ocean_temperatures,
            atmospheric_currents,
            atmospheric_temperatures,
            precipitation,
        ) = weather(
            &template.points,
            &template.half_edges,
            &terrain,
            &edges, 
            &adjacencies,
        );

        let biomes = biomes(
            &template.points, 
            &terrain,
            &ocean_temperatures,
            &atmospheric_temperatures,
            &precipitation,
        );

        for p in 0..template.points.len() {
            match biomes[p] {
                Biome::Ice => heights[p] = 0.0,
                Biome::Rainforest => heights[p] *= 0.5,
                _ => {},
            }
            template.points[p] *= 1.0 + heights[p]
        }

        let borders = borders(
            &template, 
            &edges,
        );
        let mesh = template.mesh();

        return Planet { 
            points: template.points, 
            adjacencies, 
            mesh, 
            terrain, 
            biomes,
            borders,
            ocean_currents,
            ocean_temperatures,
            atmospheric_currents,
            atmospheric_temperatures,
            precipitation,
        };
    }
}