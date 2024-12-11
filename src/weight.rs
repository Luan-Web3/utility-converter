pub mod weight_conversation {

    /// # This function convert from grams to kilograms
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.5, converter::grams_to_kilograms(500.0));
    /// assert_eq!(1.0, converter::grams_to_kilograms(1000.0));
    /// assert_eq!(5.0, converter::grams_to_kilograms(5000.0));
    /// ```
    pub fn grams_to_kilograms(grams: f64) -> f64 {
        grams / 1000.0
    }
    
    /// # This function convert from grams to tonnes
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.005, converter::grams_to_tonnes(5000.0));
    /// assert_eq!(0.01, converter::grams_to_tonnes(10000.0));
    /// assert_eq!(0.05, converter::grams_to_tonnes(50000.0));
    /// ```
    pub fn grams_to_tonnes(grams: f64) -> f64 {
        grams / 1000000.0
    }
    
    /// # This function convert from grams to pounds
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(11.0231, converter::grams_to_pounds(5000.0));
    /// assert_eq!(22.0462, converter::grams_to_pounds(10000.0));
    /// assert_eq!(110.231, converter::grams_to_pounds(50000.0));
    /// ```
    pub fn grams_to_pounds(grams: f64) -> f64 {
        grams * 0.00220462
    }

    /// # This function convert from kilograms to grams
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(5000.0, converter::kilograms_to_grams(5.0));
    /// assert_eq!(10000.0, converter::kilograms_to_grams(10.0));
    /// assert_eq!(50000.0, converter::kilograms_to_grams(50.0));
    /// ```
    pub fn kilograms_to_grams(kilograms: f64) -> f64 {
        kilograms * 1000.0
    }

    /// # This function convert from kilograms to tonnes
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.005, converter::kilograms_to_tonnes(5.0));
    /// assert_eq!(0.01, converter::kilograms_to_tonnes(10.0));
    /// assert_eq!(0.05, converter::kilograms_to_tonnes(50.0));
    /// ```
    pub fn kilograms_to_tonnes(kilograms: f64) -> f64 {
        kilograms / 1000.0
    }
    
    /// # This function convert from kilograms to pounds
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(11.0231, converter::kilograms_to_pounds(5.0));
    /// assert_eq!(22.0462, converter::kilograms_to_pounds(10.0));
    /// assert_eq!(110.231, converter::kilograms_to_pounds(50.0));
    /// ```
    pub fn kilograms_to_pounds(kilograms: f64) -> f64 {
        kilograms * 2.20462
    }

    /// # This function convert from tonnes to grams
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(500000.0, converter::tonnes_to_grams(0.5));
    /// assert_eq!(1000000.0, converter::tonnes_to_grams(1.0));
    /// assert_eq!(5000000.0, converter::tonnes_to_grams(5.0));
    /// ```
    pub fn tonnes_to_grams(tonnes: f64) -> f64 {
        tonnes * 1000000.0
    }
    
    /// # This function convert from tonnes to kilograms
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(500.0, converter::tonnes_to_kilograms(0.5));
    /// assert_eq!(1000.0, converter::tonnes_to_kilograms(1.0));
    /// assert_eq!(5000.0, converter::tonnes_to_kilograms(5.0));
    /// ```
    pub fn tonnes_to_kilograms(tonnes: f64) -> f64 {
        tonnes * 1000.0
    }
    
    /// # This function convert from tonnes to pounds
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(1102.31, converter::tonnes_to_pounds(0.5));
    /// assert_eq!(2204.62, converter::tonnes_to_pounds(1.0));
    /// assert_eq!(11023.099999999999, converter::tonnes_to_pounds(5.0));
    /// ```
    pub fn tonnes_to_pounds(tonnes: f64) -> f64 {
        tonnes * 2204.62
    }

    /// # This function convert from pounds to grams
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(226.79645471781984, converter::pounds_to_grams(0.5));
    /// assert_eq!(453.5929094356397, converter::pounds_to_grams(1.0));
    /// assert_eq!(2267.9645471781987, converter::pounds_to_grams(5.0));
    /// ```
    pub fn pounds_to_grams(pounds: f64) -> f64 {
        pounds / 0.00220462
    }
    
    /// # This function convert from pounds to kilograms
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.22679645471781987, converter::pounds_to_kilograms(0.5));
    /// assert_eq!(0.45359290943563974, converter::pounds_to_kilograms(1.0));
    /// assert_eq!(2.267964547178199, converter::pounds_to_kilograms(5.0));
    /// ```
    pub fn pounds_to_kilograms(pounds: f64) -> f64 {
        pounds / 2.20462
    }
    
    /// # This function convert from pounds to tonnes
    /// 
    /// # example
    /// 
    /// ```rust
    /// use utility_converter::converter;
    /// 
    /// assert_eq!(0.0004535929094356397, converter::pounds_to_tonnes(1.0));
    /// assert_eq!(0.0022679645471781985, converter::pounds_to_tonnes(5.0));
    /// assert_eq!(0.022679645471781987, converter::pounds_to_tonnes(50.0));
    /// ```
    pub fn pounds_to_tonnes(pounds: f64) -> f64 {
        pounds / 2204.62
    }
}

#[cfg(test)]
mod test {
    use super::weight_conversation;

    #[test]
    fn test_grams_to_kilograms() {
        let result: f64 = weight_conversation::grams_to_kilograms(1000.0);
        let expect: f64 = 1.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_grams_to_tonnes() {
        let result: f64 = weight_conversation::grams_to_tonnes(1000.0);
        let expect: f64 = 0.001;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_grams_to_pounds() {
        let result: f64 = weight_conversation::grams_to_pounds(1000.0);
        let expect: f64 = 2.20462;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilograms_to_grams() {
        let result: f64 = weight_conversation::kilograms_to_grams(10.0);
        let expect: f64 = 10000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilograms_to_tonnes() {
        let result: f64 = weight_conversation::kilograms_to_tonnes(10.0);
        let expect: f64 = 0.01;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_kilograms_to_pounds() {
        let result: f64 = weight_conversation::kilograms_to_pounds(10.0);
        let expect: f64 = 22.0462;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_tonnes_to_grams() {
        let result: f64 = weight_conversation::tonnes_to_grams(1.0);
        let expect: f64 = 1000000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_tonnes_to_kilograms() {
        let result: f64 = weight_conversation::tonnes_to_kilograms(1.0);
        let expect: f64 = 1000.0;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_tonnes_to_pounds() {
        let result: f64 = weight_conversation::tonnes_to_pounds(1.0);
        let expect: f64 = 2204.62;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_pounds_to_grams() {
        let result: f64 = weight_conversation::pounds_to_grams(10.0);
        let expect: f64 = 4535.929094356397;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_pounds_to_kilograms() {
        let result: f64 = weight_conversation::pounds_to_kilograms(10.0);
        let expect: f64 = 4.535929094356398;
        assert_eq!(result, expect);
    }

    #[test]
    fn test_pounds_to_tonnes() {
        let result: f64 = weight_conversation::pounds_to_tonnes(100.0);
        let expect: f64 = 0.045359290943563974;
        assert_eq!(result, expect);
    }
}