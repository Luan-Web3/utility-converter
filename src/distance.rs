pub mod distance_conversation {

    /// # This function convert from centimeters to meters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.1, converter::centimeters_to_meters(10.0));
    /// assert_eq!(1.0, converter::centimeters_to_meters(100.0));
    /// assert_eq!(5.0, converter::centimeters_to_meters(500.0));
    /// ```
    pub fn centimeters_to_meters(centimeters: f64) -> f64 {
        centimeters / 100.0
    }
    
    /// # This function convert from centimeters to kilometers
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.001, converter::centimeters_to_kilometers(100.0));
    /// assert_eq!(0.01, converter::centimeters_to_kilometers(1000.0));
    /// assert_eq!(0.05, converter::centimeters_to_kilometers(5000.0));
    /// ```
    pub fn centimeters_to_kilometers(centimeters: f64) -> f64 {
        centimeters / 100000.0
    }
    
    /// # This function convert from centimeters to inches
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(39.3701, converter::centimeters_to_inches(100.0));
    /// assert_eq!(393.701, converter::centimeters_to_inches(1000.0));
    /// assert_eq!(1968.505, converter::centimeters_to_inches(5000.0));
    /// ```
    pub fn centimeters_to_inches(centimeters: f64) -> f64 {
        centimeters * 0.393701
    }
    
    /// # This function convert from centimeters to miles
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(6.213727366498068, converter::centimeters_to_miles(1000000.0));
    /// assert_eq!(62.137273664980675, converter::centimeters_to_miles(10000000.0));
    /// assert_eq!(310.68636832490336, converter::centimeters_to_miles(50000000.0));
    /// ```
    pub fn centimeters_to_miles(centimeters: f64) -> f64 {
        centimeters / 160934.0
    }

    /// # This function convert from meters to centimeters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(100.0, converter::meters_to_centimeters(1.0));
    /// assert_eq!(1000.0, converter::meters_to_centimeters(10.0));
    /// assert_eq!(5000.0, converter::meters_to_centimeters(50.0));
    /// ```
    pub fn meters_to_centimeters(meters: f64) -> f64 {
        meters * 100.0
    }
    
    /// # This function convert from meters to kilometers
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(1.0, converter::meters_to_kilometers(1000.0));
    /// assert_eq!(10.0, converter::meters_to_kilometers(10000.0));
    /// assert_eq!(50.0, converter::meters_to_kilometers(50000.0));
    /// ```
    pub fn meters_to_kilometers(meters: f64) -> f64 {
        meters / 1000.0
    }
    
    /// # This function convert from meters to inches
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(39.3701, converter::meters_to_inches(1.0));
    /// assert_eq!(196.8505, converter::meters_to_inches(5.0));
    /// assert_eq!(393.701, converter::meters_to_inches(10.0));
    /// ```
    pub fn meters_to_inches(meters: f64) -> f64 {
        meters * 39.3701
    }
    
    /// # This function convert from meters to miles
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(6.213727366498068, converter::meters_to_miles(10000.0));
    /// assert_eq!(31.068636832490338, converter::meters_to_miles(50000.0));
    /// assert_eq!(62.137273664980675, converter::meters_to_miles(100000.0));
    /// ```
    pub fn meters_to_miles(meters: f64) -> f64 {
        meters / 1609.34
    }

    /// # This function convert from kilometers to centimeters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(10000.0, converter::kilometers_to_centimeters(0.1));
    /// assert_eq!(50000.0, converter::kilometers_to_centimeters(0.5));
    /// assert_eq!(100000.0, converter::kilometers_to_centimeters(1.0));
    /// ```
    pub fn kilometers_to_centimeters(kilometers: f64) -> f64 {
        kilometers * 100000.0
    }
    
    /// # This function convert from kilometers to meters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(1000.0, converter::kilometers_to_meters(1.0));
    /// assert_eq!(5000.0, converter::kilometers_to_meters(5.0));
    /// assert_eq!(10000.0, converter::kilometers_to_meters(10.0));
    /// ```
    pub fn kilometers_to_meters(kilometers: f64) -> f64 {
        kilometers * 1000.0
    }
    
    /// # This function convert from kilometers to inches
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(39370.1, converter::kilometers_to_inches(1.0));
    /// assert_eq!(196850.5, converter::kilometers_to_inches(5.0));
    /// assert_eq!(393701.0, converter::kilometers_to_inches(10.0));
    /// ```
    pub fn kilometers_to_inches(kilometers: f64) -> f64 {
        kilometers * 39370.1
    }
    
    /// # This function convert from kilometers to miles
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.621371, converter::kilometers_to_miles(1.0));
    /// assert_eq!(3.106855, converter::kilometers_to_miles(5.0));
    /// assert_eq!(6.21371, converter::kilometers_to_miles(10.0));
    /// ```
    pub fn kilometers_to_miles(kilometers: f64) -> f64 {
        kilometers * 0.621371
    }

    /// # This function convert from inches to centimeters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(6.4516, converter::inches_to_centimeters(2.54));
    /// assert_eq!(64.51599999999999, converter::inches_to_centimeters(25.4));
    /// assert_eq!(129.03199999999998, converter::inches_to_centimeters(50.8));
    /// ```
    pub fn inches_to_centimeters(inches: f64) -> f64 {
        inches * 2.54
    }
    
    /// # This function convert from inches to meters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.064516, converter::inches_to_meters(2.54));
    /// assert_eq!(0.64516, converter::inches_to_meters(25.4));
    /// assert_eq!(1.29032, converter::inches_to_meters(50.8));
    /// ```
    pub fn inches_to_meters(inches: f64) -> f64 {
        inches * 0.0254
    }
    
    /// # This function convert from inches to kilometers
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.508, converter::inches_to_kilometers(20000.0));
    /// assert_eq!(5.08, converter::inches_to_kilometers(200000.0));
    /// assert_eq!(10.16, converter::inches_to_kilometers(400000.0));
    /// ```
    pub fn inches_to_kilometers(inches: f64) -> f64 {
        inches * 0.0000254
    }
    
    /// # This function convert from inches to miles
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(1.0, converter::inches_to_miles(63360.0));
    /// assert_eq!(1.5782828282828283, converter::inches_to_miles(100000.0));
    /// assert_eq!(4.008838383838384, converter::inches_to_miles(254000.0));
    /// ```
    pub fn inches_to_miles(inches: f64) -> f64 {
        inches / 63360.0
    }

    /// # This function convert from miles to centimeters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(3000.0, converter::miles_to_centimeters(0.018641182099494204));
    /// assert_eq!(300000.00000000006, converter::miles_to_centimeters(1.8641182099494204));
    /// assert_eq!(402335.0, converter::miles_to_centimeters(2.5));
    /// ```
    pub fn miles_to_centimeters(miles: f64) -> f64 {
        miles * 160934.0
    }
    
    /// # This function convert from miles to meters
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(10000.0, converter::miles_to_meters(6.213727366498068));
    /// assert_eq!(1609.34, converter::miles_to_meters(1.0));
    /// assert_eq!(4023.35, converter::miles_to_meters(2.5));
    /// ```
    pub fn miles_to_meters(miles: f64) -> f64 {
        miles * 1609.34
    }
    
    /// # This function convert from miles to kilometers
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(1.60934, converter::miles_to_kilometers(1.0));
    /// assert_eq!(8.0467, converter::miles_to_kilometers(5.0));
    /// assert_eq!(16.0934, converter::miles_to_kilometers(10.0));
    /// ```
    pub fn miles_to_kilometers(miles: f64) -> f64 {
        miles * 1.60934
    }
    
    /// # This function convert from miles to inches
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(63360.0, converter::miles_to_inches(1.0));
    /// assert_eq!(316800.0, converter::miles_to_inches(5.0));
    /// assert_eq!(633600.0, converter::miles_to_inches(10.0));
    /// ```
    pub fn miles_to_inches(miles: f64) -> f64 {
        miles * 63360.0
    }
}

#[cfg(test)]
mod test {
    use super::distance_conversation;

    #[test]
    fn test_centimeters_to_meters() {
        let result: f64 = distance_conversation::centimeters_to_meters(30.0);
        let expect: f64 = 0.3;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_centimeters_to_kilometers() {
        let result: f64 = distance_conversation::centimeters_to_kilometers(3000.0);
        let expect: f64 = 0.03;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_centimeters_to_inches() {
        let result: f64 = distance_conversation::centimeters_to_inches(3000.0);
        let expect: f64 = 1181.103;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_centimeters_to_miles() {
        let result: f64 = distance_conversation::centimeters_to_miles(3000.0);
        let expect: f64 = 0.018641182099494204;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_meters_to_centimeters() {
        let result: f64 = distance_conversation::meters_to_centimeters(1.0);
        let expect: f64 = 100.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_meters_to_kilometers() {
        let result: f64 = distance_conversation::meters_to_kilometers(1000.0);
        let expect: f64 = 1.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_meters_to_inches() {
        let result: f64 = distance_conversation::meters_to_inches(1.0);
        let expect: f64 = 39.3701;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_meters_to_miles() {
        let result: f64 = distance_conversation::meters_to_miles(10000.0);
        let expect: f64 = 6.213727366498068;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilometers_to_centimeters() {
        let result: f64 = distance_conversation::kilometers_to_centimeters(0.1);
        let expect: f64 = 10000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilometers_to_meters() {
        let result: f64 = distance_conversation::kilometers_to_meters(1.0);
        let expect: f64 = 1000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilometers_to_inches() {
        let result: f64 = distance_conversation::kilometers_to_inches(1.0);
        let expect: f64 = 39370.1;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilometers_to_miles() {
        let result: f64 = distance_conversation::kilometers_to_miles(1.0);
        let expect: f64 = 0.621371;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_inches_to_centimeters() {
        let result: f64 = distance_conversation::inches_to_centimeters(2.54);
        let expect: f64 = 6.4516;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_inches_to_meters() {
        let result: f64 = distance_conversation::inches_to_meters(2.54);
        let expect: f64 = 0.064516;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_inches_to_kilometers() {
        let result: f64 = distance_conversation::inches_to_kilometers(200000.0);
        let expect: f64 = 5.08;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_inches_to_miles() {
        let result: f64 = distance_conversation::inches_to_miles(254000.0);
        let expect: f64 = 4.008838383838384;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_miles_to_centimeters() {
        let result: f64 = distance_conversation::miles_to_centimeters(0.018641182099494204);
        let expect: f64 = 3000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_miles_to_meters() {
        let result: f64 = distance_conversation::miles_to_meters(6.213727366498068);
        let expect: f64 = 10000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_miles_to_kilometers() {
        let result: f64 = distance_conversation::miles_to_kilometers(1.0);
        let expect: f64 = 1.60934;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_miles_to_inches() {
        let result: f64 = distance_conversation::miles_to_inches(1.0);
        let expect: f64 = 63360.0;
        assert_eq!(result, expect);
    }
}