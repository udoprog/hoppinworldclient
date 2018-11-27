
use amethyst_rhusics::time_sync;
use add_removal_to_entity;
use amethyst::prelude::*;
use amethyst::utils::removal::*;
use amethyst::input::*;
use amethyst::ui::*;
use amethyst::shrev::EventChannel;
use amethyst::renderer::VirtualKeyCode;
use hoppinworldruntime::{AllEvents, CustomStateEvent, RemovalId};

#[derive(Default)]
pub struct PauseMenuState;

impl dynamic::State<MyState, AllEvents> for PauseMenuState {
    fn on_start(&mut self, world: &mut World) {
        let ui_root = world
            .exec(|mut creator: UiCreator| creator.create("assets/base/prefabs/pause_ui.ron", ()));
        add_removal_to_entity(ui_root, RemovalId::PauseUi, world);
    }

    fn update(&mut self, world: &mut World) -> Trans<MyState> {
        // Necessary otherwise rhusics will keep the same DeltaTime and will not be paused.
        time_sync(world);
        Trans::None
    }

    fn handle_event(
        &mut self,
        world: &mut World,
        event: &AllEvents,
    ) -> Trans<MyState> {
        match event {
            AllEvents::Ui(UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            }) => {
                if let Some(ui_transform) = world.read_storage::<UiTransform>().get(entity) {
                    match &*ui_transform.id {
                        "resume_button" => Trans::Pop,
                        "retry_button" => {
                            world
                                .write_resource::<EventChannel<CustomStateEvent>>()
                                .single_write(CustomStateEvent::Retry);
                            Trans::Pop
                        }
                        "quit_button" => {
                            world
                                .write_resource::<EventChannel<CustomStateEvent>>()
                                .single_write(CustomStateEvent::GotoMainMenu);
                            Trans::Pop
                        }
                        _ => Trans::None,
                    }
                } else {
                    Trans::None
                }
            }
            AllEvents::Window(ev) => {
                if is_key_down(&ev, VirtualKeyCode::Escape) {
                    Trans::Pop
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
            RemovalId::PauseUi,
        );
    }
}
