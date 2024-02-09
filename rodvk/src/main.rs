// SPDX-FileCopyrightText: 2024 Mika Tammi
//
// SPDX-License-Identifier: MIT

use std::sync::Arc;
use vulkano::{
    device::Device,
    image::{view::ImageView, Image},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::Swapchain,
};

fn get_render_pass(device: Arc<Device>, swapchain: &Arc<Swapchain>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                // Set the format the same as the swapchain.
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
    .unwrap()
}

fn get_framebuffers(images: &[Arc<Image>], render_pass: &Arc<RenderPass>) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

fn main() -> anyhow::Result<()> {
    use vulkano::{
        device::{DeviceCreateInfo, DeviceExtensions, QueueCreateInfo},
        image::ImageUsage,
        instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
        swapchain::{Surface, SwapchainCreateInfo},
        VulkanLibrary,
    };
    use winit::{
        event::{ElementState, Event, KeyEvent, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        keyboard::{Key, NamedKey},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new().expect("failed to create event loop");

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let required_extensions = Surface::required_extensions(&event_loop);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create instance");

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("rodvk")
            .build(&event_loop)
            .expect("failed to build a window"),
    );

    let surface =
        Surface::from_window(instance.clone(), window.clone()).expect("could not create surface");

    // TODO: Fix this with proper way to select physical device
    // Select first physical device available
    let physical = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    let required_device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let queue_family_index = 0;
    let (device, mut queues) = Device::new(
        physical.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: required_device_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let _queue = queues.next().unwrap();

    let caps = physical
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");

    let dimensions = window.inner_size();
    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = physical
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;

    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1, // How many buffers to use in swapchain
            image_format,
            image_extent: dimensions.into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )
    .expect("cannot create swapchain");

    let render_pass = get_render_pass(device.clone(), &swapchain);
    let _framebuffers = get_framebuffers(&images, &render_pass);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed, exiting");
                elwt.exit();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                println!("The ESC button was pressed, exiting");
                elwt.exit();
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw
            }
            _ => (),
        }
    })?;

    Ok(())
}
