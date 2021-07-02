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

use crate::properties::{PROPERTY_HIDDEN, PROPERTY_INVALIDATED};
use crate::system_widgets::base_widget::BaseWidget;
use crate::widget::Widget;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureQuery};
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use sdl2::video::Window;
use std::cell::RefCell;
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
pub struct WidgetCache {
    cache: Vec<WidgetCacheContainer>,
    texture_cache: TextureCache,
}

/// This is the `WidgetCache` that is used to store `Widget` references in a drawing tree by ID.
impl WidgetCache {
    /// Creates a new `WidgetCache`, adding the `BaseWidget` to the top level of the Window, with the name
    /// `root` as the root `Widget`.  This `Widget` can be modified like any other - its properties
    /// can be changed, background color, border color, etc. can all be manipulated just like any
    /// other `Widget`.  Its ID is `0`.
    pub fn new(w: u32, h: u32) -> Self {
        let mut base_widget = BaseWidget::default();

        &base_widget.properties().set_bounds(w, h);

        Self {
            cache: vec![WidgetCacheContainer::new(
                Box::new(base_widget),
                String::from("root"),
                0,
            )],
            texture_cache: TextureCache::default(),
        }
    }

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
        let mut hidden_widgets: Vec<u32> = vec![];
        let children_of_widget = self.get_children_of(0);

        for id in &children_of_widget {
            if self.cache.len() > *id as usize {
                let is_hidden = self.cache[*id as usize]
                    .widget
                    .borrow_mut()
                    .properties()
                    .get_bool(PROPERTY_HIDDEN);
                let widget_xy = self.cache[*id as usize]
                    .widget
                    .borrow_mut()
                    .properties()
                    .get_origin();
                let widget_wh = self.cache[*id as usize]
                    .widget
                    .borrow_mut()
                    .properties()
                    .get_bounds();

                if !is_hidden {
                    if x >= widget_xy.0
                        && x <= widget_xy.0 + widget_wh.0
                        && y >= widget_xy.1
                        && y <= widget_xy.1 + widget_wh.1
                    {
                        if !hidden_widgets.contains(id) {
                            found_id = *id;
                        }
                    }
                } else {
                    hidden_widgets.push(*id);

                    for hidden_widget_id in self.get_children_of(*id) {
                        hidden_widgets.push(hidden_widget_id);
                    }
                }
            }
        }

        found_id
    }

    /// Retrieves the `Widget` stored by its `RefCell<Box>` reference.
    #[inline]
    pub fn get(&self, widget_id: u32) -> &RefCell<Box<dyn Widget>> {
        &self.cache[widget_id as usize].widget
    }

    /// Retrieves the ID of a `Widget` by its `name`.  If the `name` could not be located, the top
    /// level ID `0` is returned.
    #[inline]
    pub fn get_by_name(&self, name: String) -> u32 {
        let cache_size = self.size();

        for i in 0..cache_size {
            if self.cache[i as usize].name == name {
                return i;
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
        // TODO: check this
        if self.cache.len() > widget_id as usize {
            self.cache[widget_id as usize].children.clone()
        } else {
            self.cache[self.cache.len() - 1].children.clone()
        }
    }

    /// Adds a new `Widget` to the cache, with the given mutable `Widget`, a name for the `Widget`,
    /// and the `Widget`'s parent ID.
    #[inline]
    pub fn add(&mut self, mut widget: Box<dyn Widget>, widget_name: String, parent_id: u32) -> u32 {
        // use get_by_name to make sure the widget doesn't already exist by name.  If it does,
        // throw an error.

        // Invalidate the Widget just in case.
        widget.invalidate();

        self.cache
            .push(WidgetCacheContainer::new(widget, widget_name, parent_id));

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

    /// Recursive drawing function that takes a `Widget`'s ID, and gets a list of the children that
    /// are owned for that `Widget`.  It then walks the tree of all of the children, and draws the
    /// contents into a `Texture`.  The `TextureCache` is sent such that the `Widget` has the ability
    /// to load in an image, or render a font.  This cache should be used sparingly.
    ///
    /// If a draw method is called on a `Widget` but the `Widget` has not been invalidated (meaning
    /// it does not to be redrawn), the cached `Texture` for the `Widget`'s draw surface is returned,
    /// which is a reference to an `SDL2 Texture`.  This way, the image from GPU memory is simply
    /// copied back to screen in a very quick operation.
    ///
    /// Any `Widget`s that have a property of `PROPERTY_HIDDEN` set will short circuit the draw
    /// for that `Widget` and its children.
    ///
    /// Drawing is computed off-screen in GPU memory, so this is also a very fast operation, which
    /// should theoretically take place in less than a single draw frame.
    pub fn draw(&mut self, widget_id: u32, c: &mut Canvas<Window>) {
        let children_of_widget = self.get_children_of(widget_id);

        if children_of_widget.is_empty() {
            return;
        }

        for id in &children_of_widget {
            // TODO: check this
            let new_id: usize = *id as usize - 1;
            if self.cache.len() > new_id as usize {
                let paint_widget = &mut self.cache[new_id];
                let is_hidden = paint_widget
                    .widget
                    .borrow_mut()
                    .properties()
                    .get_bool(PROPERTY_HIDDEN);
                let widget_xy = paint_widget.widget.borrow_mut().properties().get_origin();
                let widget_wh = paint_widget.widget.borrow_mut().properties().get_bounds();

                if !is_hidden {
                    match paint_widget
                        .widget
                        .borrow_mut()
                        .draw(c, &mut self.texture_cache)
                    {
                        Some(texture) => {
                            c.copy(
                                texture,
                                None,
                                Rect::new(
                                    widget_xy.0 as i32,
                                    widget_xy.1 as i32,
                                    widget_wh.0,
                                    widget_wh.1,
                                ),
                            )
                            .unwrap();
                        }
                        None => eprintln!("No texture presented: ID={}", new_id),
                    };

                    paint_widget
                        .widget
                        .borrow_mut()
                        .properties()
                        .delete(PROPERTY_INVALIDATED);
                }
            }

            if new_id as u32 != widget_id {
                self.draw(new_id as u32, c);
            }
        }
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
