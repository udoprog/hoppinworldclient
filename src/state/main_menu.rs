
use amethyst_extra::set_discord_state;
use add_removal_to_entity;
use amethyst::prelude::*;
use amethyst::utils::removal::*;
use amethyst::ui::*;
use state::*;
use hoppinworldruntime::{AllEvents, RemovalId};

#[derive(Default)]
pub struct MainMenuState;

impl dynamic::State<MyState, AllEvents> for MainMenuState {
    fn on_start(&mut self, world: &mut World) {
        let ui_root = world
            .exec(|mut creator: UiCreator| creator.create("assets/base/prefabs/menu_ui.ron", ()));
        add_removal_to_entity(ui_root, RemovalId::MenuUi, world);

        set_discord_state(String::from("Main Menu"), world);
    }

    fn handle_event(
        &mut self,
        world: &mut World,
        event: AllEvents,
    ) -> Trans<MyState> {
        match event {
            AllEvents::Ui(UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            }) => {
                if let Some(ui_transform) = world.read_storage::<UiTransform>().get(entity) {
                    match &*ui_transform.id {
                        "play_button" => Trans::Switch(MyState::MapSelect),
                        "quit_button" => Trans::Quit,
                        _ => Trans::None,
                    }
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, world: &mut World) {
        exec_removal(
            &world.entities(),
            &world.read_storage(),
            RemovalId::MenuUi,
        );
    }
}
