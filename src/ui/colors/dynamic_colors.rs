use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::ui::colors::app_colors::{hsv_to_rgb, rgb, rgb_to_hsv, AppColors, ColorScheme};

const BRIGHTNESS_THRESHOLD: f32 = 0.5;
const TEXT_HIGHLIGHT_SATURATION_WEIGHT: f32 = 1.2;
const BACKGROUND_DESATURATION_WEIGHT: f32 = 0.7;
const BACKGROUND_ALT_BRIGHTNESS_ADJUST: f32 = 0.22;
const BACKGROUND_ALT_SATURATION_WEIGHT: f32 = 0.5;
const BACKGROUND_HIGHLIGHT_PRIMARY_WEIGHT: f32 = 0.77;
const BACKGROUND_HIGHLIGHT_SECONDARY_WEIGHT: f32 = 0.28;
const BORDER_BRIGHTNESS_ADJUST: f32 = 0.15;
const BORDER_SATURATION_WEIGHT: f32 = 0.6;
const ACCENT_SATURATION_WEIGHT: f32 = 1.2;

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, PartialEq, Serialize)]
pub enum DynamicColors {
    AshOrange,
    FleshyPink,
    VintageWine,
    CloudCover,
}

impl ColorScheme for DynamicColors {
    fn colors(&self) -> AppColors {
        match self {
            DynamicColors::AshOrange => {
                let pr = 100;
                let pg = 100;
                let pb = 100;
                let sr = 200;
                let sg = 150;
                let sb = 50;

                color_algorithm(pr, pg, pb, sr, sg, sb)
            }
            DynamicColors::FleshyPink => {
                let pr = 247;
                let pg = 10;
                let pb = 33;
                let sr = 20;
                let sg = 0;
                let sb = 220;

                color_algorithm(pr, pg, pb, sr, sg, sb)
            }
            DynamicColors::VintageWine => {
                let pr = 35;
                let pg = 0;
                let pb = 20;
                let sr = 200;
                let sg = 0;
                let sb = 50;

                color_algorithm(pr, pg, pb, sr, sg, sb)
            }
            DynamicColors::CloudCover => {
                let pr = 208;
                let pg = 218;
                let pb = 219;
                let sr = 40;
                let sg = 196;
                let sb = 252;

                color_algorithm(pr, pg, pb, sr, sg, sb)
            }
        }
    }
}

pub fn color_algorithm(pr: u8, pg: u8, pb: u8, sr: u8, sg: u8, sb: u8) -> AppColors {
    let (ph, ps, pv) = rgb_to_hsv(pr, pg, pb);
    let (sh, ss, sv) = rgb_to_hsv(sr, sg, sb);

    let is_bright = pv > BRIGHTNESS_THRESHOLD;

    let text = if is_bright {
        rgb(0, 0, 0)
    } else {
        rgb(255, 255, 255)
    };
    let text_alt = if is_bright {
        rgb(60, 60, 60)
    } else {
        rgb(190, 190, 190)
    };
    let text_highlight_rgb = hsv_to_rgb(sh, (ss * TEXT_HIGHLIGHT_SATURATION_WEIGHT).min(1.0), sv);
    let text_highlight = rgb(
        text_highlight_rgb.0,
        text_highlight_rgb.1,
        text_highlight_rgb.2,
    );

    let accent_rgb = hsv_to_rgb(sh, (ss * ACCENT_SATURATION_WEIGHT).min(1.0), sv);
    let accent = rgb(accent_rgb.0, accent_rgb.1, accent_rgb.2);

    let background_rgb = hsv_to_rgb(ph, ps * BACKGROUND_DESATURATION_WEIGHT, pv);
    let background = rgb(background_rgb.0, background_rgb.1, background_rgb.2);
    let background_alt_v = if is_bright {
        (pv - BACKGROUND_ALT_BRIGHTNESS_ADJUST).max(0.0)
    } else {
        (pv + BACKGROUND_ALT_BRIGHTNESS_ADJUST).min(1.0)
    };
    let background_alt_rgb =
        hsv_to_rgb(ph, ps * BACKGROUND_ALT_SATURATION_WEIGHT, background_alt_v);
    let background_alt = rgb(
        background_alt_rgb.0,
        background_alt_rgb.1,
        background_alt_rgb.2,
    );
    let background_highlight_rgb = (
        ((background_rgb.0 as f32 * BACKGROUND_HIGHLIGHT_PRIMARY_WEIGHT)
            + (accent_rgb.0 as f32 * BACKGROUND_HIGHLIGHT_SECONDARY_WEIGHT)) as u8,
        ((background_rgb.1 as f32 * BACKGROUND_HIGHLIGHT_PRIMARY_WEIGHT)
            + (accent_rgb.1 as f32 * BACKGROUND_HIGHLIGHT_SECONDARY_WEIGHT)) as u8,
        ((background_rgb.2 as f32 * BACKGROUND_HIGHLIGHT_PRIMARY_WEIGHT)
            + (accent_rgb.2 as f32 * BACKGROUND_HIGHLIGHT_SECONDARY_WEIGHT)) as u8,
    );
    let background_highlight = rgb(
        background_highlight_rgb.0,
        background_highlight_rgb.1,
        background_highlight_rgb.2,
    );

    let border_v = if is_bright {
        (pv - BORDER_BRIGHTNESS_ADJUST).max(0.0)
    } else {
        (pv + BORDER_BRIGHTNESS_ADJUST).min(1.0)
    };
    let border_rgb = hsv_to_rgb(ph, ps * BORDER_SATURATION_WEIGHT, border_v);
    let border = rgb(border_rgb.0, border_rgb.1, border_rgb.2);

    let warning_rgb = hsv_to_rgb(30.0, 0.9, if is_bright { 0.8 } else { 1.0 });
    let warning = rgb(warning_rgb.0, warning_rgb.1, warning_rgb.2);

    let error_rgb = hsv_to_rgb(0.0, 0.9, if is_bright { 0.8 } else { 1.0 });
    let error = rgb(error_rgb.0, error_rgb.1, error_rgb.2);

    AppColors {
        text,
        text_alt,
        text_highlight,
        background,
        background_alt,
        background_highlight,
        warning,
        error,
        border,
        accent,
    }
}
