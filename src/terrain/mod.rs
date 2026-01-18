pub mod biomes;
pub mod borders;
pub mod tectonics;

const MOUNTAIN_HEIGHT: f32 = 0.03;
pub const LAND_HEIGHT: f32 = -0.01;
pub const SHALLOW_HEIGHT: f32 = -0.02;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terrain {
    Mountain,
    Land,
    Shallow,
    Ocean,
}

pub fn terrain(
    heights: &mut Vec<f32>,
) -> Vec<Terrain> {
    (0..heights.len()).map(|p| {
        let terrain = Terrain::at_height(heights[p]);
        match terrain {
            Terrain::Land => {
                heights[p] = heights[p].max(0.0)
            },
            Terrain::Shallow => {
                heights[p] = SHALLOW_HEIGHT
            },
            _ => {},
        }
        terrain
    }).collect()
}

impl Terrain {
    pub fn at_height(height: f32) -> Self {
        if height > MOUNTAIN_HEIGHT {
            Self::Mountain
        } else if height > LAND_HEIGHT {
            Self::Land
        } else if height > SHALLOW_HEIGHT {
            Self::Shallow
        } else {
            Self::Ocean
        }
    }
}