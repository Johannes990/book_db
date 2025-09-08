use ratatui::style::Color;

pub struct AppColors {
    pub text: Color,
    pub text_alt: Color,
    pub text_highlight: Color,
    pub background: Color,
    pub background_alt: Color,
    pub background_highlight: Color,
    pub warning: Color,
    pub error: Color,
    pub border: Color,
    pub accent: Color,
}

pub trait ColorScheme {
    fn colors(&self) -> AppColors;
}

// this is for YivColor, so it can recognize rgb(x, y, z) pattern
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == rf {
        60.0 * (((gf - bf) / delta) % 6.0)
    } else if max == gf {
        60.0 * ((bf - rf) / delta + 2.0)
    } else {
        60.0 * ((rf - gf) / delta + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;

    let (rf, gf, bf) = match (h as i32 % 360) / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((rf + m) * 255.0).round() as u8,
        ((gf + m) * 255.0).round() as u8,
        ((bf + m) * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::{hsv_to_rgb, rgb_to_hsv};

    fn approx_eq<T>(a: T, b: T, eps: T) -> bool
    where
        T: Into<f64> + Copy,
    {
        let (a, b, eps) = (a.into(), b.into(), eps.into());
        (a - b).abs() < eps
    }

    #[test]
    fn test_rgb_to_hsv_primary_colors() {
        // red
        let (h, s, v) = rgb_to_hsv(255, 0, 0);
        assert!(approx_eq(h, 0.0, 0.1));
        assert!(approx_eq(s, 1.0, 1e-3));
        assert!(approx_eq(v, 1.0, 1e-3));

        // green
        let (h, s, v) = rgb_to_hsv(0, 255, 0);
        assert!(approx_eq(h, 120.0, 0.1));
        assert!(approx_eq(s, 1.0, 1e-3));
        assert!(approx_eq(v, 1.0, 1e-3));

        // blue
        let (h, s, v) = rgb_to_hsv(0, 0, 255);
        assert!(approx_eq(h, 240.0, 0.1));
        assert!(approx_eq(s, 1.0, 1e-3));
        assert!(approx_eq(v, 1.0, 1e-3));
    }

    #[test]
    fn test_rgb_to_hsv_secondary_colors() {
        // cyan
        let (h, s, v) = rgb_to_hsv(0, 255, 255);
        assert!(approx_eq(h, 180.0, 1e-3));
        assert!(approx_eq(s, 1.0, 1e-6));
        assert!(approx_eq(v, 1.0, 1e-6));

        // yellow
        let (h, s, v) = rgb_to_hsv(255, 255, 0);
        assert!(approx_eq(h, 60.0, 1e-3));
        assert!(approx_eq(s, 1.0, 1e-6));
        assert!(approx_eq(v, 1.0, 1e-6));

        // magenta
        let (h, s, v) = rgb_to_hsv(255, 0, 255);
        assert!(approx_eq(h, 300.0, 1e-3));
        assert!(approx_eq(s, 1.0, 1e-6));
        assert!(approx_eq(v, 1.0, 1e-6));
    }

    #[test]
    fn test_hsv_to_rgb_primary_colors() {
        // red
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert!(approx_eq(r, 255, 1));
        assert!(approx_eq(g, 0, 1));
        assert!(approx_eq(b, 0, 1));

        // green
        let (r, g, b) = hsv_to_rgb(120.0, 1.0, 1.0);
        assert!(approx_eq(r, 0, 1));
        assert!(approx_eq(g, 255, 1));
        assert!(approx_eq(b, 0, 1));

        // blue
        let (r, g, b) = hsv_to_rgb(240.0, 1.0, 1.0);
        assert!(approx_eq(r, 0, 1));
        assert!(approx_eq(g, 0, 1));
        assert!(approx_eq(b, 255, 1));
    }

    #[test]
    fn test_hsv_to_rgb_secondary_colors() {
        // cyan
        let (r, g, b) = hsv_to_rgb(180.0, 1.0, 1.0);
        assert!(approx_eq(r, 0, 1));
        assert!(approx_eq(g, 255, 1));
        assert!(approx_eq(b, 255, 1));

        // yellow
        let (r, g, b) = hsv_to_rgb(60.0, 1.0, 1.0);
        assert!(approx_eq(r, 255, 1));
        assert!(approx_eq(g, 255, 1));
        assert!(approx_eq(b, 0, 1));

        // magenta
        let (r, g, b) = hsv_to_rgb(300.0, 1.0, 1.0);
        assert!(approx_eq(r, 255, 1));
        assert!(approx_eq(g, 0, 1));
        assert!(approx_eq(b, 255, 1));
    }

    #[test]
    fn test_hsv_to_rgb_roundtrip() {
        let rgb_vals = [(25, 134, 90), (255, 255, 0), (120, 120, 200), (50, 75, 100)];

        for val in rgb_vals {
            let (h, s, v) = rgb_to_hsv(val.0, val.1, val.2);
            let (r2, g2, b2) = hsv_to_rgb(h, s, v);

            assert!(approx_eq(val.0, r2, 1));
            assert!(approx_eq(val.1, g2, 1));
            assert!(approx_eq(val.2, b2, 1));
        }
    }

    #[test]
    fn test_rgb_to_hsv_roundtrip() {
        let hsv_vals = [
            (59.0, 0.75, 0.45),
            (330.0, 0.42, 0.99),
            (101.0, 1.0, 0.3),
            (0.2, 1.0, 0.5),
        ];

        for val in hsv_vals {
            dbg!(val.0, val.1, val.2);
            let (r, g, b) = hsv_to_rgb(val.0, val.1, val.2);
            let (h2, s2, v2) = rgb_to_hsv(r, g, b);
            dbg!(h2, s2, v2);

            assert!(approx_eq(val.0, h2, 0.5));
            assert!(approx_eq(val.1, s2, 0.01));
            assert!(approx_eq(val.2, v2, 0.01));
        }
    }

    #[test]
    fn test_gray_scale() {
        // black
        let (h, s, v) = rgb_to_hsv(0, 0, 0);
        assert!(approx_eq(h, 0.0, 1e-6));
        assert!(approx_eq(s, 0.0, 1e-6));
        assert!(approx_eq(v, 0.0, 1e-6));

        // gray
        let (h, s, v) = rgb_to_hsv(128, 128, 128);
        assert!(approx_eq(h, 0.0, 1e-6));
        assert!(approx_eq(s, 0.0, 1e-6));
        assert!(approx_eq(v, 0.5, 0.002));

        // white
        let (h, s, v) = rgb_to_hsv(255, 255, 255);
        assert!(approx_eq(h, 0.0, 1e-6));
        assert!(approx_eq(s, 0.0, 1e-6));
        assert!(approx_eq(v, 1.0, 1e-6));
    }
}
