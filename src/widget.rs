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

/// This is the `Widget` trait that all implemented `Widget`s need to extend in order to function
/// with the `Pushrod` library.  All functions in this trait (aside from default implementations)
/// should be implemented by the `Widget`.
pub trait Widget {

    fn properties(&mut self) -> &mut WidgetProperties;

    fn set_property(&mut self, property_key: u32, property_value: String) {
        self.properties().set(property_key, property_value.clone());
    }

    fn invalidate(&mut self) {
        self.properties().set(PROPERTY_INVALIDATED, String::from("true"));
    }

    fn invalidated(&mut self) -> bool {
        if self.properties().key_set(PROPERTY_INVALIDATED) {
            self.properties().delete(PROPERTY_INVALIDATED);
            true
        } else {
            false
        }
    }

}
