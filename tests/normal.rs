// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate approx;
#[macro_use]
extern crate cgmath;

use cgmath::*;

#[test]
fn test_constructor() {
    assert_eq!(norm2(1f32, 2f32), Normal2::new(1f32, 2f32));
    assert_eq!(norm3(1f64, 2f64, 3f64), Normal3::new(1f64, 2f64, 3f64));
    assert_eq!(norm4(1isize, 2isize, 3isize, 4isize), Normal4::new(1isize, 2isize, 3isize, 4isize));
}

#[test]
fn test_from_value() {
    assert_eq!(Normal2::from_value(102isize), Normal2::new(102isize, 102isize));
    assert_eq!(Normal3::from_value(22isize), Normal3::new(22isize, 22isize, 22isize));
    assert_eq!(Normal4::from_value(76.5f64), Normal4::new(76.5f64, 76.5f64, 76.5f64, 76.5f64));
}
