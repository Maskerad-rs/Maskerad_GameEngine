// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO: We'll need an async way of streaming an ogg file, and maybe a PoolAllocator.

use lewton::inside_ogg::OggStreamReader;
use std::io::{Read, Seek};

use resources::resource_errors::{ResourceError, ResourceResult};


pub struct SoundResource<T: Read + Seek>(OggStreamReader<T>);

impl<T: Read + Seek> From<OggStreamReader<T>> for SoundResource<T> {
    fn from(ogg_stream: OggStreamReader<T>) -> Self {
        SoundResource(ogg_stream)
    }
}

impl<T: Read + Seek> SoundResource<T> {
    //TODO: Doesn't work well for async I/O. See this: https://docs.rs/lewton/0.8.0/lewton/inside_ogg/async/index.html
    pub fn from_reader(reader: T) -> ResourceResult<SoundResource<T>> {
        let sound_stream = OggStreamReader::new(reader)?;
        Ok(SoundResource(sound_stream))
    }

    pub fn decompress_packet(&mut self) -> ResourceResult<Option<Vec<Vec<i16>>>> {
        self.0.read_dec_packet().map_err(|vorbis_error| {
            ResourceError::from(vorbis_error)
        })
    }
}