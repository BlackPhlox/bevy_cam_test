use bevy::{prelude::*, render::camera::{ActiveCameras, CameraPlugin}};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_state(CameraState::Camera1)
        .add_startup_system(setup)
        .add_system(switch_cameras)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum CameraState {
    Camera1,
    Camera2,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(Camera1);

    //Defaults to the latest added camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(Camera2);
}

#[derive(Component)]
struct Camera1;
#[derive(Component)]
struct Camera2;


fn switch_cameras(
    mut active_cameras: ResMut<ActiveCameras>,
    mut cam_state: ResMut<State<CameraState>>,
    keys: Res<Input<KeyCode>>,
    mut query: QuerySet<(
        QueryState<(&Camera1, &mut Camera)>,
        QueryState<(&Camera2, &mut Camera)>,
    )>,
    mut query1: Query<&mut Camera>
) {
    let q0 = query.q0().single().1;
    let q1 = query.q1().single().1;
    
    if keys.just_pressed(KeyCode::C) {
        if cam_state.current().eq(&CameraState::Camera1){
            let removed_cam = active_cameras.remove(CameraPlugin::CAMERA_3D);
            
            if removed_cam.is_some() {
                let r = removed_cam.unwrap();
                if r.entity.is_some() {
                    let t = removed_cam.unwrap().entity;
                    query1.get();
                }
            }
            
            removed_cam.
            active_cameras.add(CameraPlugin::CAMERA_3D);
        }
    }
    /*
    if settings.locked_to_player && !settings.ltp {
        act_cams.remove("Camera3d");

        let (_, mut b) = query.q1_mut().single_mut().unwrap();
        b.name = Some("Camera3d".to_string());

        act_cams.add("Camera3d");

        let (_, mut b) = query.q0_mut().single_mut().unwrap();
        b.name = Some("Test".to_string());

        settings.ltp = true;
    } else if !settings.locked_to_player && settings.ltp {
        act_cams.remove("Camera3d");

        let (_, mut b) = query.q0_mut().single_mut().unwrap();
        b.name = Some("Camera3d".to_string());

        act_cams.add("Camera3d");

        let (_, mut b) = query.q1_mut().single_mut().unwrap();
        b.name = Some("Test".to_string());

        settings.ltp = false;
    }
    */
}