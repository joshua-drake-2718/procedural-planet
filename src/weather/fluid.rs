use bevy::math::Vec3;

pub fn fluid(
    points: &Vec<Vec3>,
    half_edges: &Vec<usize>,
    edges: &Vec<Vec<usize>>, 
    adjacencies: &Vec<Vec<usize>>, 
    weights: &Vec<f32>,
    starting_velocities: Vec<Vec3>,
    starting_pressures: &Vec<f32>,
    iterations: usize,
) -> Vec<f32> {
    let total_weights = total_weights(adjacencies, &weights);
    let mut currents = currents(
        points, 
        half_edges, 
        edges, 
        adjacencies, 
        weights, 
        &starting_velocities,
    );

    let divergences: Vec<f32> = edges.iter().map(|edges| {
        edges.iter().map(|e| {
            currents[*e]
        }).sum()
    }).collect();

    let mut pressures = vec![0.0].repeat(points.len());
    for _ in 0..iterations {
        for p in 0..points.len() {
            if total_weights[p] == 0.0 || weights[p] == 0.0 { continue }

            let mut pressure = starting_pressures[p] - divergences[p];

            for q in &adjacencies[p] {
                pressure += weights[p] * weights[*q] * pressures[*q];
            }

            pressures[p] = pressure / total_weights[p] / weights[p];
        }
    }

    for p in 0..points.len() {
        let pressure = pressures[p];
        for i in 0..edges[p].len() {
            let e = edges[p][i];
            let q = adjacencies[p][i];

            let current = weights[p] * weights[q] * (pressure - pressures[q]);
            currents[e] += current;
        }
    }

    return currents;
}

pub fn velocities(
    points: &Vec<Vec3>,
    edges: &Vec<Vec<usize>>,
    adjacencies: &Vec<Vec<usize>>,
    currents: &Vec<f32>,
) -> Vec<Vec3> {
    (0..points.len()).map(|p| {
        velocity(p, points, edges, adjacencies, &currents)
    }).collect()
}

pub fn total_weights(
    adjacencies: &Vec<Vec<usize>>,
    weights: &Vec<f32>,
) -> Vec<f32> {
    adjacencies.iter().map(|adjacencies| {
        adjacencies.iter()
            .map(|q| {
                weights[*q]
            }).sum()
    }).collect()
}

pub fn currents(
    points: &Vec<Vec3>,
    half_edges: &Vec<usize>,
    edges: &Vec<Vec<usize>>,
    adjacencies: &Vec<Vec<usize>>,
    weights: &Vec<f32>,
    velocities: &Vec<Vec3>
) -> Vec<f32> {
    let total_weights = total_weights(adjacencies, weights);

    let mut currents = vec![0.0].repeat(half_edges.len());
    for p in 0..points.len() {
        if total_weights[p] == 0.0 { continue }
        
        let point = points[p];
        let velocity = velocities[p];

        for i in 0..edges[p].len() {
            let e = edges[p][i];
            let q = adjacencies[p][i];
            let direction = points[q] - point;
            let weight = weights[q] / total_weights[p];

            let current = weight * direction.dot(velocity);
            currents[e] += current;
            currents[half_edges[e]] -= current;
        }
    }
    return currents;
}

fn velocity(
    p: usize,
    points: &Vec<Vec3>,
    edges: &Vec<Vec<usize>>,
    adjacencies: &Vec<Vec<usize>>,
    currents: &Vec<f32>,
) -> Vec3 {
    let point = points[p];
    (0..edges[p].len()).map(|i| {
        let e = edges[p][i];
        let q = adjacencies[p][i];
        let direction = points[q] - point;
        currents[e].max(0.0) * direction.normalize()
    }).sum()
}