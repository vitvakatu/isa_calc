// Copyright 2019 Ilya Bogdanov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::BufRead;
use std::process::exit;

type Pressure = f64;
type Temperature = f64;
type Density = f64;

const GRAVITY_CONSTANT: f64 = 9.80665;
const GAS_CONSTANT: f64 = 287.00;
const ISA_TEMP: Temperature = 288.15;
const ISA_PRESSURE: Pressure = 101325.0;

struct ProfileSegment {
    bottom: u32,
    top: u32,
    lapse_rate: f64,
}

const SEGMENTS: &[ProfileSegment] = &[
    ProfileSegment {
        bottom: 0,
        top: 11000,
        lapse_rate: -0.0065,
    },
    ProfileSegment {
        bottom: 11000,
        top: 20000,
        lapse_rate: 0.0,
    },
    ProfileSegment {
        bottom: 20000,
        top: 32000,
        lapse_rate: 0.001,
    },
    ProfileSegment {
        bottom: 32000,
        top: 47000,
        lapse_rate: 0.0028,
    },
    ProfileSegment {
        bottom: 47000,
        top: 50000,
        lapse_rate: 0.0,
    },
];

#[derive(Debug, Default, Clone, Copy)]
struct Parameters {
    temperature: Temperature,
    pressure: Pressure,
    density: Density,
}

fn compute(altitude: u32) -> Parameters {
    let mut temperature = ISA_TEMP;
    let mut pressure = ISA_PRESSURE;

    for segment in SEGMENTS {
        let height_diff = altitude.min(segment.top) - segment.bottom;

        pressure = if segment.lapse_rate == 0.0 {
            compute_pressure_isothermal(temperature, pressure, segment, height_diff as f64)
        } else {
            compute_pressure(temperature, pressure, segment, height_diff as f64)
        };

        temperature += temperature_diff(segment, height_diff as f64);

        if altitude <= segment.top {
            break;
        }
    }

    Parameters {
        temperature,
        pressure,
        density: pressure / (GAS_CONSTANT * temperature),
    }
}

fn temperature_diff(segment: &ProfileSegment, height_diff: f64) -> f64 {
    segment.lapse_rate * height_diff
}

fn compute_pressure(
    previous_temp: f64,
    pressure: f64,
    segment: &ProfileSegment,
    height_diff: f64,
) -> f64 {
    let temp = temperature_diff(segment, height_diff) + previous_temp;
    pressure * (temp / previous_temp).powf(-GRAVITY_CONSTANT / (segment.lapse_rate * GAS_CONSTANT))
}

fn compute_pressure_isothermal(
    previous_temp: f64,
    pressure: f64,
    segment: &ProfileSegment,
    height_diff: f64,
) -> f64 {
    let temp = temperature_diff(segment, height_diff) + previous_temp;
    pressure * (-GRAVITY_CONSTANT / (temp * GAS_CONSTANT) * height_diff).exp()
}

fn main() {
    println!("Enter the altitude (in meters): ");
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();

    let altitude: u32 = input.trim().parse().unwrap();

    if altitude > 50000 {
        println!("Cannot calculate for more than 50 km");
        exit(0);
    }

    let params = compute(altitude);

    println!("Answer: {:?}", params);
}
