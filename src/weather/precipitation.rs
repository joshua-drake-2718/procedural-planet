use crate::terrain::Terrain;
use crate::weather::advection::advection;

const DECAY: f32 = 750.0;
const EVAPORATION: f32 = 0.3;

pub fn precipitation(
    terrain: &Vec<Terrain>,
    edges: &Vec<Vec<usize>>, 
    adjacencies: &Vec<Vec<usize>>, 
    currents: &Vec<f32>,
    iterations: usize,
) -> Vec<f32> {
    let mut precipitation = vec![0.0].repeat(terrain.len());
    let decay = 1.0 - DECAY / terrain.len() as f32;

    advection(&mut precipitation, edges, adjacencies, currents, iterations, |precipitations| {
        for p in 0..terrain.len() {
            match terrain[p] {
                Terrain::Ocean | Terrain::Shallow => {
                    precipitations[p] *= 1.0 - EVAPORATION;
                    precipitations[p] += EVAPORATION;
                },
                Terrain::Land => precipitations[p] *= decay,
                Terrain::Mountain => precipitations[p] = 0.0,
            }
        }
    });

    return precipitation;
}