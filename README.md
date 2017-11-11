[![Build Status](https://travis-ci.org/Malkaviel/KindredEngine.svg?branch=master)](https://travis-ci.org/Malkaviel/KindredEngine)  [![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![codecov](https://codecov.io/gh/Malkaviel/KindredEngine/branch/master/graph/badge.svg)](https://codecov.io/gh/Malkaviel/KindredEngine)
# Kindred game engine
Experimental game engine written in Rust, far from being usable at this time.

### Math library :
The engine uses the 'cgmath' crate, which provides the fundamental tools for computer graphics (Vectors, Matrices, Quaternions...).
cgmath uses the simd crate to take advantage of SIMD instructions for calculations.
It also uses Serde, a popular serialization/deserialization library.
A very basic Xorshift random number generator is also included, using the 'rand' crate.


