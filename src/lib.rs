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

#![deny(missing_docs)]
#![feature(alloc_system, box_syntax, const_fn, stmt_expr_attributes)]

//! This crate provides abstractions for libmpv of the mpv media player. It is dynamically checked
//! that no client is used after their parent exited, if no command that has similar behaviour to
//! `quit` was used.
//!
//! # Examples
//!
//! FIXME:
//! TO BE DONE

extern crate alloc_system;

extern crate encoding;
extern crate libc;
extern crate parking_lot;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;

/// Contains bindings to libmpv functions.
pub mod raw;
mod wrapper;
pub use wrapper::*;
#[cfg(test)]
mod tests;

#[allow(missing_docs)]
pub const MPV_CLIENT_API_MAJOR: u32 = 1;
#[allow(missing_docs)]
pub const MPV_CLIENT_API_MINOR: u32 = 21;
#[allow(missing_docs)]
pub const MPV_CLIENT_API_VERSION: u32 = mpv_make_version(MPV_CLIENT_API_MAJOR,
                                                         MPV_CLIENT_API_MINOR);
const fn mpv_make_version(major: u32, minor: u32) -> u32 {
    major << 16 | minor
}