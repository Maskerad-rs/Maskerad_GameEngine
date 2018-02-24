// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fs;
use std::fmt;

// We need our own version of this structure because the one in
// std annoyingly doesn't let you get data out of it.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    append: bool,
    truncate: bool,
}

impl AsRef<OpenOptions> for OpenOptions {
    fn as_ref(&self) -> &OpenOptions {
        self
    }
}

impl fmt::Display for OpenOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rights = String::new();
        if self.read {
            rights.push_str("read, ");
        }
        if self.write {
            rights.push_str("write, ");
        }
        if self.create {
            rights.push_str("create, ");
        }
        if self.append {
            rights.push_str("append, ");
        }
        if self.truncate {
            rights.push_str("truncate");
        }

        write!(f, "[{}]", rights)
    }
}

impl OpenOptions {
    // Create a new instance
    pub fn new() -> OpenOptions {
        debug!("Creating an OpenOptions.");
        Default::default()
    }

    // Open for reading
    pub fn set_read(&mut self, read: bool) -> &mut OpenOptions {
        debug!("Setting the read option of the OpenOptions to {}", read);
        self.read = read;
        self
    }

    // Open for writing
    pub fn set_write(&mut self, write: bool) -> &mut OpenOptions {
        debug!("Setting the write option of the OpenOptions to {}", write);
        self.write = write;
        self
    }

    // Create the file if it does not exist yet
    pub fn set_create(&mut self, create: bool) -> &mut OpenOptions {
        debug!("Setting the create option of the OpenOptions to {}", create);
        self.create = create;
        self
    }

    // Append at the end of the file
    pub fn set_append(&mut self, append: bool) -> &mut OpenOptions {
        debug!("Setting the append option of the OpenOptions to {}", append);
        self.append = append;
        self
    }

    // Truncate the file to 0 bytes after opening
    pub fn set_truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        debug!("Setting the truncate option of the OpenOptions to {}", truncate);
        self.truncate = truncate;
        self
    }

    pub fn to_fs_openoptions(&self) -> fs::OpenOptions {
        debug!("Creating an fs::OpenOptions from this OpenOptions.");
        let mut opt = fs::OpenOptions::new();
        opt.read(self.read)
            .write(self.write)
            .create(self.create)
            .append(self.append)
            .truncate(self.truncate)
            .create(self.create);
        opt
    }
}
