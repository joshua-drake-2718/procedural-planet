use bevy::math::Vec3;

use crate::terrain::Terrain;
use crate::weather::temperatures::temperature;

const ICE_TEMPERATURE: f32 = -0.8;
const SNOW_TEMPERATURE: f32 = -0.6;
const TUNDRA_TEMPERATURE: f32 = -0.2;

const ARID_PRECIPITATION: f32 = 0.1;

const TROPICAL_TEMPERATURE: f32 = 0.3;
const RAINFOREST_PRECIPITATION: f32 = 0.6;

const FOREST_PRECIPITATION: f32 = 0.4;

impl Biome {
    pub const fn color(&self) -> [f32; 3] {
        match self {
            Self::Mountain => [0.2, 0.2, 0.2],
            Self::Forest => [0.0, 0.3, 0.0],
            Self::Grassland => [0.1, 0.4, 0.0],
            Self::Plains => [0.3, 0.4, 0.05],
            Self::Rainforest => [0.0, 0.2, 0.0],
            Self::Desert => [0.7, 0.7, 0.4],
            Self::Tundra => [0.1, 0.3, 0.1],
            Self::Shallow => [0.5, 0.5, 0.3],
            Self::Ocean => [0.0, 0.1, 0.1],
            Self::Ice => [0.5, 1.0, 1.0],
            Self::Snow => [1.0, 1.0, 1.0],
        }
    }
}

pub enum Biome {
    Mountain,
    Forest,
    Grassland,
    Plains,
    Rainforest,
    Desert,
    Tundra,
    Shallow,
    Ocean,
    Ice,
    Snow,
}

pub fn biomes(
    points: &Vec<Vec3>,
    terrain: &Vec<Terrain>,
    ocean_temperatures: &Vec<f32>,
    atmospheric_temperatures: &Vec<f32>,
    precipitation: &Vec<f32>,
) -> Vec<Biome> {
    (0..points.len()).map(|p| {
        let temperature = temperature(points[p].y);
        let ocean_temperature = (temperature + 2.0 * ocean_temperatures[p] + atmospheric_temperatures[p]) / 4.0;
        let land_temperature = (temperature + 2.0 * atmospheric_temperatures[p]) / 3.0;
        let precipitation = precipitation[p];

        match terrain[p] {
            Terrain::Mountain => Biome::Mountain,
            Terrain::Land => land(land_temperature, precipitation),
            Terrain::Shallow => shallow(ocean_temperature),
            Terrain::Ocean => ocean(ocean_temperature),
        }
    }).collect()
}

fn land(temperature: f32, precipitation: f32) -> Biome {
    if temperature < SNOW_TEMPERATURE {
        Biome::Snow
    } else if temperature < TUNDRA_TEMPERATURE {
        Biome::Tundra
    } else if precipitation < ARID_PRECIPITATION {
        Biome::Desert
    } else if temperature > TROPICAL_TEMPERATURE {
        tropical(precipitation)
    } else {
        temperate(precipitation)
    }
}

fn tropical(precipitation: f32) -> Biome {
    if precipitation > RAINFOREST_PRECIPITATION {
        Biome::Rainforest
    } else {
        Biome::Plains
    }
}

fn temperate(precipitation: f32) -> Biome {
    if precipitation > FOREST_PRECIPITATION {
        Biome::Forest
    } else {
        Biome::Grassland
    }
}

fn shallow(temperature: f32) -> Biome {
    if temperature < ICE_TEMPERATURE {
        Biome::Ice
    } else {
        Biome::Shallow
    }
}

fn ocean(temperature: f32) -> Biome {
    if temperature < ICE_TEMPERATURE {
        Biome::Ice
    } else {
        Biome::Ocean
    }
}