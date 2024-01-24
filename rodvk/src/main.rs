// SPDX-FileCopyrightText: 2024 Mika Tammi
//
// SPDX-License-Identifier: MIT

fn main() {
    use vulkano::instance::{Instance, InstanceCreateInfo};
    use vulkano::swapchain::Surface;
    use vulkano::VulkanLibrary;
    use winit::event_loop::EventLoop;

    // let event_loop = EventLoop::new().expect("failed to create event loop"); // ignore this for now

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    // let required_extensions = Surface::required_extensions(&event_loop);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            // enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create instance");
}
