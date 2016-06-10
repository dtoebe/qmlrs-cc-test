extern crate qmlrs;

pub fn ui3(mut engine: qmlrs::Engine) -> qmlrs::Engine {
    engine.load_local_file("assets/ui3.qml");

    engine
}
