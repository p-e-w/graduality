// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use std::{
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

use phf::{phf_map, Map};
use serde::{de::Error, Deserialize, Deserializer};
use simple_easing::{
    back_in, back_in_out, back_out, bounce_in, bounce_in_out, bounce_out, circ_in, circ_in_out,
    circ_out, cubic_in, cubic_in_out, cubic_out, elastic_in, elastic_in_out, elastic_out, expo_in,
    expo_in_out, expo_out, linear, quad_in, quad_in_out, quad_out, quart_in, quart_in_out,
    quart_out, quint_in, quint_in_out, quint_out, reverse, roundtrip, sine_in, sine_in_out,
    sine_out,
};

static EASING_FUNCTIONS: Map<&'static str, fn(f32) -> f32> = phf_map! {
    "back_in" => back_in,
    "back_out" => back_out,
    "back_in_out" => back_in_out,
    "bounce_in" => bounce_in,
    "bounce_out" => bounce_out,
    "bounce_in_out" => bounce_in_out,
    "circ_in" => circ_in,
    "circ_out" => circ_out,
    "circ_in_out" => circ_in_out,
    "cubic_in" => cubic_in,
    "cubic_out" => cubic_out,
    "cubic_in_out" => cubic_in_out,
    "elastic_in" => elastic_in,
    "elastic_out" => elastic_out,
    "elastic_in_out" => elastic_in_out,
    "expo_in" => expo_in,
    "expo_out" => expo_out,
    "expo_in_out" => expo_in_out,
    "quad_in" => quad_in,
    "quad_out" => quad_out,
    "quad_in_out" => quad_in_out,
    "quart_in" => quart_in,
    "quart_out" => quart_out,
    "quart_in_out" => quart_in_out,
    "quint_in" => quint_in,
    "quint_out" => quint_out,
    "quint_in_out" => quint_in_out,
    "sine_in" => sine_in,
    "sine_out" => sine_out,
    "sine_in_out" => sine_in_out,
    "linear" => linear,
    "reverse" => reverse,
    "roundtrip" => roundtrip,
};

#[derive(Clone)]
pub struct Easing(Rc<dyn Fn(f32) -> f32>);

impl Default for Easing {
    fn default() -> Self {
        Self(Rc::new(linear))
    }
}

impl PartialEq for Easing {
    #[allow(unused_variables)]
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Debug for Easing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "?")
    }
}

impl<'de> Deserialize<'de> for Easing {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;

        let function = EASING_FUNCTIONS.get(&name).ok_or(Error::custom(format!(
            "invalid easing function name: {}",
            name,
        )))?;

        Ok(Self(Rc::new(function)))
    }
}
