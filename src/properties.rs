// Pushrod Widgets
// Properties
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
use std::u32;
use sdl2::pixels::Color;

pub const PROPERTY_NATIVE_WIDGET_ADDER: u32 = 0;
pub const PROPERTY_INVALIDATED: u32 = 1;
pub const PROPERTY_POSITION: u32 = 2;
pub const PROPERTY_SIZE: u32 = 3;
pub const PROPERTY_TEXT: u32 = 4;
pub const PROPERTY_MAIN_COLOR: u32 = 5;
pub const PROPERTY_BACKGROUND_COLOR: u32 = 6;

/// This is a structure that stores properties for Widgets, which can be used to define the object's
/// behavior.
#[derive(Debug, Clone, Default)]
pub struct WidgetProperties {
    properties: HashMap<u32, String>,
}

/// This is the implementation of the `WidgetProperties` store.  This is used by each and every
/// `Widget`, which stores information about the property.  Once each property is set, the `Widget`
/// can respond to the set action, and repaint itself, or anything similar.
impl WidgetProperties {
    /// Sets a value for a property based on its numerical key.
    pub fn set(&mut self, property_key: u32, property_value: String) {
        self.properties.insert(property_key, property_value);
    }

    /// Deletes a property value for the given numerical key.
    pub fn delete(&mut self, property_key: u32) {
        self.properties.remove(&property_key);
    }

    /// Retrieves the value for a property.
    pub fn get(&mut self, property_key: u32) -> String {
        self.properties.get(&property_key).unwrap_or(&String::from("")).clone()
    }

    /// Returns a flag indicating whether or not a property for a numerical key has been set.
    pub fn key_set(&mut self, property_key: u32) -> bool {
        self.properties.contains_key(&property_key)
    }

    /// Retrieves a color based on the given property key.  If the color cannot be found, the
    /// `default_color` specified will be returned.
    pub fn get_color(&self, property_key: u32, default_color: Color) -> Color {
        if self.properties.contains_key(&property_key) {
            let color_spec = self.properties.get(&property_key).unwrap();
            let color_u32 = u32::from_str_radix(color_spec, 16).unwrap();
            let color_b = (color_u32 & 0x0000FF) as u8;
            let color_g = ((color_u32 & 0x00FF00) >> 8) as u8;
            let color_r = ((color_u32 & 0xFF0000) >> 16) as u8;

            Color::RGB(color_r, color_g, color_b)
        } else {
            default_color
        }
    }

    /// Retrieves the stored bounds as a tuple.  If the bounds cannot be found, invisible bounds
    /// are returned (0x0).
    pub fn get_bounds(&self, property_key: u32) -> (u32, u32) {
        if self.properties.contains_key(&property_key) {
            let tokens: Vec<&str> = self.properties.get(&property_key).unwrap().split(' ').collect();

            (u32::from_str_radix(tokens[0], 10).unwrap(), u32::from_str_radix(tokens[1], 10).unwrap())
        } else {
            (0_u32, 0_u32)
        }
    }

}
