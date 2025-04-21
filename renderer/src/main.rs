//! Demonstrates all common configuration options,
//! and how to modify them at runtime
//!
//! Controls:
//!   Orbit: Middle click
//!   Pan: Shift + Middle click
//!   Zoom: Mousewheel

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin, TouchControls};
use rand::Rng;
use std::{collections::HashMap, f32::consts::TAU, process::Command};

#[derive(Resource)]
struct Positions {
    positions: HashMap<(usize, usize, usize), String>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    positions: Res<Positions>,
) {

    // let mat = materials.add(StandardMaterial {
    //     base_color: Color::srgba(0.8, 0.7, 0.6, 0.5),
    //     alpha_mode: AlphaMode::Add,
    //     ..default()
    // });

    let mut rng = rand::rng();
    let mut materials_block = HashMap::new();
    for block in positions.positions.values() {

        let r: u8 = rng.random_range(0..=255);
        let g: u8 = rng.random_range(0..=255);
        let b: u8 = rng.random_range(0..=255);

        let random_color = Color::srgb_u8(r, g, b);
        let mat = materials.add(StandardMaterial {
            base_color: random_color,
            // alpha_mode: AlphaMode::Add,
            ..default()
        });
        materials_block.insert(block, mat);
    }

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=2 {
                let block = positions
                    .positions
                    .get(&(i, j, k))
                    .unwrap();

                let mat = materials_block
                    .get(block)
                    .unwrap();

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(mat.clone()),
                    Transform::from_xyz(i as f32, k as f32, j as f32),
                ));
            }
        }
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0, // You can tweak this for softer/harsher ambient light
    });
    //
    // Camera
    commands.spawn((
        // Note we're setting the initial position below with yaw, pitch, and radius, hence
        // we don't set transform on the camera.
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position, relative to focus (overrides camera's transform).
            yaw: Some(TAU / 8.0),
            pitch: Some(TAU / 8.0),
            radius: Some(5.0),
            // Set limits on rotation and zoom
            // yaw_upper_limit: Some(TAU / 4.0),
            // yaw_lower_limit: Some(-TAU / 4.0),
            // pitch_upper_limit: Some(TAU / 3.0),
            // pitch_lower_limit: Some(-TAU / 3.0),
            // zoom_upper_limit: Some(5.0),
            // zoom_lower_limit: 1.0,
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // Change the controls (these match Blender)
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Left,
            modifier_pan: Some(KeyCode::ShiftLeft),
            // Reverse the zoom direction
            reversed_zoom: true,
            // Use alternate touch controls
            touch_controls: TouchControls::TwoFingerOrbit,
            ..default()
        },
    ));
}

// This is how you can change config at runtime.
// Press 'T' to toggle the camera controls.
fn toggle_camera_controls_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if key_input.just_pressed(KeyCode::KeyT) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.enabled = !pan_orbit.enabled;
        }
    }
}

fn parse_sol(line: &str) -> HashMap<(usize, usize, usize), String> {
    let atoms: Vec<&str> = line.split(" ").collect();
    let mut positions: HashMap<(usize, usize, usize), String> = HashMap::new();

    for atom in &atoms {
        match atom {
            atom if atom.starts_with("cell_value(") => {
                let mut atom = atom.strip_prefix("cell_value(").and_then(|s| s.strip_suffix(")")).and_then(|s| Some(s.split(","))).expect("Invalid atom");
                let x = atom.next().unwrap().parse::<usize>().unwrap();
                let y = atom.next().unwrap().parse::<usize>().unwrap();
                let z = atom.next().unwrap().parse::<usize>().unwrap();
                let block = atom.next().unwrap();
                positions.insert((x, y, z), block.to_string());
            }
            _ => (),
        }
    }

    positions
}


fn main() {

    let program = "../programs/generator.lp";

    let output = Command::new("clingo")
        .arg(program)
        .arg("-n 1")
        .arg("--parallel-mode=11")
        .arg("--seed=34837")
        .arg("--rand-freq=1")
        .output()
        .expect("Failed to execute clingo");

    let clingo_output = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = clingo_output.split("\n").collect();

    dbg!(lines[4]);
    let positions = parse_sol(lines[4]);

    App::new()
        .insert_resource(Positions { positions })
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_camera_controls_system)
        .run();
}
