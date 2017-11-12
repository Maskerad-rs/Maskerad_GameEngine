//We have the trait : Subsystem.
//We have the traits : Audio, Log, Rendering... bound to the Subsystem trait.
//We have SubsystemType : An enum indicating the subsystem_type.
//We have (in the Systems_layer), the actual systems implementing the traits.
//We have a subsystem_context: an hashmap owing the subsystems, passed as argument to the KindredEngine struct

//Subsystem trait : start_up(), shut_down() and builder pattern (finalizing with start_up()).
//We can try to use closures to use subsystems in the subsystem_locator.


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum SystemType {
    Audio,
    Graphic,
    Physic,
    Log,
    Resource,
    Input,
}

pub trait System {
    fn start_up(&mut self);
    fn shut_down(&mut self);
}

pub trait Log : System {
    fn subsystem_type() -> SystemType {SystemType::Log} //The type of the subsystem
    fn write_to_general_log(&mut self); //The main log file, when the game runs.
    fn write_to_dedicated_log(&mut self, subsystem_type: SystemType); //The dedicated log file to a struct.
}