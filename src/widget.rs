// Pushrod Widgets
// Widget Definition
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

use crate::properties::{WidgetProperties, PROPERTY_INVALIDATED};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use crate::caches::TextureCache;

/// This is the `Widget` trait that all implemented `Widget`s need to extend in order to function
/// with the `Pushrod` library.  All functions in this trait (aside from default implementations)
/// should be implemented by the `Widget`.
pub trait Widget {

    /// This provides access to the `WidgetProperties` set for a `Widget`.  These must be defined
    /// in the structure of the `Widget`, as they allow for direct manipulation of the properties.
    fn properties(&mut self) -> &mut WidgetProperties;

    /// This provides an entry point to draw the contents of a `Widget` on to a `Texture`.
    fn draw(&mut self, c: &mut Canvas<Window>, t: &mut TextureCache) -> Option<&Texture> {
        None
    }

    /// Sets a property for a `Widget`.
    fn set_property(&mut self, property_key: u32, property_value: String) {
        self.properties().set(property_key, property_value.clone());
        // Send signal that a property changed to the engine.
    }

    /// Set the invalidation key for this `Widget`, indicating that the `TextureCache` needs to
    /// be redrawn.
    fn invalidate(&mut self) {
        self.properties().set(PROPERTY_INVALIDATED, String::from("true"));
    }

    /// Flag indicating whether or not the `draw` method needs to be called for this `Widget` so
    /// that its `TextureCache` is refreshed.
    fn invalidated(&mut self) -> bool {
        if self.properties().key_set(PROPERTY_INVALIDATED) {
            self.properties().delete(PROPERTY_INVALIDATED);
            true
        } else {
            false
        }
    }

}
