//We have the trait : Subsystem.
//We have the traits : Audio, Log, Rendering... bound to the Subsystem trait.
//We have SubsystemType : An enum indicating the subsystem_type.
//We have (in the Systems_layer), the actual systems implementing the traits.
//We have a subsystem_context: an hashmap owing the subsystems, passed as argument to the KindredEngine struct

//Subsystem trait : start_up(), shut_down() and builder pattern.
//We can try to use closures to use subsystems in the subsystem_locator.
pub mod loggers;
pub mod system_types;
pub mod filesystems; //The filesystem interfaces are largely inspired by GGEZ's VFS at this time.