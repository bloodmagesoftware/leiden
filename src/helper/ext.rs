/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Leiden is licensed under the terms of the custom license available at:
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE
 *
 * Unauthorized copying, modification, distribution, or use of this software, via any medium, is strictly prohibited.
 */

use bevy_ui::Val;

pub trait SetVal {
    fn set_val(&mut self, val: f32) -> &mut Self;
}

impl SetVal for Val {
    fn set_val(&mut self, val: f32) -> &mut Self {
        match self {
            Val::Auto => {}
            Val::Px(px) => *px = val,
            Val::Percent(percent) => *percent = val,
            Val::Vw(vw) => *vw = val,
            Val::Vh(vh) => *vh = val,
            Val::VMin(vmin) => *vmin = val,
            Val::VMax(vmax) => *vmax = val,
        }
        self
    }
}
