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

struct WidgetCacheContainer {
    widget: RefCell<Box<dyn Widget>>,
    name: String,
    parent: u32,
    children: Vec<u32>,
}

impl WidgetCacheContainer {
    pub fn new(widget: Box<dyn Widget>, name: String, parent: u32) -> Self {
        Self {
            widget: RefCell::new(widget),
            name,
            parent,
            children: Vec::new(),
        }
    }
}

/// This is the `WidgetCache` store structure.
#[derive(Default)]
pub struct WidgetCache {
    cache: Vec<WidgetCacheContainer>,
    texture_cache: TextureCache,
}

/// This is the `WidgetCache` that is used to store `Widget` references in a drawing tree by ID.
impl WidgetCache {
    /// Retrieves the ID of the widget at the X/Y coordinates given.
    ///
    /// Follows the following rules:
    /// - If the object is hidden, any objects underneath that object are short-circuited
    /// - If an object is visible, it walks the object's children to see if they are within the same
    ///   given coordinates
    ///
    /// The found ID is then returned having met all of those criteria.  If no ID was found, a
    /// 0 value (root level widget) is returned.
    #[inline]
    pub fn id_at_point(&self, x: u32, y: u32) -> u32 {
        let mut found_id: u32 = 0;

        found_id
    }

    /// Retrieves the `Widget` stored by its `RefCell<Box>` reference.
    #[inline]
    pub fn get(&self, widget_id: u32) -> &RefCell<Box<dyn Widget>> {
        &self.cache[widget_id as usize].widget
    }

    /// Retrieves the ID of a `Widget` by its `name`.  If the `name` could not be located, the top
    /// level ID `0` is returned.
    pub fn get_by_name(&self, name: String) -> u32 {
        let cache_size = self.size();

        for i in 0..cache_size {
            if self.cache[i as usize].name == name {
                return i
            }
        }

        0
    }

    /// Retrieves the parent ID of the widget ID specified.  If the widget is a top level widget (meaning
    /// there are no additional parents), a 0 will be returned.
    #[inline]
    pub fn get_parent_of(&self, widget_id: u32) -> u32 {
        self.cache[widget_id as usize].parent
    }

    /// Retrieves a list of children for the specified widget ID.  The child listing will always
    /// return a `Vec` - if any widgets have been added to this `Widget` as a parent, those IDs
    /// will be returned here.  If this widget has no children, an empty `Vec` will be returned.
    #[inline]
    pub fn get_children_of(&self, widget_id: u32) -> Vec<u32> {
        self.cache[widget_id as usize].children.clone()
    }

    /// Adds a new `Widget` to the cache, with the given mutable `Widget`, a name for the `Widget`,
    /// and the `Widget`'s parent ID.
    #[inline]
    pub fn add_widget(&mut self, mut widget: Box<dyn Widget>, widget_name: String, parent_id: u32) -> u32 {
        // use get_by_name to make sure the widget doesn't already exist by name.  If it does,
        // throw an error.

        // Invalidate the Widget just in case.
        &widget.invalidate();

        self.cache.push(WidgetCacheContainer::new(
            widget,
            widget_name,
            parent_id,
        ));

        let widget_id = self.size() - 1;

        self.cache[parent_id as usize].children.push(widget_id);

        widget_id
    }

    /// Retrieves the total number of `Widget`s in the cache.
    #[inline]
    pub fn size(&self) -> u32 {
        self.cache.capacity() as u32
    }

    /// Determines whether any of the `Widget`s in the cache have indicated that they need to be
    /// redrawn.
    pub fn invalidated(&self) -> bool {
        let mut invalidated: bool = false;

        for x in &self.cache {
            if x.widget.borrow_mut().invalidated() {
                invalidated = true;
                break;
            }
        }

        invalidated
    }
}

/// This is a storage object for the `TextureCache`.
pub struct TextureCache {
    images: HashMap<String, Texture>,
    ttf_context: Sdl2TtfContext,
}

/// Default implementation for TextureCache.
impl Default for TextureCache {
    fn default() -> Self {
        Self {
            images: HashMap::new(),
            ttf_context: sdl2::ttf::init().map_err(|e| e.to_string()).unwrap(),
        }
    }
}

/// The `TextureCache` provides a mechanism for caching images, and returning the current
/// text rendering context.
impl TextureCache {
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
