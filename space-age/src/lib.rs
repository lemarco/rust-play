// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const EARTH_YEAR_SECS: f64 = 31557600.;
macro_rules! impl_planet {
    ($(($type:ident, $ratio:literal)),*) => {
        $(
            pub struct $type;
            impl Planet for $type {
                fn years_during(d: &Duration) -> f64 {
                    d.seconds / EARTH_YEAR_SECS / $ratio
                }
            }
        )*
    }
}

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}
pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}
impl_planet!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
