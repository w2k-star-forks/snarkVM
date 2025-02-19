// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::too_many_arguments)]
#![warn(clippy::cast_possible_truncation)]
#![cfg_attr(test, allow(clippy::assertions_on_result_states))]

#[macro_use]
extern crate enum_index_derive;

pub use snarkvm_console_network::Network;
pub use snarkvm_console_types::prelude::*;

mod data;
pub use data::*;

mod data_types;
pub use data_types::*;

mod id;
pub use id::*;

mod locator;
pub use locator::*;

mod request;
pub use request::*;

mod response;
pub use response::*;
