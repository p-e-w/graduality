// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use csscolorparser::Color;
use serde::Deserialize;

use crate::easing::Easing;

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Adjustment {
    Fit,
    Stretch,
    Zoom,
}

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Interpolation {
    Linear,
    Spline,
}

#[derive(PartialEq, Copy, Clone, Debug, Deserialize)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct Step {
    position: Position,
    color: Color,
    duration: f32,
    easing: Easing,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct Point {
    delay: f32,
    interpolation: Interpolation,
    steps: Vec<Step>,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
struct Configuration {
    adjustment: Adjustment,
    speed: f32,
    #[serde(rename = "point")]
    points: Vec<Point>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let configuration_1: Configuration = toml::from_str(
            r#"
            adjustment = 'stretch'
            speed = 1

            [[point]]
            delay = 0
            interpolation = 'spline'
            steps = [
                { position = { x = -1, y = 1 }, color = 'red', duration = 1, easing = 'linear' },
            ]
            "#,
        )
        .unwrap();

        let configuration_2 = Configuration {
            adjustment: Adjustment::Stretch,
            speed: 1.0,
            points: vec![Point {
                delay: 0.0,
                interpolation: Interpolation::Spline,
                steps: vec![Step {
                    position: Position { x: -1.0, y: 1.0 },
                    color: Color::new(1.0, 0.0, 0.0, 1.0),
                    duration: 1.0,
                    easing: Easing::default(),
                }],
            }],
        };

        assert_eq!(configuration_1, configuration_2);
    }
}
