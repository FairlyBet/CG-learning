use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

// Components
#[derive(Component)]
struct GraphNode {
    id: String,
    label: String,
}

#[derive(Component)]
struct GraphEdge {
    from: String,
    to: String,
    label: Option<String>,
}

#[derive(Component)]
struct NodeText {
    world_position: Vec3,
}

#[derive(Component)]
struct EdgeText {
    world_position: Vec3,
}

// Graph data structure
struct GraphData {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

struct NodeData {
    id: String,
    label: String,
    position: Vec3,
    color: Color,
    shape: NodeShape,
}

struct EdgeData {
    from: String,
    to: String,
    label: Option<String>,
}

#[derive(Clone)]
enum NodeShape {
    Cube,
    Sphere,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_text_positions))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Set background color to RGB(160, 160, 160)
    commands.insert_resource(ClearColor(Color::srgb(
        160.0 / 255.0,
        160.0 / 255.0,
        160.0 / 255.0,
    )));

    // Create sample graph data
    let graph = create_sample_graph();

    // 3D Camera
    commands.spawn(Camera3dBundle {
        camera: Camera {
            order: 0, // Set 3D camera to render first
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a 2D camera for UI/text
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1, // Set 2D camera to render second
            ..default()
        },
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        ))
        .with_translation(Vec3::new(5.0, 5.0, 5.0)),
        ..default()
    });

    // Store node positions for edge creation
    let mut node_positions = HashMap::new();

    // Create nodes
    for node in &graph.nodes {
        let mesh = match node.shape {
            NodeShape::Cube => meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            NodeShape::Sphere => meshes.add(Sphere::new(0.5)),
        };

        let material = materials.add(StandardMaterial {
            base_color: node.color,
            ..default()
        });

        // Spawn node mesh
        let node_entity = commands
            .spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(node.position),
                    ..default()
                },
                GraphNode {
                    id: node.id.clone(),
                    label: node.label.clone(),
                },
            ))
            .id();

        // Create text above the node
        commands.spawn((
            // Text2dBundle {
            //     text: Text::from_section(
            //         &node.label,
            //         TextStyle {
            //             font_size: 24.0,
            //             color: Color::WHITE,
            //             ..Default::default()
            //         },
            //     )
            //     .with_justify(JustifyText::Center),
            //     transform: Transform::from_xyz(0.0, 0.0, 0.0), // Will be updated by system
            //     ..default()
            // },
            NodeText {
                world_position: node.position + Vec3::new(0.0, 1.5, 0.0),
            },
        ));

        node_positions.insert(node.id.clone(), node.position);
    }

    // Create edges
    for edge in &graph.edges {
        if let (Some(&from_pos), Some(&to_pos)) =
            (node_positions.get(&edge.from), node_positions.get(&edge.to))
        {
            create_edge(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                from_pos,
                to_pos,
                edge,
            );
        }
    }
}

fn create_edge(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    from: Vec3,
    to: Vec3,
    edge_data: &EdgeData,
) {
    let direction = to - from;
    let length = direction.length();
    let center = (from + to) / 2.0;

    // Create a thin cylinder as the edge line
    let mesh = meshes.add(Cylinder::new(0.02, length));
    let material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        ..default()
    });

    // Calculate rotation to align cylinder with edge direction
    let up = Vec3::Y;
    let rotation = if direction.normalize().dot(up).abs() > 0.99 {
        // Handle edge case where direction is parallel to up vector
        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
    } else {
        Quat::from_rotation_arc(up, direction.normalize())
    };

    // Spawn edge mesh
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(center).with_rotation(rotation),
            ..default()
        },
        GraphEdge {
            from: edge_data.from.clone(),
            to: edge_data.to.clone(),
            label: edge_data.label.clone(),
        },
    ));

    // Add edge label if present
    if let Some(label) = &edge_data.label {
        commands.spawn((
            // Text2dBundle {
            //     text: Text::from_section(
            //         label,
            //         TextStyle {
            //             font_size: 16.0,
            //             color: Color::WHITE,
            //             ..Default::default()
            //         },
            //     )
            //     .with_justify(JustifyText::Center),
            //     transform: Transform::from_translation(center + Vec3::new(0.0, 0.3, 0.0)),
            //     ..default()
            // },
            EdgeText {
                world_position: center + Vec3::new(0.0, 0.3, 0.0),
            },
        ));
    }
}

fn create_sample_graph() -> GraphData {
    GraphData {
        nodes: vec![
            NodeData {
                id: "1".to_string(),
                label: "192.168.0.1\n".to_string(),
                position: Vec3::new(-5.0, -1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "2".to_string(),
                label: "192.0.2.9".to_string(),
                position: Vec3::new(-4.0, 1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "3".to_string(),
                label: "192.0.2.14".to_string(),
                position: Vec3::new(-3.0, -1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "4".to_string(),
                label: "10.0.0.1".to_string(),
                position: Vec3::new(-2.0, 1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "5".to_string(),
                label: "172.16.10.1".to_string(),
                position: Vec3::new(-1.0, -1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "6".to_string(),
                label: "203.0.113.1".to_string(),
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "7".to_string(),
                label: "203.0.113.254".to_string(),
                position: Vec3::new(1.0, -1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "8".to_string(),
                label: "198.51.100.1".to_string(),
                position: Vec3::new(2.0, 1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
            NodeData {
                id: "9".to_string(),
                label: "93.184.216.34".to_string(),
                position: Vec3::new(3.0, -1.0, 0.0),
                color: Color::srgb(0.8, 0.8, 0.2),
                shape: NodeShape::Sphere,
            },
        ],
        edges: vec![
            EdgeData {
                from: "1".to_string(),
                to: "2".to_string(),
                label: Some("Edge 1".to_string()),
            },
            EdgeData {
                from: "2".to_string(),
                to: "3".to_string(),
                label: Some("Edge 2".to_string()),
            },
            EdgeData {
                from: "3".to_string(),
                to: "4".to_string(),
                label: None,
            },
            EdgeData {
                from: "4".to_string(),
                to: "5".to_string(),
                label: Some("Edge 4".to_string()),
            },
            EdgeData {
                from: "5".to_string(),
                to: "6".to_string(),
                label: Some("Edge 4".to_string()),
            },
            EdgeData {
                from: "6".to_string(),
                to: "7".to_string(),
                label: Some("Edge 4".to_string()),
            },
            EdgeData {
                from: "7".to_string(),
                to: "8".to_string(),
                label: Some("Edge 4".to_string()),
            },
            EdgeData {
                from: "8".to_string(),
                to: "9".to_string(),
                label: Some("Edge 4".to_string()),
            },
        ],
    }
}

fn rotate_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera3d>>) {
    for mut transform in camera_query.iter_mut() {
        let radius = 10.0;
        let angle = time.elapsed_seconds() * 0.3;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        transform.translation = Vec3::new(x, 5.0, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn update_text_positions(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut text_queries: ParamSet<(
        Query<(&mut Transform, &NodeText)>,
        Query<(&mut Transform, &EdgeText)>,
    )>,
    primary_window: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();

    // Get the window dimensions from the primary window
    if let Ok(window) = primary_window.get_single() {
        // Update node text positions
        for (mut text_transform, text) in text_queries.p0().iter_mut() {
            if let Some(screen_pos) =
                camera.world_to_viewport(camera_transform, text.world_position)
            {
                text_transform.translation.x = screen_pos.x - window.width() / 2.0;
                text_transform.translation.y = screen_pos.y - window.height() / 2.0;
            }
        }

        // Update edge text positions
        for (mut text_transform, text) in text_queries.p1().iter_mut() {
            if let Some(screen_pos) =
                camera.world_to_viewport(camera_transform, text.world_position)
            {
                text_transform.translation.x = screen_pos.x - window.width() / 2.0;
                text_transform.translation.y = screen_pos.y - window.height() / 2.0;
            }
        }
    }
}
