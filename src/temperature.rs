pub mod temperature_conversation {

    /// # This function convert from celsius to fahrenheit
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(59.0, converter::celsius_to_fahrenheit(15.0));
    /// assert_eq!(77.0, converter::celsius_to_fahrenheit(25.0));
    /// assert_eq!(86.0, converter::celsius_to_fahrenheit(30.0));
    /// ```
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    /// # This function convert from celsius to kelvin
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(288.15, converter::celsius_to_kelvin(15.0));
    /// assert_eq!(298.15, converter::celsius_to_kelvin(25.0));
    /// assert_eq!(303.15, converter::celsius_to_kelvin(30.0));
    /// ```
    pub fn celsius_to_kelvin(celsius: f64) -> f64 {
        celsius + 273.15
    }

    /// # This function convert from fahrenheit to celsius
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(15.0, converter::fahrenheit_to_celsius(59.0));
    /// assert_eq!(25.0, converter::fahrenheit_to_celsius(77.0));
    /// assert_eq!(30.0, converter::fahrenheit_to_celsius(86.0));
    /// ```
    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    /// # This function convert from fahrenheit to kelvin
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(288.15, converter::fahrenheit_to_kelvin(59.0));
    /// assert_eq!(298.15, converter::fahrenheit_to_kelvin(77.0));
    /// assert_eq!(303.15, converter::fahrenheit_to_kelvin(86.0));
    /// ```
    pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
        let celsius = fahrenheit_to_celsius(fahrenheit);
        return celsius_to_kelvin(celsius);
    }

    /// # This function convert from kelvin to celsius
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(15.0, converter::kelvin_to_celsius(288.15));
    /// assert_eq!(25.0, converter::kelvin_to_celsius(298.15));
    /// assert_eq!(30.0, converter::kelvin_to_celsius(303.15));
    /// ```
    pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
        kelvin - 273.15
    }

    /// # This function convert from kelvin to fahrenheit
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(59.0, converter::kelvin_to_fahrenheit(288.15));
    /// assert_eq!(77.0, converter::kelvin_to_fahrenheit(298.15));
    /// assert_eq!(86.0, converter::kelvin_to_fahrenheit(303.15));
    /// ```
    pub fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
        let celsius = kelvin_to_celsius(kelvin);
        return celsius_to_fahrenheit(celsius);
    }
}

#[cfg(test)]
mod test {
    use super::temperature_conversation;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let result: f64 = temperature_conversation::celsius_to_fahrenheit(25.0);
        let expect: f64 = 77.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let result: f64 = temperature_conversation::celsius_to_kelvin(25.0);
        let expect: f64 = 298.15;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let result: f64 = temperature_conversation::fahrenheit_to_celsius(77.0);
        let expect: f64 = 25.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        let result: f64 = temperature_conversation::fahrenheit_to_kelvin(77.0);
        let expect: f64 = 298.15;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        let result: f64 = temperature_conversation::kelvin_to_celsius(298.15);
        let expect: f64 = 25.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        let result: f64 = temperature_conversation::kelvin_to_fahrenheit(298.15);
        let expect: f64 = 77.0;
        assert_eq!(result, expect);
    }
}