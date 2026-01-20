use bevy::math::Vec3;
use math::vec3::{random_unit, random_units};
use noise::{NoiseFn, Perlin};
use rand::random_range;

const NUM_PLATES: usize = 20;

pub const OCEAN_DEPTH: f32 = 0.03;

const MAX_HEIGHT: f32 = 0.09;
const CONTINENT_HEIGHT: f32 = 0.01;
const CONTINENT_SLOPE: f32 = 0.04;

const NOISE_DETAIL: f64 = 2.0;
const NOISE_HEIGHT: f32 = 0.01;

const CONVERGENT_STRESS: f32 = 0.2;
const DIVERGENT_STRESS: f32 = 0.05;
const OCEAN_STRESS: f32 = 0.5;

pub fn tectonics(
    points: &Vec<Vec3>, 
    adjacencies: &Vec<Vec<usize>>,
) -> Vec<f32> {
    let mut plates = vec![];
    let mut queue = vec![];
    let mut colors = vec![];
    let mut rotations = vec![];

    for point in random_units(NUM_PLATES) {
        let i = (0..points.len())
            .min_by(|a, b| {
                point
                    .distance_squared(points[*a])
                    .total_cmp(&point.distance_squared(points[*b]))
            })
            .unwrap();
        plates.push(vec![]);
        queue.push((queue.len(), i));

        let color = [
            random_range(0.0..1.0),
            random_range(0.0..1.0),
            random_range(0.0..1.0),
        ];
        colors.push(color);

        rotations.push(random_unit());
    }

    let mut assigned = vec![false].repeat(points.len());
    let mut velocities = vec![Vec3::ZERO].repeat(points.len());
    while !queue.is_empty() {
        let (plate, p) = queue.swap_remove(random_range(0..queue.len()));
        
        if !assigned[p] {
            assigned[p] = true;
            plates[plate].push(p);

            let velocity = rotations[plate].cross(points[p]);
            velocities[p] = velocity;

            for q in &adjacencies[p] {
                if !assigned[*q] {
                    queue.push((plate, *q));
                }
            }
        }
    }

    plates.sort_by(|a, b| {
        b.len().cmp(&a.len())
    });

    let mut polar_plate = 0;
    let mut pole_latitude = 0.0;
    for plate in 0..plates.len() {
        for p in &plates[plate] {
            let latitude = points[*p].y.abs();
            if latitude > pole_latitude {
                polar_plate = plate;
                pole_latitude = latitude;
            }
        }
    }
    plates.swap(2, polar_plate);

    let mut stress = vec![0.0].repeat(points.len());
    for plate in 0..plates.len() {
        for p in &plates[plate] {
            let velocity_1 = velocities[*p];
            for q in &adjacencies[*p] {
                let velocity_2 = velocities[*q];
                let distance = points[*p] - points[*q];
                let mut added_stress = distance.dot(velocity_2 - velocity_1);
                if !continental(plate) { added_stress *= OCEAN_STRESS }
                if added_stress > 0.0 {
                    added_stress *= CONVERGENT_STRESS;
                } else {
                    added_stress *= DIVERGENT_STRESS;
                }
                stress[*p] += added_stress;
                stress[*q] += added_stress;
            }
        }
    }

    let perlin = Perlin::new(0);
    let mut heights = vec![0.0].repeat(points.len());
    for (i, plate) in plates.drain(..).enumerate() {
        if plate.is_empty() { continue }

        let plate_centre = points[plate[0]];

        for p in plate {
            let noise_point = [
                NOISE_DETAIL * points[p].x as f64,
                NOISE_DETAIL * points[p].y as f64,
                NOISE_DETAIL * points[p].z as f64,
            ];
            let noise = perlin.get(noise_point) as f32;
            
            heights[p] += NOISE_HEIGHT * noise;

            let stress = stress[p];
            heights[p] += stress;

            if continental(i) {
                let distance = plate_centre.distance_squared(points[p]);
                heights[p] += CONTINENT_HEIGHT - CONTINENT_SLOPE * distance;
            } else {
                heights[p] -= OCEAN_DEPTH;
            }

            if heights[p] > MAX_HEIGHT {
                heights[p] = MAX_HEIGHT;
            }
        }
    }

    return heights;
}

const fn continental(i: usize) -> bool {
    i % 3 == 1
}