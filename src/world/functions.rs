use noise::{NoiseFn, OpenSimplex, BasicMulti, Worley};

pub struct MountainNoise {
    simplex: OpenSimplex,
    worley: Worley,
}

fn clip(value: f64) -> f64 {
    value.min(1.0).max(0.0)
}

// Converts a value from range oldMin-oldMax to range 0-1
fn map_from_range(value: f64, old_min: f64, old_max: f64) -> f64 {
    clip((value - old_min) / (old_max - old_min))
}

// Converts a vale from range 0-1 to range min-max
fn map_to_range(value: f64, min: f64, max: f64) -> f64 {
    clip(value * (max - min) + min)
}

fn magnitude(dx: f64, dy: f64) -> f64 {
    return (dx * dx + dy * dy).sqrt();
}

impl MountainNoise {
    pub fn new() -> MountainNoise {
        let worley = Worley::new();
        // Enables using distance to the nearest point
        let worley = worley.enable_range(true);
        // Disables adding the value of the nearest point.
        let worley = worley.set_displacement(0.0);
        MountainNoise {
            simplex: OpenSimplex::new(),
            worley,
        }
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        // Macroscopic details.
        // For some reason, worley's distance starts at -1.
        let mut base = self.worley.get([x, y]) + 1.0;

        // Smaller details.
        let mut detail = self.worley.get([x * 4.0, y * 4.0]) + 1.0;
        detail = map_to_range(detail, 0.73, 1.0);
        // Only have details close to high points on the macroscopic texture.
        detail *= map_from_range(base, 0.34, 0.79);

        // Clip low values.
        base = map_from_range(base, 0.4, 1.0);
        // Add in the smaller details.
        base += detail;
        base /= 2.0;
        // Make everything more slopey.
        base = base.powf(2.2);

        // Get some large Simplex noise.
        let mut rustle = self.simplex.get([x * 0.8, y * 0.8]) + 0.5;
        rustle = map_to_range(map_from_range(rustle, 0.15, 1.0), 0.15, 1.0);
        rustle = rustle.powf(2.0);
        // Use it to vary the height of our mountains, to make it less monotonous.
        base *= rustle;

        base
    }
}

pub struct MountainNoise2 {
    simplex: BasicMulti,
}

impl MountainNoise2 {
    pub fn new() -> MountainNoise2 {
        let mut result = MountainNoise2 {
            simplex: BasicMulti::new(),
        };
        result.simplex.persistence = 0.5;
        result
    }

    fn get_noise(&self, coord: [f64; 2]) -> f64 {
        self.simplex.get(coord) * 0.5 + 0.5
    }

    pub fn get(&self, x: f64, y: f64) -> f64 {
        let d = 0.2;
        let left = self.get_noise([x - d, y]);
        let right = self.get_noise([x + d, y]);
        let up = self.get_noise([x, y - d]);
        let down = self.get_noise([x, y + d]);
        let [dx, dy] = [(right - left) / (d * 2.0), (down - up) / (d * 2.0)];
        let slope = magnitude(dx, dy);

        let base = self.get_noise([x, y]);
        let eroded = base + (1.0 - slope) * 0.7;
        (eroded / 1.5).powf(2.6)
    }
}
