Utility Converter
===========================

![GitHub repo size](https://img.shields.io/github/repo-size/Luan-Web3/utility-converter?style=for-the-badge)
![GitHub language count](https://img.shields.io/github/languages/count/Luan-Web3/utility-converter?style=for-the-badge)
![GitHub forks](https://img.shields.io/github/forks/Luan-Web3/utility-converter?style=for-the-badge)

This Rust lib offering accurate conversions for temperature, weight, and distance.ses)

## Instalation

```
cargo add utility_converter
```

## Examples

```rust
use utility_converter::converter;

fn main() {
    let fahrenheit: f64 = converter::celsius_to_fahrenheit(25.0);
    ...
}
```

## Other functions

### Temperature

- **`celsius_to_fahrenheit`** — Convert from Celsius to Fahrenheit.
- **`celsius_to_kelvin`** — Convert from Celsius to Kelvin.
- **`fahrenheit_to_celsius`** — Convert from Fahrenheit to Celsius.
- **`fahrenheit_to_kelvin`** — Convert from Fahrenheit to Kelvin.
- **`kelvin_to_celsius`** — Convert from Kelvin to Celsius.
- **`kelvin_to_fahrenheit`** — Convert from Kelvin to Fahrenheit.

### Weight

- **`grams_to_kilograms`** - Convert from Grams to Kilograms.
- **`grams_to_tonnes`** - Convert from Grams to Tonnes.
- **`grams_to_pounds`** - Convert from Grams to Pounds.
- **`kilograms_to_grams`** - Convert from Kilograms to Grams.
- **`kilograms_to_tonnes`** - Convert from Kilograms to Tonnes.
- **`kilograms_to_pounds`** - Convert from Kilograms to Pounds.
- **`tonnes_to_grams`** - Convert from Tonnes to Grams.
- **`tonnes_to_kilograms`** - Convert from Tonnes to Kilograms.
- **`tonnes_to_pounds`** - Convert from Tonnes to Pounds.
- **`pounds_to_grams`** - Convert from Pounds to Grams.
- **`pounds_to_kilograms`** - Convert from Pounds to Kilograms.
- **`pounds_to_tonnes`** - Convert from Pounds to Tonnes.

### Distance

- **`centimeters_to_meters`** - Convert from Centimeters to Meters.
- **`centimeters_to_kilometers`** - Convert from Centimeters to Kilometers.
- **`centimeters_to_inches`** - Convert from Centimeters to Inches.
- **`centimeters_to_miles`** - Convert from Centimeters to Miles.
- **`meters_to_centimeters`** - Convert from Meters to Centimeters.
- **`meters_to_kilometers`** - Convert from Meters to Kilometers.
- **`meters_to_inches`** - Convert from Meters to Inches.
- **`meters_to_miles`** - Convert from Meters to Miles.
- **`kilometers_to_centimeters`** - Convert from Kilometers to Centimeters.
- **`kilometers_to_meters`** - Convert from Kilometers to Meters.
- **`kilometers_to_inches`** - Convert from Kilometers to Inches.
- **`kilometers_to_miles`** - Convert from Kilometers to Miles.
- **`inches_to_centimeters`** - Convert from Inches to Centimeters.
- **`inches_to_meters`** - Convert from Inches to Meters.
- **`inches_to_kilometers`** - Convert from Inches to Kilometers.
- **`inches_to_miles`** - Convert from Inches to Miles.
- **`miles_to_centimeters`** - Convert from Miles to Centimeters.
- **`miles_to_meters`** - Convert from Miles to Meters.
- **`miles_to_kilometers`** - Convert from Miles to Kilometers.
- **`miles_to_inches`** - Convert from Miles to Inches.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
