pub fn advection<T: FnMut(&mut Vec<f32>)>(
    values: &mut Vec<f32>,
    edges: &Vec<Vec<usize>>,
    adjacencies: &Vec<Vec<usize>>, 
    currents: &Vec<f32>,
    iterations: usize,
    mut f: T,
) {
    let weights: Vec<f32> = currents.iter().map(|current| {
        current.min(0.0)
    }).collect();
    let total_weights: Vec<f32> = edges.iter().map(|edges| {
        edges.iter().map(|e| {
            weights[*e]
        }).sum()
    }).collect();

    for _ in 0..iterations {
        *values = (0..edges.len()).map(|p| {
            if total_weights[p] == 0.0 {
                values[p]
            } else {
                (0..edges[p].len()).map(|i| {
                    let e = edges[p][i];
                    let q = adjacencies[p][i];
                    weights[e] * values[q]
                }).sum::<f32>() / total_weights[p]
            }
        }).collect();
        f(values)
    }
}