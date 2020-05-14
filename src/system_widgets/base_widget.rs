// Pushrod Widgets
// Base Widget
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::collections::HashMap;

use crate::caches::TextureCache;
use crate::properties::{
    WidgetProperties, PROPERTY_BACKGROUND_COLOR, PROPERTY_MAIN_COLOR, PROPERTY_SIZE,
};
use crate::texture_store::TextureStore;
use crate::widget::Widget;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

/// Base Widget.
#[derive(Default)]
pub struct BaseWidget {
    texture_store: TextureStore,
    properties: WidgetProperties,
}

/// Implementation for drawing a `BaseWidget`, with the `Widget` trait objects applied.
impl Widget for BaseWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, _t: &mut TextureCache) -> Option<&Texture> {
        // ONLY update the texture if the `BaseWidget` shows that it's been invalidated.
        if self.invalidated() {
            let base_color = self
                .properties
                .get_color(PROPERTY_MAIN_COLOR, Color::RGB(255, 255, 255));
            let border_color = self
                .properties
                .get_color(PROPERTY_BACKGROUND_COLOR, Color::RGB(0, 0, 0));
            let bounds = self.properties.get_bounds();

            self.texture_store
                .create_or_resize_texture(c, bounds.0, bounds.1);

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                texture.set_draw_color(border_color);
                texture
                    .draw_rect(Rect::new(0, 0, bounds.0, bounds.1))
                    .unwrap();
            })
            .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    fn properties(&mut self) -> &mut WidgetProperties {
        &mut self.properties
    }
}
