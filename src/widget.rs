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

    /// Draws the widget.  If you wish to modify the canvas object, you must declare it as `mut` in
    /// your implementation (ie `fn draw(&mut self, mut canvas: Canvas<Window>)`).  The `_canvas`
    /// is the currently active drawing canvas at the time this function is called.  This called
    /// during the draw loop of the `Engine`.  This returns a reference to the stored `Texture` object
    /// within the `Widget`.  It is then copied to the canvas, and displayed in the display loop.
    /// In this function, you can just return a reference to the `Texture` if no invalidation state
    /// was set, otherwise, the draw can be re-performed, and the `Texture` returned.  If the drawing
    /// function returns no texture, return a `None`, and it will not be rendered during the display
    /// loop, but it will still be called.  A `TextureCache` is provided in case your `Widget` needs
    /// to cache an image or a font store.
    ///
    /// So, why not just call `draw` each time, if the `Engine` already handles the calling of the
    /// draw for you when an object needs invalidation?  This is to avoid excess CPU usage.  You
    /// **can** call the draw method each time: all it will do is return the reference to the already
    /// drawn `Texture` if you do this.  It's only at the time the contents needs to be redrawn will
    /// the logic for the draw take place (so long the `invalidated` state is obeyed)
    fn draw(&mut self, _c: &mut Canvas<Window>, _t: &mut TextureCache) -> Option<&Texture> {
        None
    }

    /// Sets a property for a `Widget`.
    fn set_property(&mut self, property_key: u32, property_value: String) {
        self.properties().set(property_key, property_value);
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
        self.properties().key_set(PROPERTY_INVALIDATED)
    }

}
