# [tuhka]
New up-to-date Vulkan bindings for Rust.

## Usage

``` rust
use tuhka::{vk, Library, Instance};
let library = unsafe {
    Library::load()
}.unwrap();
let app_name = c"Test";
let engine_name = c"Engine";
let application_info = vk::ApplicationInfo
    ::default()
    .application_name(app_name)
    .application_version(vk::make_api_version(0, 0, 1, 0))
    .engine_name(engine_name)
    .engine_version(vk::make_api_version(0, 0, 1, 0))
    .api_version(vk::API_VERSION_1_4);
let create_info = vk::InstanceCreateInfo
    ::default()
    .application_info(Some(&application_info));
let instance: Instance = unsafe {
    library.create_instance(&create_info, None)
}.unwrap();
unsafe {
    library.destroy_instance(&instance, None);
}
```

## Features
### `window` (disabled by default)
Provides the `window` module, which can be used to create surfaces for a
[`raw_window_handle`](https://crates.io/crates/raw-window-handle).
### `bytemuck` (disabled by default)
Implements [`bytemuck`](https://crates.io/crates/bytemuck) `Zeroable`, `AnyBitPattern` and `NoUninit` 
for applicable Vulkan structures, and `AnyBitPattern` and `NoUninit` for non-dispatchable handles
and enums/bitflags.

[tuhka]: https://crates.io/crates/tuhka
