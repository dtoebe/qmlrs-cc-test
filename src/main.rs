#[macro_use]
extern crate qmlrs;

mod ui3;
mod ui2;

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.load_local_file("assets/ui1.qml");

    engine = ui2::ui2(engine);
    engine = ui3::ui3::ui3(engine);

    engine.exec();
}
