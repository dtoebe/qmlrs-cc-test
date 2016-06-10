extern crate qmlrs;

pub fn ui2(mut engine: qmlrs::Engine) -> qmlrs::Engine {
    engine.load_local_file("assets/ui2.qml");

    engine
}
