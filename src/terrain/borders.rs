use bevy::math::Vec3;
use triangulation::Triangulation;

pub fn borders(
    template: &Triangulation, 
    edges: &Vec<Vec<usize>>
) -> Vec<Vec<Vec3>> {
    let mut borders = vec![];
    for p in 0..template.points.len() {
        borders.push(vec![]);
        if let Some(start) = edges[p].first() {
            let start = *start;
            let mut e = start;
    
            loop {
                add_centroid(template, &mut borders[p], e / 3);
                add_midpoint(template, &mut borders[p], p, e);
                
                e = template.half_edges[e];
                e = Triangulation::prev_edge(e);
                
                if e == start { break }
            }
            
            add_centroid(template, &mut borders[p], e / 3);
            add_midpoint(template, &mut borders[p], p, e);
        }
    }

    return borders;
}

fn add_midpoint(
    template: &Triangulation,
    border: &mut Vec<Vec3>,
    p: usize,
    e: usize,
) {
    let a = template.points[p];
    let b = template.points[template.triangles[e]];
    let point = (a + b) / 2.0;
    add_point(border, point);
}

fn add_centroid(
    template: &Triangulation,
    border: &mut Vec<Vec3>,
    t: usize,
) {
    let a = template.points[template.triangles[3*t]];
    let b = template.points[template.triangles[3*t+1]];
    let c = template.points[template.triangles[3*t+2]];
    let point = (a + b + c) / 3.0;
    add_point(border, point);
}

fn add_point(
    border: &mut Vec<Vec3>, 
    point: Vec3,
) {
    if point.length_squared() > 1.0 {
        border.push(1.001 * point);
    } else {
        border.push(point.normalize());
    }
}