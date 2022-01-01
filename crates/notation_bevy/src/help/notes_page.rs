use bevy::prelude::*;
use bevy_egui::egui::{self, Ui};
use notation_bevy_utils::asset::markdown_asset::MarkDownAsset;
use notation_bevy_utils::easy_mark::{label_from_style, EasyMarkStyle};
use notation_model::prelude::TrackKind;

use crate::prelude::{NotationState, NotationAssets, NotationTheme};

use super::help_panel::{HelpPage, HelpPageId};
use super::page_helper::PageHelper;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct NotesPage {}

impl HelpPage for NotesPage {
    fn page_id(&self) -> HelpPageId {
        HelpPageId::Notes
    }
    fn tab_label(&self) -> &'static str {
        "Notes"
    }
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        _texts: &Assets<MarkDownAsset>,
        _assets: &NotationAssets,
        state: &NotationState,
        theme: &NotationTheme,
    ) {
        let strong_style = EasyMarkStyle {
            strong: true,
            ..EasyMarkStyle::default()
        };
        let scale = state
            .tab
            .as_ref()
            .map(|x| x.meta.scale.clone())
            .unwrap_or_default();
        let key = state
            .tab
            .as_ref()
            .map(|x| x.meta.key.clone())
            .unwrap_or_default();
        PageHelper::add_key_scale(ui, &key, &scale);
        ui.separator();
        egui::Grid::new("notes").show(ui, |ui| {
            let syllables = scale.get_syllables();
            for syllable in syllables.iter() {
                PageHelper::add_syllable_color(ui, theme, syllable);
            }
            ui.end_row();
            for (index, syllable) in syllables.iter().enumerate() {
                PageHelper::add_syllable(ui, theme, false, syllable, false, index == 0);
            }
            ui.end_row();
            for (index, syllable) in syllables.iter().enumerate() {
                PageHelper::add_syllable(ui, theme, false, syllable, true, index == 0);
            }
            ui.end_row();
            for (index, syllable) in syllables.iter().enumerate() {
                PageHelper::add_syllable_pitch(ui, theme, &scale, &key, syllable, index == 0);
            }
            ui.end_row();
            if let Some(fretboard) = state.tab.as_ref().and_then(|tab| {
                tab.get_track_of_kind(TrackKind::Guitar)
                    .and_then(|x| x.get_fretboard6())
            }) {
                if fretboard.capo > 0 {
                    ui.separator();
                    ui.add(label_from_style("guitar", &strong_style));
                    ui.add(label_from_style("capo", &strong_style));
                    ui.add(label_from_style("at", &strong_style));
                    ui.add(label_from_style(
                        fretboard.capo.to_string().as_str(),
                        &strong_style,
                    ));
                    let frets = if fretboard.capo == 1 { "fret" } else { "frets" };
                    ui.add(label_from_style(frets, &strong_style));
                    ui.separator();
                    ui.end_row();
                    for (index, syllable) in syllables.iter().enumerate() {
                        PageHelper::add_syllable_pitch_with_capo(
                            ui,
                            theme,
                            fretboard.capo,
                            &scale,
                            &key,
                            syllable,
                            index == 0,
                        );
                    }
                    ui.end_row();
                }
            }
        });
    }
}
