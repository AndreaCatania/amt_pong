extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    window::{DisplayConfig, WindowBundle},
    renderer::{types::DefaultBackend, RenderingSystem},
    utils::application_root_dir,
};

use crate::render_graph::RenderGraph;


mod render_graph;
mod pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    //let path = format!("{}/resources/display_config.ron", application_root_dir());
    let path = "resources/display_config.ron";
    let display_config = DisplayConfig::load(&path);


    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config(display_config))?
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ))
        .with_bundle(TransformBundle::new())?;


    let mut game = Application::new("./", pong::Pong {}, game_data)?;

    game.run();

    println!("The game is correctly closed");
    Ok(())
}
