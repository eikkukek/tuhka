//! New up-to-date Vulkan bindings.
//!
//! # Usage
//! ``` no_run
//! use tuhka::{vk, Library, Instance};
//! let library = unsafe {
//!     Library::load()
//! }.unwrap();
//! let app_name = c"Test";
//! let engine_name = c"Engine";
//! let application_info = vk::ApplicationInfo
//!    ::default()
//!    .application_name(app_name)
//!    .application_version(vk::make_api_version(0, 0, 1, 0))
//!    .engine_name(engine_name)
//!    .engine_version(vk::make_api_version(0, 0, 1, 0))
//!    .api_version(vk::API_VERSION_1_4);
//! let create_info = vk::InstanceCreateInfo
//!    ::default()
//!    .application_info(Some(&application_info));
//! let instance: Instance = unsafe {
//!     library.create_instance(&create_info, None)
//! }.unwrap();
//! unsafe {
//!     library.destroy_instance(&instance, None);
//! }
//! ```

#![no_std]

pub mod vk;
mod option;
mod library;
mod instance;
mod device;
mod result;
mod core_gen;
mod extension_gen;
#[cfg(feature = "window")]
pub mod window;

pub(crate) use option::AsPtr;
pub use result::*;
pub use core_gen::*;

pub use library::*;
pub use instance::*;
pub use device::*;
pub use extension_gen::*;
mod macros;

/*
 * don't test, because can't mark as no_run
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadMeDoctests;
*/
