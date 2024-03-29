//! Bevy systems for the graphics module.

use crate::core::graphics::resources::GraphicsState;
use crate::core::window::components::{RawHandleWrapper, Window};
use crate::core::window::events::{CloseRequestedEvent, WindowCreatedEvent};
use crate::core::window::resources::WinitWindows;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{NonSend, NonSendMut, Query};
use log::info;

/// Creates a surface for each window created.
///
/// Runs on `Update` when a [`WindowCreatedEvent`] is received,
/// use the [`GraphicsState`] to create a surface for the window, passing in the window and raw handle.
pub fn u_create_surface(
    mut window_created_event: EventReader<WindowCreatedEvent>,
    winit_windows: NonSend<WinitWindows>,
    mut graphics_state: NonSendMut<GraphicsState>,
    query: Query<(&Window, &RawHandleWrapper)>,
) {
    for event in window_created_event.read() {
        let window = winit_windows
            .windows
            .get(&event.window_id)
            .unwrap_or_else(|| panic!("Window {:?} not found!", event.window_id));
        let window_entity = winit_windows.window_to_entity[&event.window_id];
        let (window_component, raw_window_handle) = query
            .get(window_entity)
            .unwrap_or_else(|_| panic!("No Window component found on entity {:?}!", window_entity));
        graphics_state.create_surface(window, window_component, raw_window_handle);
        info!("Surface created for window on {:?}", window_entity);
    }
}

/// Destroys a surface for each window that has a close requested event.
///
/// Runs on `Update` when a [`CloseRequestedEvent`] is received,
/// use the [`GraphicsState`] to destroy the surface for the window.
pub fn u_destroy_surface(
    mut close_requested_event: EventReader<CloseRequestedEvent>,
    mut graphics_state: NonSendMut<GraphicsState>,
) {
    for event in close_requested_event.read() {
        graphics_state.destroy_surface(event.window_id);
    }
}
