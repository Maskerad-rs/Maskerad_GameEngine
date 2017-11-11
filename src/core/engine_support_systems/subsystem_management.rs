//Should subsystems have a mandatory start_up() and shut_down() methods ?

/*
To give access to our subsystems to other structs, we have 3 choices :
- Singleton
(not a good idea, and annoying to implement in Rust from what i know (juste see the lazy_static crate))

- Create a Game / GameContext struct which has our systems and pass them in functions
(bad idea, who wants to see a logger as argument in a rendering function ?)

- A service locator, probably the lesser evil.

Here's the idea :

- We create a 'Service' trait, which our subsystems implement.
This trait give functions to use the subsystem.

- We create 'Service providers' (our susbsystems) to implement this Service trait.

- A 'Service Locator' provides access to the service by finding the correct provider.


*/
