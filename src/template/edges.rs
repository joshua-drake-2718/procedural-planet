use triangulation::Triangulation;

pub fn edges(template: &Triangulation) -> Vec<Vec<usize>> {
    let mut edges = vec![];
    for _ in 0..template.points.len() {
        edges.push(vec![]);
    }

    for t in 0..template.num_triangles() {
        let a = 3*t;
        let b = 3*t+1;
        let c = 3*t+2;

        edges[template.triangles[a]].push(c);
        edges[template.triangles[b]].push(a);
        edges[template.triangles[c]].push(b);
    }

    return edges;
}