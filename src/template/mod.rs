use triangulation::Triangulation;

use edges::edges;
use adjacencies::adjacencies;

mod adjacencies;
mod edges;

pub fn template(num_points: usize) -> (Triangulation, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let template = Triangulation::random_sphere(1.0, num_points);
    let edges = edges(&template);
    let adjacencies = adjacencies(&template, &edges);
    return (template, edges, adjacencies);
}