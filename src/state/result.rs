use resource::CurrentMap;
use hoppinworldruntime::{AllEvents, RuntimeProgress, RemovalId};
use amethyst_extra::AssetLoader;
use {add_removal_to_entity, Auth, sec_to_display, submit_score, ScoreInsertRequest};
use amethyst::ui::{UiCreator, UiTransform, UiText, UiEvent, Anchor, FontFormat, UiFinder, UiEventType};
use amethyst::utils::removal::{exec_removal, Removal};
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ecs::SystemData;
use amethyst::renderer::VirtualKeyCode;
use state::MapSelectState;

#[derive(Default)]
pub struct ResultState {
    finished: bool,
}

impl dynamic::State<MyState, AllEvents> for ResultState {
    fn on_start(&mut self, world: &mut World) {
        let ui_root = world
            .exec(|mut creator: UiCreator| creator.create("assets/base/prefabs/result_ui.ron", ()));
        add_removal_to_entity(ui_root, RemovalId::ResultUi, world);

        // Time table.
        let runtime_progress = world.read_resource::<RuntimeProgress>().clone();

        info!("SEGMENT TIMES: ");
        for t in &runtime_progress.segment_times {
            print!("{},", t);
        }
        info!("");
        info!("RUN DONE!");

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

        let mut accum = 0.0;
        for (segment, time) in runtime_progress.segment_times.iter().enumerate() {
            // Accum
            world.create_entity()
                .with(UiTransform::new(String::from(""), Anchor::TopMiddle, -200.0, -350.0 - 100.0 * segment as f32, 3.0, 200.0, 100.0, -1))
                .with(UiText::new(font.clone(), sec_to_display(*time), [0.1,0.1,0.1,1.0], 35.0))
                .with(Removal::new(RemovalId::ResultUi))
                .build();


            let diff = if *time == 0.0{
                0.0
            } else {
                *time - accum
            };
            if *time != 0.0 {
                accum = *time;
            }

            // Segment
            world.create_entity()
                .with(UiTransform::new(String::from(""), Anchor::TopMiddle, 200.0, -350.0 - 100.0 * segment as f32, 3.0, 200.0, 100.0, -1))
                .with(UiText::new(font.clone(), sec_to_display(diff), [0.1,0.1,0.1,1.0], 35.0))
                .with(Removal::new(RemovalId::ResultUi))
                .build();
        }

        // Web submit score if logged in
        if let Some(auth_token) = world.res.try_fetch::<Auth>().map(|a| a.token.clone()) {
            let times = runtime_progress.segment_times.iter().map(|f| *f as f32);
            let total_time = runtime_progress.segment_times.iter().map(|f| *f as f32).last().unwrap();
            let insert = ScoreInsertRequest {
                mapid: 1,
                segment_times: times.collect(),
                strafes: 0,
                jumps: 0,
                total_time: total_time,
                max_speed: 0.0,
                average_speed: 0.0,
            };

            submit_score(&mut world.write_resource(), &world.read_resource(), auth_token, insert);
        }
    }

    fn update(&mut self, world: &mut World) -> Trans<MyState> {
        if !self.finished {
            // Set the map name
            if let Some(map_name_entity) = UiFinder::fetch(&world.res).find("map_name") {
                let map_name = world.read_resource::<CurrentMap>().1.name.clone();
                world.write_storage::<UiText>().get_mut(map_name_entity).unwrap().text = map_name;
                self.finished = true;
            }
        }

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
                        "back_button" => Trans::Switch(MyState::MapSelect),
                        _ => Trans::None,
                    }
                } else {
                    Trans::None
                }
            }
            AllEvents::Window(ev) => {
                if is_key_down(&ev, VirtualKeyCode::Escape) {
                    Trans::Switch(MyState::MapSelect)
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
            RemovalId::ResultUi,
        );
    }
}
