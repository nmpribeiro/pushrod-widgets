// Pushrod Widgets
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

/// Properties is the store that each `Widget` uses to define its behavior, using a `HashMap` to
/// store the properties.  Each property is identified by a numeric (u32) key.
pub mod properties;

/// This is the `Widget` trait that all drawable `Widget`s use.  Any special functionality should
/// be defined using interactions with properties.  `Widget`s can decide whether or not to set
/// themselves in `invalidated` state after a property value changes, which indicates to the
/// top-level drawing loop whether or not a `Widget` needs to be redrawn.
pub mod widget;

/// This is a `Widget` cache that is used to store `Widget`s on the display field of a `Window`.
/// Each cached object is stored in a `Vec`, which is accessible using its numeric ID.
pub mod cache;
