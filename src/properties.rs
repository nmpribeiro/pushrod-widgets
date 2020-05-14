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

use sdl2::pixels::Color;
use std::collections::HashMap;
use std::u32;

pub const PROPERTY_NATIVE_WIDGET_ADDER: u32 = 0;
pub const PROPERTY_INVALIDATED: u32 = 1;
pub const PROPERTY_HIDDEN: u32 = 2;
pub const PROPERTY_ORIGIN: u32 = 3;
pub const PROPERTY_SIZE: u32 = 4;
pub const PROPERTY_TEXT: u32 = 5;
pub const PROPERTY_MAIN_COLOR: u32 = 6;
pub const PROPERTY_BACKGROUND_COLOR: u32 = 7;

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
    /*
     * PRIVATE MEMBERS
     */
    #[inline]
    fn get_tuples(&self, property_key: u32, default_tuple: (u32, u32)) -> (u32, u32) {
        if self.properties.contains_key(&property_key) {
            let tokens: Vec<&str> = self
                .properties
                .get(&property_key)
                .unwrap()
                .split(' ')
                .collect();

            (
                u32::from_str_radix(tokens[0], 10).unwrap(),
                u32::from_str_radix(tokens[1], 10).unwrap(),
            )
        } else {
            default_tuple
        }
    }

    /*
     * PUBLIC MEMBERS
     */

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
        self.properties
            .get(&property_key)
            .unwrap_or(&String::from(""))
            .clone()
    }

    /// Returns a flag indicating whether or not a property for a numerical key has been set.
    pub fn key_set(&mut self, property_key: u32) -> bool {
        self.properties.contains_key(&property_key)
    }

    /// Sets the invalidated state for the `Widget`.
    pub fn invalidate(&mut self) {
        self.set(PROPERTY_INVALIDATED, String::from("1"));
    }

    /// Stores the color for the specified key.  Format is "r g b a", as numerical values, base 10.
    /// Sets the invalidate flag afterward.
    pub fn set_color(&mut self, property_key: u32, color: Color) {
        self.set(
            property_key,
            format!("{} {} {} {}", color.r, color.g, color.b, color.a),
        );
        self.invalidate();
    }

    /// Sets the size of the `Widget`.  Sets the invalidate flag afterward.
    pub fn set_bounds(&mut self, w: u32, h: u32) {
        self.set(PROPERTY_SIZE, format!("{} {}", w, h));
        self.invalidate();
    }

    /// Sets the origin for the `Widget`.  Does not set the invalidate flag, as the repositioning of
    /// the `Widget` does not require a repaint.
    pub fn set_origin(&mut self, x: u32, y: u32) {
        self.set(PROPERTY_ORIGIN, format!("{} {}", x, y));
    }

    /// Sets a boolean for a given property key.
    pub fn set_bool(&mut self, property_key: u32) {
        self.set(property_key, String::from("1"));
    }

    /// Retrieves a color based on the given property key.  If the color cannot be found, the
    /// `default_color` specified will be returned.
    pub fn get_color(&self, property_key: u32, default_color: Color) -> Color {
        if self.properties.contains_key(&property_key) {
            let tokens: Vec<u8> = dbg!(self
                .properties
                .get(&property_key)
                .unwrap()
                .split(' ')
                .map(|x| (u32::from_str_radix(x, 10).unwrap()) as u8))
            .collect();

            Color::RGBA(tokens[0], tokens[1], tokens[2], tokens[3])
        } else {
            default_color
        }
    }

    /// Retrieves the stored bounds as a tuple.  If the bounds cannot be found, invisible bounds
    /// are returned (0x0).
    pub fn get_bounds(&self) -> (u32, u32) {
        self.get_tuples(PROPERTY_SIZE, (0_u32, 0_u32))
    }

    /// Retrieves the origin of the `Widget`.  If the origin cannot be found, an origin of 0x0 is
    /// returned.
    pub fn get_origin(&self) -> (u32, u32) {
        self.get_tuples(PROPERTY_ORIGIN, (0_u32, 0_u32))
    }

    /// Retrieves the boolean value for a specified property.  If the property has not been set
    /// with `set_bool`, the return will be `false`.  Otherwise, if the specified key exists, and
    /// the value is set to `1`, the return value will be `true`.
    pub fn get_bool(&self, property_key: u32) -> bool {
        if self.properties.contains_key(&property_key) {
            if self.properties.get(&property_key).unwrap() == "1" {
                return true;
            }
        }

        false
    }
}
