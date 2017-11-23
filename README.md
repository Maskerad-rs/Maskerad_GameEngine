[![Build Status](https://travis-ci.org/Malkaviel/Chinstrap_Engine.svg?branch=master)](https://travis-ci.org/Malkaviel/Chinstrap_Engine)
[![Build status](https://ci.appveyor.com/api/projects/status/4dowa31sf4mgmgrb/branch/master?svg=true)](https://ci.appveyor.com/project/Malkaviel/kindredengine/branch/master)
[![codecov](https://codecov.io/gh/Malkaviel/Chinstrap_Engine/branch/master/graph/badge.svg)](https://codecov.io/gh/Malkaviel/Chinstrap_Engine)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


# WARNING
This is a personal project for learning game engine architecture, game engine implementation, and Rust.

The purpose and general architecture is still not defined :
 - General-purpose or highly genre specific ?
 - features ?
 - "keep it simple", or "bulletproof" ?
 - ...
 
 It is far from being usable at this time, but the idea is to implement a game engine, see what works well,
 what should be refactored and what should become its own library (which should be pushed on crates.io).
 
 If you are looking for a promising Rust game engine or game engine resources, just take a look at : 
 http://arewegameyet.com/index.html

# Chinstrap game engine
game engine written in Rust.

### Math library :
The engine uses the 'cgmath' crate, which provides the fundamental tools for computer graphics (Vectors, Matrices, Quaternions...).
cgmath uses the simd crate to take advantage of SIMD instructions for calculations.
It also uses Serde, a popular serialization/deserialization library.
A very basic Xorshift random number generator is also included, using the 'rand' crate.


