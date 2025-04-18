use crate::simulation::ship::{load_ship_from_json, save_ship_to_json};
use crate::simulation::{build_schedule, build_world, run_simulation};
use std::path::PathBuf;

mod app;
mod simulation;

fn main() {
    let mut world = build_world(20);
    let mut schedule = build_schedule();

    let ship_path = PathBuf::from("./test.json");
    let ship_json = std::fs::read_to_string(ship_path).unwrap();
    let ship_entity = load_ship_from_json(&ship_json, &mut world).unwrap();
    println!("Ship entity: {:?}", ship_entity);

    let ship_save_path = PathBuf::from("./test_save.json");
    let json = save_ship_to_json(ship_entity, &mut world).unwrap();
    std::fs::write(ship_save_path, json).unwrap();

    run_simulation(ship_entity, &mut world, &mut schedule);

    //let options = eframe::NativeOptions {
    //    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    //    ..Default::default()
    //};
    //
    //eframe::run_native(
    //    "Spaceship",
    //    options,
    //    Box::new(|_cc| Ok(Box::<SpaceshipApp>::default())),
    //)
}
