/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(register_tool)]
#![deny(unsafe_code)]
#![doc = "The script crate contains all matters DOM."]
// Register the linter `crown`, which is the Servo-specific linter for the script
// crate. Issue a warning if `crown` is not being used to compile, but not when
// building rustdoc or running clippy.
#![register_tool(crown)]
#![cfg_attr(any(doc, clippy), allow(unknown_lints))]
#![deny(crown_is_not_used)]

// These are used a lot so let's keep them for now
#[macro_use]
extern crate js;
#[macro_use]
extern crate jstraceable_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate malloc_size_of_derive;
#[macro_use]
extern crate servo_atoms;

mod animation_timeline;
mod animations;
#[macro_use]
mod task;
mod body;
pub mod clipboard_provider;
pub mod conversions;
mod devtools;
pub mod document_loader;
#[macro_use]
mod dom;
mod canvas_state;
pub mod fetch;
mod image_listener;
mod init;
mod layout_image;

pub mod document_collection;
pub mod iframe_collection;
pub mod layout_dom;
mod mem;
#[allow(unsafe_code)]
pub mod messaging;
mod microtask;
mod network_listener;
mod realms;
mod script_module;
pub mod script_runtime;
#[allow(unsafe_code)]
pub mod script_thread;
pub mod security_manager;
pub mod serviceworker_manager;
mod stylesheet_loader;
mod stylesheet_set;
mod task_manager;
mod task_queue;
mod task_source;
pub mod test;
pub mod textinput;
mod timers;
mod unpremultiplytable;
mod webdriver_handlers;
mod window_named_properties;

mod unminify;

mod drag_data_store;
mod links;
mod xpath;

pub use init::init;
pub use script_runtime::JSEngineSetup;

pub use crate::dom::bindings::codegen::DomTypeHolder::DomTypeHolder;
pub use crate::dom::bindings::codegen::DomTypes::DomTypes;
// These trait exports are public, because they are used in the DOM bindings.
// Since they are used in derive macros,
// it is useful that they are accessible at the root of the crate.
pub use crate::dom::bindings::inheritance::HasParent;
pub use crate::dom::bindings::reflector::{DomObject, MutDomObject, Reflector};
pub use crate::dom::bindings::trace::{CustomTraceable, JSTraceable};
