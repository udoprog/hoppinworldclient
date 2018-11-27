
use amethyst_extra::{AssetLoader, set_discord_state};
use resource::{MapInfoCache, CurrentMap};
use add_removal_to_entity;
use amethyst::prelude::*;
use amethyst::utils::removal::*;
use amethyst::input::*;
use amethyst::ui::*;
use state::*;
use amethyst::renderer::VirtualKeyCode;
use hoppinworldruntime::{AllEvents, RemovalId};

#[derive(Default)]
pub struct MapSelectState;

impl dynamic::State<MyState, AllEvents> for MapSelectState {
    fn on_start(&mut self, mut world: &mut World) {
        let ui_root = world.exec(|mut creator: UiCreator| {
            creator.create("assets/base/prefabs/map_select_ui.ron", ())
        });
        add_removal_to_entity(ui_root, RemovalId::MapSelectUi, world);

        let font = world
            .read_resource::<AssetLoader>()
            .load(
                "font/arial.ttf",
                FontFormat::Ttf,
                (),
                &mut world.write_resource(),
                &mut world.write_resource(),
                &world.read_resource(),
            ).expect("Failed to load font");
        let maps = world.read_resource::<MapInfoCache>().maps.clone();
        for (accum, (internal, info)) in maps.into_iter().enumerate() {
            info!("adding map!");
            let entity =
                UiButtonBuilder::new(format!("map_select_{}", internal), info.name.clone())
                    .with_font(font.clone())
                    .with_text_color([0.2, 0.2, 0.2, 1.0])
                    .with_font_size(30.0)
                    .with_size(512.0, 75.0)
                    .with_layer(8.0)
                    .with_position(0.0, -300.0 - 100.0 * accum as f32)
                    .with_anchor(Anchor::TopMiddle)
                    .build_from_world(world);
            add_removal_to_entity(entity, RemovalId::MapSelectUi, world);
        }

        set_discord_state(String::from("Main Menu"), world);
    }

    fn handle_event(
        &mut self,
        world: &mut World,
        event: &AllEvents,
    ) -> Trans<MyState> {
        let mut change_map = None;
        match event {
            AllEvents::Ui(UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            }) => {
                if let Some(ui_transform) = world.read_storage::<UiTransform>().get(entity) {
                    match &*ui_transform.id {
                        "back_button" => {
                            return Trans::Switch(MyState::MainMenu);
                        }
                        id => {
                            if id.starts_with("map_select_") {
                                let map_name = &id[11..];
                                change_map = Some(
                                    world
                                        .read_resource::<MapInfoCache>()
                                        .maps
                                        .iter()
                                        .find(|t| t.0 == map_name)
                                        .unwrap()
                                        .clone(),
                                );
                            }
                        }
                    }
                }
            }
            AllEvents::Window(ev) => {
                if is_key_down(&ev, VirtualKeyCode::Escape) {
                    return Trans::Switch(MyState::MainMenu);
                }
            }
            _ => {}
        }

        if let Some(row) = change_map {
            world.add_resource(CurrentMap::new(row.0, row.1));
            return Trans::Switch(MyState::MapLoad);
        }
        Trans::None
    }

    fn on_stop(&mut self, world: &mut World) {
        exec_removal(
            &world.entities(),
            &world.read_storage(),
            RemovalId::MapSelectUi,
        );
    }
}
