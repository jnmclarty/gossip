use super::GossipUi;
use crate::comms::ToOverlordMessage;
use crate::GLOBALS;
use eframe::egui;
use egui::widgets::{Button, Slider};
use egui::{Align, Context, Layout, ScrollArea, Ui, Vec2};

pub(super) fn update(app: &mut GossipUi, ctx: &Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
    ui.heading("Settings");

    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        ui.separator();
        ui.add_space(12.0);
        if ui.button("SAVE CHANGES").clicked() {
            // Copy local settings to global settings
            *GLOBALS.settings.blocking_write() = app.settings.clone();

            // Tell the overlord to save them
            let _ = GLOBALS.to_overlord.send(ToOverlordMessage::SaveSettings);
        }

        ui.with_layout(Layout::top_down(Align::Min), |ui| {
            ui.add_space(10.0);
            ui.separator();

            ScrollArea::vertical()
                .id_source("settings")
                .override_scroll_delta(Vec2 { x: 0.0, y: app.current_scroll_offset })
                .show(ui, |ui| {

                ui.add_space(12.0);

                ui.heading("How Many Relays to Query");

                ui.horizontal(|ui| {
                    ui.label("Number of relays to query per person: ").on_hover_text("We will query N relays per person. Many people share the same relays so those will be queried about multiple people. Takes affect on restart. I recommend 2. Too many and gossip will (currently) keep connecting to new relays trying to find the unfindable, loading many events from each. Takes effect on restart.");
                    ui.add(Slider::new(&mut app.settings.num_relays_per_person, 1..=4).text("relays"));
                });

                ui.horizontal(|ui| {
                    ui.label("Maximum following feed relays: ")
                        .on_hover_text(
                            "We will not stay connected to more than this many relays for following feed. Takes affect on restart. During these early days of nostr, I recommend capping this at around 20 to 30.",
                        );
                    ui.add(Slider::new(&mut app.settings.max_relays, 5..=100).text("relays"));
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.heading("How Many Posts to Load");

                ui.horizontal(|ui| {
                    ui.label("Feed Chunk: ").on_hover_text("This is the amount of time backwards from now that we will load events from. You'll eventually be able to load more, one chunk at a time. Mostly takes effect on restart.");
                    ui.add(Slider::new(&mut app.settings.feed_chunk, 600..=86400).text("seconds, "));
                    ui.label(secs_to_string(app.settings.feed_chunk));
                });

                ui.horizontal(|ui| {
                    ui.label("Replies Chunk: ").on_hover_text("This is the amount of time backwards from now that we will load replies, mentions, and DMs from. You'll eventually be able to load more, one chunk at a time. Mostly takes effect on restart.");
                    ui.add(Slider::new(&mut app.settings.replies_chunk, 86400..=2592000).text("seconds, "));
                    ui.label(secs_to_string(app.settings.replies_chunk));
                });

                ui.horizontal(|ui| {
                    ui.label("Overlap: ").on_hover_text("If we recently loaded events up to time T, but restarted, we will now load events starting from time T minus overlap. Takes effect on restart. I recommend 300 (5 minutes).");
                    ui.add(Slider::new(&mut app.settings.overlap, 0..=3600).text("seconds, "));
                    ui.label(secs_to_string(app.settings.overlap));
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.heading("Feed");

                ui.horizontal(|ui| {
                    ui.label("Recompute feed every (milliseconds): ")
                        .on_hover_text(
                            "The UI redraws frequently. We recompute the feed less frequently to conserve CPU. Takes effect when the feed next recomputes. I recommend 3500.",
                        );
                    ui.add(Slider::new(&mut app.settings.feed_recompute_interval_ms, 250..=5000).text("milliseconds"));
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.heading("What Posts to Include");

                ui.checkbox(
                    &mut app.settings.reactions,
                    "Enable reactions (show and react)",
                );

                ui.checkbox(
                    &mut app.settings.reposts,
                    "Enable reposts (show)",
                );

                ui.add_space(12.0);

                ui.checkbox(
                    &mut app.settings.direct_replies_only,
                    "Show Direct Replies Only",
                )
                    .on_hover_text("Takes effect when the feed refreshes.");

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.heading("Posting");

                ui.horizontal(|ui| {
                    ui.label("Proof of Work: ")
                        .on_hover_text("The larger the number, the longer it takes.");
                    ui.add(Slider::new(&mut app.settings.pow, 0..=40).text("leading zeroes"));
                });

                ui.add_space(12.0);

                ui.checkbox(
                    &mut app.settings.set_client_tag,
                    "Add tag [\"client\",\"gossip\"] to posts",
                )
                    .on_hover_text("Takes effect immediately.");

                ui.add_space(12.0);

                ui.separator();
                ui.add_space(12.0);

                ui.heading("Network");

                ui.checkbox(&mut app.settings.offline, "Offline Mode")
                    .on_hover_text("If selected, no network requests will be issued. Takes effect on restart.");

                ui.checkbox(&mut app.settings.load_avatars, "Fetch Avatars")
                    .on_hover_text("If disabled, avatars will not be fetched, but cached avatars will still display. Takes effect on save.");

                ui.checkbox(&mut app.settings.check_nip05, "Check NIP-05")
                    .on_hover_text("If disabled, NIP-05 fetches will not be performed, but existing knowledge will be preserved, and following someone by NIP-05 will override this and do the fetch. Takes effect on save.");

                ui.add_space(12.0);

                ui.checkbox(
                    &mut app.settings.set_user_agent,
                    &format!("Send User-Agent Header to Relays: gossip/{}", app.about.version),
                )
                    .on_hover_text("Takes effect on next relay connection.");

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ui.heading("User Interface");

                ui.horizontal(|ui| {
                    ui.label("Switch to");
                    #[allow(clippy::collapsible_else_if)]
                    if app.settings.light_mode {
                        if ui
                            .add(Button::new("🌙 Dark"))
                            .on_hover_text("Switch to dark mode")
                            .clicked()
                        {
                            ui.ctx().set_visuals(super::style::dark_mode_visuals());
                            app.settings.light_mode = false;
                        }
                    } else {
                        if ui
                            .add(Button::new("☀ Light"))
                            .on_hover_text("Switch to light mode")
                            .clicked()
                        {
                            ui.ctx().set_visuals(super::style::light_mode_visuals());
                            app.settings.light_mode = true;
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Override DPI: ")
                        .on_hover_text(
                            "On some systems, DPI is not reported properly. In other cases, people like to zoom in or out. This lets you.",
                        );
                    ui.checkbox(
                        &mut app.override_dpi,
                        "Override to ");
                    ui.add(Slider::new(&mut app.override_dpi_value, 72..=250).text("DPI"));
                    if ui.button("Try it now").clicked() {
                        let ppt: f32 = app.override_dpi_value as f32 / 72.0;
                        ctx.set_pixels_per_point(ppt);
                    }

                    // transfer to app.settings
                    app.settings.override_dpi = if app.override_dpi {
                        // Set it in settings to be saved on button press
                        Some(app.override_dpi_value)
                    } else {
                        None
                    };
                });


                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label("Maximum FPS: ")
                        .on_hover_text(
                            "The UI redraws every frame. By limiting the maximum FPS you can reduce load on your CPU. Takes effect immediately. I recommend 10, maybe even less.",
                        );
                    ui.add(Slider::new(&mut app.settings.max_fps, 2..=60).text("Frames per second"));
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                if ui.button("Prune Database")
                    .on_hover_text("This will delete overridden events, events older than a week, and related data while keeping everything important. It can take MANY MINUTES to complete, and when complete there will be a status message indicating so. Also, because the database will be very busy, best not to use gossip while pruning, just wait.")
                    .clicked() {
                        *GLOBALS.status_message.blocking_write() = "Pruning database, please wait (this takes a long time)...".to_owned();

                        let _ = GLOBALS.to_overlord.send(ToOverlordMessage::PruneDatabase);
                    }

                ui.add_space(12.0);
            });
        });
    });
}

fn secs_to_string(secs: u64) -> String {
    let days = secs / 86400;
    let remainder = secs % 86400;
    let hours = remainder / 3600;
    let remainder = remainder % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;
    let mut output: String = String::new();
    if days > 0 {
        output.push_str(&format!(" {} days", days));
    }
    if hours > 0 {
        output.push_str(&format!(" {} hours", hours));
    }
    if minutes > 0 {
        output.push_str(&format!(" {} minutes", minutes));
    }
    output.push_str(&format!(" {} seconds", seconds));
    output
}
