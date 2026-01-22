use bevy::prelude::*;

use planet::Planet;

const NUM_POINTS: usize = 5000;

const CAMERA_SPEED: f32 = 1.0;
const CAMERA_LOCK_SPEED: f32 = 10.0;
const CAMERA_DIST: f32 = 300.0;
const CAMERA_LIMITS: f32 = 290.0;

const SUN_DIST: f32 = 1000.0;
const SUN_SPEED: f32 = 1.5;
const SUN_INTENSITY: f32 = 1000.0;

const AMBIENT_BRIGHTNESS: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            reset,
            move_camera,
            move_sun,
        ))
        .add_systems(Update, (
            switch_view_mode, 
            draw_gizmos,
        ).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ambient_light.brightness = AMBIENT_BRIGHTNESS;

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: SUN_INTENSITY,
            ..default()
        },
        Transform::from_xyz(
            SUN_DIST, 
            0.0, 
            0.0,
        ).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            CAMERA_DIST, 
            0.0, 
            0.0,
        ).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    add_planet(&mut commands, &mut meshes, &mut materials);

    commands.insert_resource(ViewMode::None);
    commands.insert_resource(CameraLock(true));
}

fn add_planet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mut planet = Planet::new(NUM_POINTS);
    planet.render(commands, meshes, materials);
    commands.insert_resource(planet);
}

fn reset(
    mut commands: Commands,
    entities: Query<Entity, With<Mesh3d>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Backslash) {
        for entity in entities {
            commands.entity(entity).despawn();
        }
        add_planet(&mut commands, &mut meshes, &mut materials);
    }
}

fn move_camera(
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<DirectionalLight>)>,
    mut sun: Single<&mut Transform, (With<DirectionalLight>, Without<Camera3d>)>,
    mut camera_lock: ResMut<CameraLock>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::Tab) {
        camera_lock.0 = true;
    }

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS){
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyQ) {
        direction.z += 1.0;
        camera_lock.0 = false;
    }
    if keys.pressed(KeyCode::KeyE){
        direction.z -= 1.0;
        camera_lock.0 = false;
    }
    direction = direction.normalize_or_zero();

    if camera_lock.0 {
        direction.z = CAMERA_LOCK_SPEED * camera.left().y;
        if camera.translation.y > CAMERA_LIMITS {
            direction.y = direction.y.min(0.0);
        } else if camera.translation.y < -CAMERA_LIMITS {
            direction.y = direction.y.max(0.0);
        }
    }

    let angle = CAMERA_SPEED * time.delta_secs() * direction;
    let horizontal_axis = *camera.left();
    let vertical_axis = if camera_lock.0 {
        Vec3::Y
    } else {
        *camera.up()
    };
    let forward_axis = camera.translation.normalize();

    let camera_rotation = Quat::from_axis_angle(vertical_axis, angle.x)
        .mul_quat(Quat::from_axis_angle(horizontal_axis, angle.y))
        .mul_quat(Quat::from_axis_angle(forward_axis, angle.z));
    camera.rotate_around(Vec3::ZERO, camera_rotation);
    
    if camera_lock.0 {
        sun.rotate_around(
            Vec3::ZERO, 
            Quat::from_rotation_y(angle.x)
        );
    }
}

fn move_sun(
    mut sun: Single<&mut Transform, With<DirectionalLight>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let right_pressed = keys.pressed(KeyCode::KeyX);
    let left_pressed = keys.pressed(KeyCode::KeyZ);

    let mut direction = 0.0;
    if right_pressed {
        direction += 1.0;
    }
    if left_pressed {
        direction -= 1.0;
    }

    direction *= SUN_SPEED * time.delta_secs();
    sun.rotate_around(
        Vec3::ZERO, 
        Quat::from_rotation_y(direction),
    );
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    view_mode: Res<ViewMode>,
    planet: Res<Planet>,
) {
    let height = match *view_mode {
        ViewMode::Ocean => 1.0,
        _ => 1.05,
    };

    let velocities = match *view_mode {
        ViewMode::Ocean => &planet.ocean_currents,
        ViewMode::Atmosphere => &planet.atmospheric_currents,
        ViewMode::Precipitation => &planet.atmospheric_currents,
        _ => &vec![],
    };

    let empty = vec![];
    let temperatures: &Vec<f32> = match *view_mode {
        ViewMode::Ocean => &planet.ocean_temperatures,
        ViewMode::Atmosphere => &planet.atmospheric_temperatures,
        ViewMode::Precipitation => &planet.precipitation,
        _ => &empty,
    };

    let sign = match *view_mode {
        ViewMode::Precipitation => -1.0,
        _ => 1.0,
    };

    if !velocities.is_empty() {
        for p in 0..planet.points.len() {
            let start = 100.0 * height * planet.points[p].normalize();
            let end = start + 200.0 * velocities[p];
            gizmos.arrow(start, end, Color::linear_rgb(
                sign * temperatures[p], 
                0.0, 
                -sign * temperatures[p],
            )).with_tip_length(0.5);
        }
    }

}

fn switch_view_mode(
    mut view_mode: ResMut<ViewMode>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Digit0) {
        *view_mode = ViewMode::None
    } else if keys.just_pressed(KeyCode::Digit1) {
        set_view_mode(&mut view_mode, ViewMode::Ocean)
    } else if keys.just_pressed(KeyCode::Digit2) {
        set_view_mode(&mut view_mode, ViewMode::Atmosphere)
    } else if keys.just_pressed(KeyCode::Digit3) {
        set_view_mode(&mut view_mode, ViewMode::Precipitation)
    }
}

fn set_view_mode(
    view_mode: &mut ViewMode,
    value: ViewMode,
) {
    *view_mode = if *view_mode == value {
        ViewMode::None
    } else {
        value
    }
}

#[derive(Resource, PartialEq, Eq)]
enum ViewMode {
    None,
    Ocean,
    Atmosphere,
    Precipitation,
}

#[derive(Resource)]
struct CameraLock(bool);