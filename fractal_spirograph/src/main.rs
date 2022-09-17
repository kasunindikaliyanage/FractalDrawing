use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_prototype_lyon::prelude::*;

const WINDOW_HEIGHT: f32 = 1000.;
const WINDOW_WIDTH: f32 = 1000.;

#[derive(Component)]
pub struct MainCamera;

pub struct WinSize {
    pub h: f32,
    pub w: f32,
}

pub struct CursorPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct AreaBoundary_1;

#[derive(Component)]
pub struct AngleBase_1(f64);

#[derive(Component)]
pub struct AreaBoundary_2;

#[derive(Component)]
pub struct AngleBase_2(f64);

#[derive(Component)]
pub struct AreaBoundary_3;

#[derive(Component)]
pub struct AngleBase_3(f64);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WindowDescriptor {
            title: "Fractal Spirograph".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(update_system)
        .run();
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    let window = windows.get_primary_mut().unwrap();
    let (WinW, WinH) = (window.width(), window.height());

    let win_size = WinSize { h: WinH, w: WinW };
    commands.insert_resource(win_size);

    let shape_1 = shapes::Circle {
        radius: 125.,
        center: Vec2::new(0., 0.),
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape_1,
        DrawMode::Stroke(StrokeMode::new(Color::BLUE, 2.0)),
        Transform::default(),
    ));

    let shape_2 = shapes::Circle {
        radius: 75.,
        center: Vec2::new(0., 0.),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape_2,
            DrawMode::Stroke(StrokeMode::new(Color::RED, 2.0)),
            Transform::default(),
        ))
        .insert(AreaBoundary_1)
        .insert(AngleBase_1(0.));

    let shape_3 = shapes::Circle {
        radius: 35.,
        center: Vec2::new(0., 0.),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape_3,
            DrawMode::Stroke(StrokeMode::new(Color::RED, 2.0)),
            Transform::default(),
        ))
        .insert(AreaBoundary_2)
        .insert(AngleBase_2(0.));

    let shape_4 = shapes::Circle {
        radius: 15.,
        center: Vec2::new(0., 0.),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape_4,
            DrawMode::Stroke(StrokeMode::new(Color::RED, 2.0)),
            Transform::default(),
        ))
        .insert(AreaBoundary_3)
        .insert(AngleBase_3(0.));
}

fn update_system(
    mut commands: Commands,
    mut boundary_query_1: Query<
        (&mut Transform, &mut AngleBase_1),
        (
            With<AreaBoundary_1>,
            Without<AreaBoundary_2>,
            Without<AreaBoundary_3>,
        ),
    >,
    mut boundary_query_2: Query<
        (&mut Transform, &mut AngleBase_2),
        (With<AreaBoundary_2>, Without<AreaBoundary_3>),
    >,
    mut boundary_query_3: Query<(&mut Transform, &mut AngleBase_3), With<AreaBoundary_3>>,
) {
    let (mut trfm_1, mut angle_1) = boundary_query_1.get_single_mut().unwrap();
    // trfm.translation.x = trfm.translation.x + 0.1;
    // trfm.translation.y = trfm.translation.y + 0.1;
    trfm_1.translation.x = (0. + angle_1.0.sin() * 200.) as f32;
    trfm_1.translation.y = (0. + angle_1.0.cos() * 200.) as f32;

    let (mut trfm_2, mut angle_2) = boundary_query_2.get_single_mut().unwrap();
    trfm_2.translation.x = (trfm_1.translation.x as f64 + angle_2.0.sin() * 110.) as f32;
    trfm_2.translation.y = (trfm_1.translation.y as f64 + angle_2.0.cos() * 110.) as f32;

    let (mut trfm_3, mut angle_3) = boundary_query_3.get_single_mut().unwrap();
    trfm_3.translation.x = (trfm_2.translation.x as f64 + angle_3.0.sin() * 50.) as f32;
    trfm_3.translation.y = (trfm_2.translation.y as f64 + angle_3.0.cos() * 50.) as f32;

    angle_1.0 = angle_1.0 + 0.05;
    angle_2.0 = angle_2.0 + 0.1;
    angle_3.0 = angle_3.0 + 0.6;
}
