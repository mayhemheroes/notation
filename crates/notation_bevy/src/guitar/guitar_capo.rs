use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, FillRectangle, LayoutSize, ShapeOp};

use crate::prelude::{NotationTheme};

#[derive(Clone, Debug)]
pub struct GuitarCapoData {
    pub capo: u8,
    pub guitar_size: LayoutSize,
}

impl GuitarCapoData {
    pub fn new(capo: u8) -> Self {
        Self {
            capo,
            guitar_size: LayoutSize::ZERO,
        }
    }
}

impl ShapeOp<NotationTheme, FillRectangle> for GuitarCapoData {
    fn get_shape(&self, theme: &NotationTheme) -> FillRectangle {
        let width = self.guitar_size.width * theme.guitar.capo_width_factor;
        let height = self.guitar_size.height * theme.guitar.capo_height_factor;
        let color = theme.colors.strings.capo;
        let offset = if self.capo == 0 || self.guitar_size.width <= 0.0 {
            BevyUtil::offscreen_offset()
        } else {
            let height = self.guitar_size.height * theme.guitar.capo_height_factor;
            let finger_radius = theme.guitar.string_x_factor * self.guitar_size.width / 2.0;
            let y = theme
                .guitar
                .calc_fret_y(self.capo, self.guitar_size.height)
                + height * 0.5
                - finger_radius;
            Vec3::new(0.0, y, theme.core.mini_bar_z + 2.0)
        };
        FillRectangle {
            width,
            height,
            origin: shapes::RectangleOrigin::Center,
            color,
            offset,
        }
    }
}
