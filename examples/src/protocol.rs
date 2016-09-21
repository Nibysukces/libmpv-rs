// Copyright (C) 2016  ParadoxSpiral
//
// This file is part of mpv-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

use mpv;
use mpv::{MpvInstance, Parent, PlaylistOp};
use mpv::utils;
use mpv::protocol::*;

use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;
use std::slice;
use std::time::Duration;
use std::thread;

fn open(_: &mut (), uri: &str) -> File {
    // Open the file, and strip the `filereader://` part
    let ret = File::open(&uri[13..]).unwrap();

    println!("Opened file[{}], ready for orders o7", &uri[13..]);
    ret
}

fn close(_: Box<File>) {
    println!("Closing file, bye bye~~");
}

fn read(cookie: &mut File, buf: *mut i8, nbytes: u64) -> i64 {
    unsafe {
        let slice = slice::from_raw_parts_mut(buf, nbytes as _);
        let forbidden_magic = mem::transmute::<&mut [i8], &mut [u8]>(slice);

        cookie.read(forbidden_magic).unwrap() as _
    }    
}

fn seek(cookie: &mut File, offset: i64) -> i64 {
    cookie.seek(SeekFrom::Start(offset as u64)).unwrap() as _
}

fn size(cookie: &mut File) -> i64 {
    cookie.metadata().unwrap().len() as _
}

pub fn exec() {
    let path = format!("filereader://{}",
                       env::args()
                           .nth(1)
                           .expect("Expected local path, found nil."));

    let protocol = unsafe { Protocol::new("filereader".into(),
                                 (),
                                 open,
                                 close,
                                 read,
                                 Some(seek),
                                 Some(size)) };

    let mpv = Parent::new(false).unwrap();
    mpv.register_protocol(protocol).unwrap();

    mpv.set_property("volume", 30).unwrap();

    mpv.playlist(&PlaylistOp::Loadfiles(&[utils::File::new(Path::new(&path),
                                                           utils::FileState::AppendPlay,
                                                           None)]))
       .unwrap();

    thread::sleep(Duration::from_secs(10));

    mpv.seek(&mpv::Seek::RelativeForward(Duration::from_secs(15))).unwrap();

    thread::sleep(Duration::from_secs(5));
}