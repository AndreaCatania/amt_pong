extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    window::{DisplayConfig, WindowBundle},
    renderer::{
        types::DefaultBackend, RenderingSystem,
        sprite::{SpriteSheet},
    },
    utils::application_root_dir,
    assets::Processor,
    ui::{DrawUiDesc, UiBundle,},
    input::{InputBundle, StringBindings},
};

use crate::render_graph::RenderGraph;


mod render_graph;
mod pong;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
       // .level_for("amethyst_rendy", amethyst::LogLevelFilter::Warn)
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_factory::factory", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_memory::allocator::dynamic", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_graph::node::render::pass", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_graph::node::present", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_graph::graph", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_memory::allocator::linear", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_wsi", amethyst::LogLevelFilter::Warn)
        .start();

    let display_config = "resources/display_config.ron";
    let display_config = DisplayConfig::load(&display_config);


    let input_bind_config = "resources/bindings_config.ron";
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(input_bind_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config(display_config))?
        // The renderer must be executed on the same thread consecutively,
        // so we initialize it as thread_local which will always execute on the main thread.
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ))
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        // A Processor system is added to handle loading sprite sheets.
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with_bundle(input_bundle)?
        .with(
            systems::PaddleSystem,
            "paddle_system",
        &["input_system"]);


    let mut game = Application::new("./", pong::Pong {}, game_data)?;

    game.run();

    println!("The game is correctly closed");
    Ok(())
}
