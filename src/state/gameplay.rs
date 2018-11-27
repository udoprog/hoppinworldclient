
use amethyst::controls::HideCursor;
use amethyst::core::Time;
use amethyst_rhusics::time_sync;
use amethyst::prelude::*;
use amethyst::utils::removal::*;
use amethyst::input::*;
use amethyst::renderer::VirtualKeyCode;
use state::*;
use hoppinworldruntime::{AllEvents, CustomStateEvent, RemovalId};

#[derive(Default)]
pub struct GameplayState;

impl dynamic::State<MyState, AllEvents> for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.write_resource::<HideCursor>().hide = true;
        data.world.write_resource::<Time>().set_time_scale(1.0);
    }

    fn update(&mut self, world: &mut World) -> Trans<MyState> {
        //info!("FPS: {}", data.world.read_resource::<FPSCounter>().sampled_fps());
        //info!("Delta: {}", data.world.read_resource::<Time>().delta_seconds());
        //(&data.world.read_storage::<Transform>(), &data.world.read_storage::<ObjectType>()).join().filter(|t| *t.1 == ObjectType::Player).for_each(|t| info!("{:?}", t));

        time_sync(world);
        Trans::None
    }

    fn handle_event(
        &mut self,
        _world: &mut World,
        event: &AllEvents,
    ) -> Trans<MyState> {
        // TODO: Map finished
        match event {
            AllEvents::Window(ev) => {
                if is_key_down(&ev, VirtualKeyCode::Escape) {
                    Trans::Push(MyState::Pause)
                } else {
                    Trans::None
                }
            }
            AllEvents::Custom(CustomStateEvent::GotoMainMenu) => {
                Trans::Switch(MyState::MapSelect)
            }
            AllEvents::Custom(CustomStateEvent::MapFinished) => {
                Trans::Switch(MyState::Result)
            },
            AllEvents::Custom(CustomStateEvent::Retry) => {
                Trans::Switch(MyState::MapLoad)
            }
            _ => Trans::None,
        }
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        data.world.write_resource::<HideCursor>().hide = false;
        data.world.write_resource::<Time>().set_time_scale(0.0);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        data.world.write_resource::<HideCursor>().hide = true;
        data.world.write_resource::<Time>().set_time_scale(1.0);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.write_resource::<HideCursor>().hide = false;
        // Not sure if I should put 0. Might cause errors later when implementing replays and stuff.
        data.world.write_resource::<Time>().set_time_scale(1.0);
        exec_removal(
            &data.world.entities(),
            &data.world.read_storage(),
            RemovalId::Scene,
        );
        exec_removal(
            &data.world.entities(),
            &data.world.read_storage(),
            RemovalId::GameplayUi,
        );
        
        // TODO for retry, can remove?
        data.world.maintain();
    }
}
