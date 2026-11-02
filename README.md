# tuhka
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
[ash]: https://crates.io/crates/ash
