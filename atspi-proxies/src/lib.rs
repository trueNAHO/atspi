#![deny(clippy::all, clippy::pedantic, clippy::cargo, unsafe_code)]
#![allow(clippy::multiple_crate_versions)]

#[cfg(all(feature = "async-std", feature = "tokio"))]
compile_error!("You may not mix the async-std and tokio features.");

#[cfg(all(not(feature = "async-std"), not(feature = "tokio")))]
compile_error!("You must specify either the async-std or tokio feature.");

#[cfg(feature = "unstable-traits")]
use atspi_macros::atspi_proxy;

#[cfg(not(feature = "unstable-traits"))]
use zbus::dbus_proxy as atspi_proxy;

pub mod accessible;
pub mod action;
pub mod application;
pub mod bus;
pub mod cache;
pub mod collection;
pub mod component;
pub mod device_event_controller;
pub mod device_event_listener;
pub mod document;
pub mod editable_text;
pub use atspi_common::{events, CoordType, Interface, InterfaceSet};

pub mod hyperlink;
pub mod hypertext;
pub mod image;
pub mod registry;
pub mod selection;
pub mod socket;
pub mod table;
pub mod table_cell;
pub mod text;
pub mod value;

mod state;
pub use state::*;

pub mod error;
pub use error::AtspiError;

pub use zbus;

pub trait AtspiProxy {
	const INTERFACE: Interface;
}
