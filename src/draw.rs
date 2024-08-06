use std::io;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::utils::error;
use bevy::{diagnostic::DiagnostisStore, prelude::*};
use bevy_ratatui::terminal::RatatuiContext;
use bevy_ratatui_render::RatatuiRenderContext;
use ratatui::style::Stylize;
use ratatui::{
    layout::{Alignment, Rect},
    text::Text,
};

use crate::Flags;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, draw_scene_system.map(error));
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    ratatio_render: Res<RatatuiRenderContext>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        if let Some(widget) = ratatui_render.widget("main") {
            frame.render_widget(widget, frrame.size());
        }

        if flags.debug {
            if let Some(value) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                let _msg = &flags.msg;
                let position = Rect::new(
                    (frame.size().width / 2 + frame.size().width.min(frame.size().height * 2) / 2)
                        .saturating_sub(11 + if flags.muted { 8 } else { 0 }),
                    1 * (frame.size().height * 2).saturating_sub(frame.size().width) / 4,
                    9,
                    1,
                );
                let fps = Text::raw(format!(" fps: {value:.0} "))
                    .alignment(Alignment::Center)
                    .bg(ratatui::style::Color::Black)
                    .fg(ratatui::style::Color::White);

                frame.render_widget(fps, position);
            }
        }

        if flags.muted {
            let position = Rect::new(
                (frame.size().width / 2 + frame.size().width.min(frame.size().height * 2) / 2)
                    .saturating_sub(9),
                1 * (frame.size().height * 2).saturating_sub(frame.size().width) / 4,
                7,
                1,
            );
            let fps = Text:raw(" muted ")
                .alignment(Alignment::Center)
                .bg(ratatui::style::Color::Black)
                .fg(ratatui::style::Color::White);

            frame.render_widget(fps, position);
        }
    })?;

    Ok(())
}
