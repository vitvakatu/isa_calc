use std::io::BufRead;
use std::process::exit;

type Pressure = f64;
type Temperature = f64;
type Density = f64;

const GRAVITY_CONSTANT: f64 = 9.80665;
const GAS_CONSTANT: f64 = 287.00;
const ISA_TEMP: Temperature = 288.15;
const ISA_PRESSURE: Pressure = 101325.0;

struct ProfilePart {
    bottom: u32,
    top: u32,
    lapse_rate: f64,
}

const PROFILES: &[ProfilePart] = &[
    ProfilePart {
        bottom: 0,
        top: 11000,
        lapse_rate: -0.0065,
    },
    ProfilePart {
        bottom: 11000,
        top: 20000,
        lapse_rate: 0.0,
    },
    ProfilePart {
        bottom: 20000,
        top: 32000,
        lapse_rate: 0.001,
    },
    ProfilePart {
        bottom: 32000,
        top: 47000,
        lapse_rate: 0.0028,
    },
    ProfilePart {
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

    for profile in PROFILES {
        let height_diff = altitude.min(profile.top) - profile.bottom;

        pressure = if profile.lapse_rate == 0.0 {
            compute_pressure_isothermal(temperature, pressure, profile, height_diff as f64)
        } else {
            compute_pressure(temperature, pressure, profile, height_diff as f64)
        };

        temperature += temperature_diff(profile, height_diff as f64);

        if altitude <= profile.top {
            break;
        }
    }

    Parameters {
        temperature,
        pressure,
        density: pressure / (GAS_CONSTANT * temperature),
    }
}

fn temperature_diff(profile: &ProfilePart, height_diff: f64) -> f64 {
    profile.lapse_rate * height_diff
}

fn compute_pressure(
    previous_temp: f64,
    pressure: f64,
    profile: &ProfilePart,
    height_diff: f64,
) -> f64 {
    let temp = temperature_diff(profile, height_diff) + previous_temp;
    pressure * (temp / previous_temp).powf(-GRAVITY_CONSTANT / (profile.lapse_rate * GAS_CONSTANT))
}

fn compute_pressure_isothermal(
    previous_temp: f64,
    pressure: f64,
    profile: &ProfilePart,
    height_diff: f64,
) -> f64 {
    let temp = temperature_diff(profile, height_diff) + previous_temp;
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
