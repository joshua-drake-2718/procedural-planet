use triangulation::Triangulation;

pub fn adjacencies(template: &Triangulation, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    return edges.iter().map(|edges| {
        edges.iter().map(|e| {
            template.triangles[*e]
        }).collect()
    }).collect();
}