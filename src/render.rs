use bevy::prelude::*;
use triangulation::Triangulation;

impl crate::Planet {
    pub fn render(
        &mut self,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        self.mesh.scale_by(Vec3::splat(100.0));

        let colors: Vec<[f32; 4]> = self.biomes.iter().map(|b| {
            let c = b.color();
            [c[0], c[1], c[2], 1.0]
        }).collect();
        self.mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        self.mesh.duplicate_vertices();
        self.mesh.compute_flat_normals();

        commands.spawn( (
            Mesh3d(meshes.add(self.mesh.clone())),
            MeshMaterial3d(materials.add(Color::WHITE)),
        ));
        
        let mut ocean = Triangulation::sphere(99.5, 3).mesh();
        ocean.compute_smooth_normals();
        commands.spawn((
            Mesh3d(meshes.add(ocean)),
            MeshMaterial3d(materials.add(Color::linear_rgba(0.0, 0.0, 0.75, 0.25))),
        ));

        // let border_material = materials.add(
        //     Color::hsl(0.0, 0.0, 0.25)
        // );
        // for border in &self.borders {
        //     if !border.is_empty() {
        //         let vertices: Vec<Vec3> = border.iter().map(|v| { 100.0 * v }).collect();
        //         let mut mesh = Mesh::from(Polyline3d::new(vertices));
        //         mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, border.clone());
        //         commands.spawn((
        //             Mesh3d(meshes.add(mesh)),
        //             MeshMaterial3d(border_material.clone()),
        //         ));
        //     }
        // }
    }
}