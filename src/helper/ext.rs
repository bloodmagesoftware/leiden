/*
 * Leiden - A video game
 * Copyright (C) 2024  Frank Mayer
 *
 * This file is part of Leiden.
 *
 * Fair Core License, Version 1.0, Apache 2.0 Future License
 * https://github.com/bloodmagesoftware/leiden/blob/main/LICENSE.md
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
