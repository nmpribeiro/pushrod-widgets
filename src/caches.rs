// Pushrod Widgets
// Cache
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
//
// TODO: This should probably be a draw tree, but it needs to store the top-down representation
// TODO: of the structure.  So, a tree is not entirely accurate.

use crate::widget::Widget;
use std::cell::RefCell;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use sdl2::video::Window;
use std::collections::HashMap;
use std::path::Path;

/// This is the `WidgetCache` store structure.
pub struct WidgetCache {
    cache: Vec<RefCell<Box<dyn Widget>>>,
}

/// This is the `WidgetCache` that is used to store `Widget` references in a drawing tree by ID.
impl WidgetCache {
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
        }
    }
}

/// This is a storage object for the `TextureCache`.
pub struct TextureCache {
    images: HashMap<String, Texture>,
    ttf_context: Sdl2TtfContext,
}

/// The `TextureCache` provides a mechanism for caching images, and returning the current
/// text rendering context.
impl TextureCache {
    /// Creates a new `TextureCache`, generally used by the Pushrod runtime.  While you are
    /// able to instantiate your own `TextureCache`, it is advised that the main run loop
    /// handle this for you, as you may experience unexpected results and/or excess memory usage.
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            ttf_context: sdl2::ttf::init().map_err(|e| e.to_string()).unwrap(),
        }
    }

    /// Returns the currently available text context as a reference.
    pub fn get_ttf_context(&self) -> &Sdl2TtfContext {
        &self.ttf_context
    }

    /// Returns an image loaded into a `Texture` reference, caching it in memory.
    pub fn get_image(&mut self, c: &mut Canvas<Window>, image_name: String) -> &Texture {
        self.images.entry(image_name.clone()).or_insert({
            c.texture_creator()
                .load_texture(Path::new(&image_name))
                .unwrap()
        })
    }
}
