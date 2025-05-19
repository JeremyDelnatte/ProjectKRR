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
use std::{collections::HashMap, f32::consts::TAU, process::Command, str::FromStr};

#[derive(Debug)]
enum Direction {
    Above,
    Below,
    North,
    South,
    East,
    West,
}

impl Direction {
    fn pipe_width(&self) -> f32 {
        match self {
            Direction::Above | Direction::Below => 0.3,
            Direction::North | Direction::South => 0.3,
            Direction::East | Direction::West => 0.65,
        }
    }

    fn pipe_height(&self) -> f32 {
        match self {
            Direction::Above | Direction::Below => 0.65,
            Direction::North | Direction::South => 0.3,
            Direction::East | Direction::West => 0.3,
        }
    }

    fn pipe_depth(&self) -> f32 {
        match self {
            Direction::Above | Direction::Below => 0.3,
            Direction::North | Direction::South => 0.65,
            Direction::East | Direction::West => 0.3,
        }
    }

    fn pipe_transform(&self, x: f32, y: f32, z: f32) -> Transform {
        match self {
            Direction::Above => Transform::from_xyz(x, y + 0.175, z),
            Direction::Below => Transform::from_xyz(x, y - 0.175, z),
            Direction::North => Transform::from_xyz(x, y, z + 0.175),
            Direction::South => Transform::from_xyz(x, y, z - 0.175),
            Direction::East => Transform::from_xyz(x + 0.175, y, z),
            Direction::West => Transform::from_xyz(x - 0.175, y, z),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Direction::Above),
            "b" => Ok(Direction::Below),
            "n" => Ok(Direction::North),
            "s" => Ok(Direction::South),
            "e" => Ok(Direction::East),
            "w" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

#[derive(Resource)]
struct ActiveLayer {
    y: usize,
}

#[derive(Resource)]
struct Positions {
    positions: HashMap<(usize, usize, usize), String>,
    pipe_positions: HashMap<(usize, usize, usize), (Direction, Direction)>,
}

#[derive(Resource, Debug)]
struct Config {
    width: usize,
    height: usize,
    depth: usize,
    hide: bool,
}

#[derive(Component)]
struct Block {
    y: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    positions: Res<Positions>,
    config: Res<Config>,
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

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::WHITE,
    //         ..default()
    //     })),
    //     Transform::from_xyz(0.3, 1.3, 0.3),
    // ));

    let pipe_color = Color::srgb_u8(255, 0, 0);
    let pipe_mat = materials.add(StandardMaterial {
        base_color: pipe_color,
        ..default()
    });

    for x in 1..=config.width {
        for z in 1..=config.depth {
            for y in 1..=config.height {
                let block = positions
                    .positions
                    .get(&(x, y, z))
                    .unwrap();

                let mat = materials_block
                    .get(block)
                    .unwrap();

                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(mat.clone()),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                    Block { y },
                ));

                if let Some((in_dir, out_dir)) = positions.pipe_positions.get(&(x, y, z)) {

                    let width = in_dir.pipe_width();
                    let height = in_dir.pipe_height();
                    let depth = in_dir.pipe_depth();

                    let in_pipe_transform = in_dir.pipe_transform(x as f32, y as f32, z as f32);

                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(width, height, depth))),
                        MeshMaterial3d(pipe_mat.clone()),
                        in_pipe_transform,
                    ));

                    let width = out_dir.pipe_width();
                    let height = out_dir.pipe_height();
                    let depth = out_dir.pipe_depth();

                    let out_pipe_transform = out_dir.pipe_transform(x as f32, y as f32, z as f32);


                    dbg!(x, y, z, in_dir, out_dir, in_pipe_transform, out_pipe_transform);

                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(width, height, depth))),
                        MeshMaterial3d(pipe_mat.clone()),
                        out_pipe_transform,
                    ));
                }
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
            focus: Vec3::new(2.5, 2.5, 2.5),
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

fn switch_layer_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<Config>,
    mut active_layer: ResMut<ActiveLayer>,
    mut query: Query<(&Block, &mut Visibility)>,
) {
    // if key_input.just_pressed(KeyCode::ArrowUp) {
    //     active_layer.y += 1;
    // } else if key_input.just_pressed(KeyCode::ArrowDown) {
    //     if active_layer.y > 0 {
    //         active_layer.y -= 1;
    //     }
    // }
    //
    if key_input.just_pressed(KeyCode::KeyH) {
        config.hide = !config.hide;
    }

    for (block, mut visibility) in query.iter_mut() {
        *visibility = if config.hide {
            Visibility::Hidden

        } else {
            Visibility::Visible
        };
    }
}

fn parse_sol(line: &str) -> Positions {
    let atoms: Vec<&str> = line.trim().split(" ").collect();
    let mut positions: HashMap<(usize, usize, usize), String> = HashMap::new();
    let mut pipe_positions: HashMap<(usize, usize, usize), (Direction, Direction)> = HashMap::new();

    dbg!(atoms.len());

    for atom in &atoms {
        match atom {
            atom if atom.starts_with("block_pos(") => {
                let mut atom = atom.strip_prefix("block_pos(").and_then(|s| s.strip_suffix(")")).and_then(|s| Some(s.split(","))).expect("Invalid atom");
                let x = atom.next().unwrap().parse::<usize>().unwrap();
                let y = atom.next().unwrap().parse::<usize>().unwrap();
                let z = atom.next().unwrap().parse::<usize>().unwrap();
                let block = atom.next().unwrap();
                positions.insert((x, y, z), block.to_string());
            },
            atom if atom.starts_with("pipe_pos(") => {
                dbg!(atom);
                let mut atom = atom.strip_prefix("pipe_pos(").and_then(|s| s.strip_suffix(")")).and_then(|s| Some(s.split(","))).expect("Invalid atom");
                let x = atom.next().unwrap().parse::<usize>().unwrap();
                let y = atom.next().unwrap().parse::<usize>().unwrap();
                let z = atom.next().unwrap().parse::<usize>().unwrap();
                let in_dir = atom.next().unwrap().parse::<Direction>().unwrap();
                let out_dir = atom.next().unwrap().parse::<Direction>().unwrap();
                pipe_positions.insert((x, y, z), (in_dir, out_dir));
            },
            _ => (),
        }
    }

    Positions {
        positions,
        pipe_positions,
    }
}


use clap::Parser;

/// Compute the volume of a box (defaults to a 3×3×3 cube)
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Box height (default 3)
    #[arg(long, default_value_t = 3)]
    height: usize,

    /// Box width  (default 3)
    #[arg(long, default_value_t = 3)]
    width: usize,

    /// Box depth  (default 3)
    #[arg(long, default_value_t = 3)]
    depth: usize,
}


fn main() {

    let args = Args::parse();

    let config = Config {
        width: args.width,
        height: args.height,
        depth: args.depth,
        hide: false,
    };

    let python = "../programs/env/bin/python";

    let output = Command::new(python)
        .arg("../programs/generator.py")
        .arg(format!("--height={}", config.height))
        .arg(format!("--width={}", config.width))
        .arg(format!("--depth={}", config.depth))
        .output()
        .expect("Failed to execute python");

    let positions = parse_sol(String::from_utf8_lossy(&output.stdout).as_ref());

    // let positions = parse_sol("block_pos(2,1,1,1,1) block_pos(1,2,1,4,1) block_pos(1,1,2,7,1) block_pos(3,1,2,6,1) block_pos(3,2,2,3,1) block_pos(2,1,3,8,1) block_pos(3,1,3,2,1) block_pos(3,3,3,5,1) block_pos(1,1,3,8,2) block_pos(2,1,2,8,4) block_pos(1,2,2,7,2) block_pos(3,1,1,6,4) block_pos(2,3,3,5,4) block_pos(1,3,1,4,2) block_pos(3,3,2,3,3) block_pos(3,2,3,2,2) block_pos(1,1,1,1,2) block_pos(3,3,1,3,2) block_pos(2,3,1,4,4) block_pos(1,3,2,4,3) block_pos(2,3,2,5,2) block_pos(1,3,3,5,3) block_pos(3,2,1,6,3) block_pos(1,2,3,7,4) block_pos(2,2,2,8,3) block_pos(2,2,3,7,3) block_pos(2,2,1,6,2) pipe_pos(2,1,1,e,a) pipe_pos(1,2,1,e,a) pipe_pos(1,2,2,e,n) pipe_pos(1,2,3,e,s) pipe_pos(2,2,2,e,w) pipe_pos(2,2,3,e,w) pipe_pos(2,2,1,b,w) pipe_pos(1,3,1,b,w) pipe_pos(3,2,2,b,w) pipe_pos(3,3,3,b,a) pipe_pos(2,2,1,w,b) pipe_pos(1,3,1,w,b) pipe_pos(3,2,2,w,b) pipe_pos(3,2,3,w,a) pipe_pos(3,1,1,w,n) pipe_pos(2,2,2,w,e) pipe_pos(2,2,3,w,e) pipe_pos(2,1,1,a,e) pipe_pos(1,2,1,a,e) pipe_pos(3,2,3,a,w) pipe_pos(3,1,2,a,s) pipe_pos(3,3,3,a,b) pipe_pos(1,2,2,n,e) pipe_pos(3,1,1,n,w) pipe_pos(3,1,2,s,a) pipe_pos(1,2,3,s,e)");

    App::new()
        .insert_resource(positions)
        .insert_resource(config)
        .insert_resource(ActiveLayer { y: 1 }) // <-- starting layer
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_camera_controls_system)
        .add_systems(Update, switch_layer_system) // <-- add this
        .run();
}
