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

use std::collections::HashMap;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::texture_store::TextureStore;
use crate::widget::Widget;
use crate::caches::TextureCache;
use crate::properties::WidgetProperties;

/// Base Widget.
#[derive(Default)]
pub struct BaseWidget {
    texture_store: TextureStore,
    properties: WidgetProperties,
}

/// Implementation for drawing a `BaseWidget`, with the `Widget` trait objects applied.
impl Widget for BaseWidget {
    fn draw(&mut self, c: &mut Canvas<Window>, _t: &mut TextureCache) -> Option<&Texture> {
        // You _can_ remove this `if` statement here, and just let the code run each time.  It will
        // eventually make your application less efficient if this is constantly called.
        if self.invalidated() {
            // let bounds = self.get_config().get_size(CONFIG_SIZE);
            //
            // self.texture_store
            //     .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);
            //
            // let base_color = self.get_config().get_color(CONFIG_COLOR_BASE);
            // let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);
            //
            // c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
            //     texture.set_draw_color(base_color);
            //     texture.clear();
            //
            //     texture.set_draw_color(border_color);
            //     texture
            //         .draw_rect(Rect::new(0, 0, bounds[0], bounds[1]))
            //         .unwrap();
            // })
            //     .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    fn properties(&mut self) -> &mut WidgetProperties { &mut self.properties }
}
