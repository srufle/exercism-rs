// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        let seconds = seconds as f64;
        Duration { seconds }
    }
}

pub trait Planet {
    // Learned a trait can force the implementer
    // to define the specific value
    const CONVERSION_FACTOR: f64;
    fn years_during(d: &Duration) -> f64;
}

// The Little Book of Rust Macros
// https://veykril.github.io/tlborm/

#[macro_export]
macro_rules! planet {
    // Learned  ident is preferred to ty (type) because
    // it can be used as the name of the struct
    ($name:ident, $factor:expr) => {
        pub struct $name;
        impl Planet for $name {
            const CONVERSION_FACTOR: f64 = $factor;
            fn years_during(d: &Duration) -> f64 {
                d.seconds / (EARTH_YEAR * Self::CONVERSION_FACTOR)
            }
        }
    };
}

// As seconds
const EARTH_YEAR: f64 = 31557600.0;

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
