// SPDX-FileCopyrightText: 2024 Mika Tammi
//
// SPDX-License-Identifier: MIT

fn main() -> anyhow::Result<()> {
    use vulkano::{
        instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
        swapchain::Surface,
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
    let _instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create instance");

    let window = WindowBuilder::new()
        .with_title("rodvk")
        .build(&event_loop)
        .expect("failed to build a window");

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
