mod components;
mod resources;
mod systems;
mod utils;

use crate::core::graphics::rendering::systems::{
    rc_clear_entities, rfq_finish_queue, rp_create_command_encoder, rpq_begin_render_passes,
    rr_render,
};
use crate::core::graphics::RenderSet::{CleanUp, FinishQueue, PreQueue, Prepare, Queue};
use crate::core::graphics::{Render, RenderSet};
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::IntoSystemConfigs;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Render,
            (
                rp_create_command_encoder.in_set(Prepare),
                rp_create_command_encoder.in_set(PreQueue),
                rpq_begin_render_passes.in_set(Queue),
                rfq_finish_queue.in_set(FinishQueue),
                rr_render.in_set(RenderSet::Render),
                rc_clear_entities.in_set(CleanUp),
            ),
        );
    }
}
