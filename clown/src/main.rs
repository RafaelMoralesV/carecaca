use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("balatro-card-sprites.png");
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 71, y: 95 }, 13, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 12,
            },
        ),
        Transform::from_scale(Vec3::splat(2.0)),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::BLACK))
        .run();

    let addr = "localhost:3001";

    if let Ok(mut stream) = TcpStream::connect(addr) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buf = String::new();

        reader.read_line(&mut buf).unwrap();
        println!("{buf}");
        buf.clear();
        stream.write_all(b"quit\n").unwrap();
        reader.read_line(&mut buf).unwrap();
        println!("{buf}");
        buf.clear();

        stream.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
}
