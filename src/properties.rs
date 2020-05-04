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

pub const PROPERTY_NATIVE_WIDGET_ADDER: u32 = 0;
pub const PROPERTY_INVALIDATED: u32 = 1 + PROPERTY_NATIVE_WIDGET_ADDER;
pub const PROPERTY_POSITION: u32 = 1 + PROPERTY_INVALIDATED;

/// This is a structure that stores properties for Widgets, which can be used to define the object's
/// behavior.
#[derive(Debug, Clone)]
pub struct WidgetProperties {
    properties: HashMap<u32, String>,
}

/// This is the implementation of the `WidgetProperties` store.  This is used by each and every
/// `Widget`, which stores information about the property.  Once each property is set, the `Widget`
/// can respond to the set action, and repaint itself, or anything similar.
impl WidgetProperties {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
}
