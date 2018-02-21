# Maskerad Game Engine
[![Build Status](https://travis-ci.org/Maskerad-rs/Maskerad_GameEngine.svg?branch=master)](https://travis-ci.org/Maskerad-rs/Maskerad_GameEngine)
[![Build status](https://ci.appveyor.com/api/projects/status/7r2oyqmqr28d3xya/branch/master?svg=true)](https://ci.appveyor.com/project/Malkaviel/maskerad-gameengine/branch/master)
[![codecov](https://codecov.io/gh/Maskerad-rs/Maskerad_GameEngine/branch/master/graph/badge.svg)](https://codecov.io/gh/Maskerad-rs/Maskerad_GameEngine)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

game engine written in Rust.

***Early development, not usable for the moment.***

### Planned Features:

#### Linux and Windows support

#### Multithreaded engine
Based on a **job model**, instead of a system model.

#### 3D game engine
Powered by the **Vulkan** graphics API.

#### Scripting integration
The language of choice for game scripting is **Lua**. The reasons are :

- Small (24000 lines of code)
- Fast (Especially the **LuaJIT** implementation)
- Easily embeddable
- Simple (easy to read, well documented, "everything is a table")

#### Highly decoupled architecture.
Modifying/Adding/Removing/Replacing parts of the engine must be, more-or-less, painless.

#### Client/server architecture
for games **and tools**.

There is a server-side engine, and a variety of clients like
the client-side engine or some tools.
 
For tools, the server and the clients communicate through TCP/UDP/HTTP.
This architecture allows to develop tools decoupled from the engine, with any language or framework.

For games, the client and the server communicate through UDP, even single player games.

The reasoning behind this comes from Quake II. In this game, the single player part is just
like a multiplayer match, but the client and the server are on the same computer. The single player
part of the game is just a *special* case of the multiplayer part. 

Which makes a lot of sense.

#### Plugin/Module system
The engine must be extendable through plugin or modules. A plugin would be a
game specific subsystem written in Lua, while a module would be a dynamically linked rust library to
extend the "core" of the engine.


This engine borrows a lot of ideas from the, now discontinued, [Bitsquid engine](https://www.autodesk.com/products/stingray/overview).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

