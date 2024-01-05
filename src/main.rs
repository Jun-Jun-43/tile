use bevy::{
    core_pipeline::{bloom::{BloomSettings, BloomCompositeMode, BloomPrefilterSettings}, tonemapping::Tonemapping},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow, render::view::screenshot::ScreenshotManager,
};
use rand::Rng;

const UNOHANA: Color = Color::rgb(251.0 / 256.0, 251.0 / 256.0, 246.0 / 256.0);
const USUZUMI: Color = Color::rgb(163.0 / 256.0, 163.0 / 256.0, 162.0 / 256.0);
const MIZU: Color = Color::rgb(127.0 / 256.0, 204.0 / 256.0, 227.0 / 256.0);
const MOEGI: Color = Color::rgb(167.0 / 256.0, 189.0 / 256.0, 0.0 / 256.0);
const KARAKURENAI: Color = Color::rgb(244.0 / 256.0, 0.0 / 256.0, 25.0 / 256.0);
const SHIGOKU: Color = Color::rgb(45.0 / 256.0, 4.0 / 256.0, 37.0 / 256.0);

#[derive(Component)]
struct Tile;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (720.0, 1280.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, camera_setup))
        .add_systems(Update, (contraction, bloom, screenshots))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&mut Window>,
) {
    let colors = vec![KARAKURENAI, SHIGOKU];
    let mut rng = rand::thread_rng();

    let win = windows.single();
    let win_width = win.width();
    let win_height = win.height();

    let tile_width = win_width / 10.0;
    let tile_height = win_height / 10.0;
    let tile_margin = 10.0;

    let mut width_offset = tile_width / 2.0;
    let mut height_offset = tile_height / 2.0;

    for _ in 0..(win_height / tile_height) as usize {
        for _ in 0..(win_width / tile_width) as usize {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Quad::new(Vec2::new(
                            tile_width - tile_margin,
                            tile_height - tile_margin,
                        ))))
                        .into(),
                    transform: Transform::from_xyz(
                        (-win_width / 2.0) + width_offset,
                        (win_height / 2.0) - height_offset,
                        0.0,
                    ),
                    material: materials.add(ColorMaterial::from(
                        colors[rng.gen_range(0..2)]
                        // Color::WHITE
                    )),
                    ..default()
                },
                Tile,
            ));

            width_offset += tile_width;
        }
        width_offset = tile_width / 2.0;
        height_offset += tile_height;
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.2,
            low_frequency_boost: 0.2,
            low_frequency_boost_curvature: 1.0,
            high_pass_frequency: 0.5,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.4,
                threshold_softness: 0.5
            },
            composite_mode: BloomCompositeMode::Additive,
        }
    ));
}

fn contraction(mut query: Query<(&Mesh2dHandle, &mut Transform)>) {
    for mut mesh in query.iter_mut() {
        let mut swaying = rand::thread_rng();
        let rand_num = swaying.gen_range(-0.5..0.5) as f32;
        // let swaying = time.delta_seconds().sin() / 50.0;

        let swaying = rand_num / 15.0;
        mesh.1.scale.x -= swaying;
        mesh.1.scale.y -= swaying;
        mesh.1.translation.x -= swaying;
        mesh.1.translation.y -= swaying;
    }
}

fn bloom(mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>, time: Res<Time>) {
    let bloom = (time.elapsed_seconds() + time.delta_seconds()).sin().abs();
    let (_entity, bloom_settings) = camera.single_mut();
    let mut bloom_settings = bloom_settings.unwrap();

    bloom_settings.intensity += bloom / 500.0;
    bloom_settings.low_frequency_boost += bloom / 500.0;
    // bloom_settings.low_frequency_boost_curvature = 1.0;
    // bloom_settings.high_pass_frequency += bloom / 10.0;
    // bloom_settings.prefilter_settings.threshold = 0.4;
    // bloom_settings.prefilter_settings.threshold_softness = 0.7;
    // bloom_settings.composite_mode = BloomCompositeMode::EnergyConserving;

    println!("{}", bloom / 10.0);
}

fn screenshots(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    let path = format!("./screenshots/screenshot-{}.png", *counter);
    *counter += 1;
    println!("{}", *counter);


    if *counter < 1000 {
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}
