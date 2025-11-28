use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeightClassDto {
    UnderOrEqual(WeightDto),
    Over(WeightDto),
    None,
}

impl FromStr for WeightClassDto {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::None);
        }

        if let Some(v) = s.strip_suffix('+') {
            v.parse::<WeightDto>().map(Self::Over)
        } else {
            s.parse::<WeightDto>().map(Self::UnderOrEqual)
        }
    }
}

impl Display for WeightClassDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnderOrEqual(weight) => write!(f, "{weight}"),
            Self::Over(weight) => write!(f, "+{weight}"),
            Self::None => write!(f, ""),
        }
    }
}

struct WeightClassVisitor;

impl Visitor<'_> for WeightClassVisitor {
    type Value = WeightClassDto;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid weight class")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<WeightClassDto, E> {
        WeightClassDto::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightClassDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(WeightClassVisitor)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::WeightDto;
    use super::WeightClassDto;

    #[rstest]
    #[case("", WeightClassDto::None)]
    #[case("43", WeightClassDto::UnderOrEqual(WeightDto(43.)))]
    #[case("44", WeightClassDto::UnderOrEqual(WeightDto(44.)))]
    #[case("47", WeightClassDto::UnderOrEqual(WeightDto(47.)))]
    #[case("48", WeightClassDto::UnderOrEqual(WeightDto(48.)))]
    #[case("52", WeightClassDto::UnderOrEqual(WeightDto(52.)))]
    #[case("53", WeightClassDto::UnderOrEqual(WeightDto(53.)))]
    #[case("56", WeightClassDto::UnderOrEqual(WeightDto(56.)))]
    #[case("57", WeightClassDto::UnderOrEqual(WeightDto(57.)))]
    #[case("59", WeightClassDto::UnderOrEqual(WeightDto(59.)))]
    #[case("60", WeightClassDto::UnderOrEqual(WeightDto(60.)))]
    #[case("63", WeightClassDto::UnderOrEqual(WeightDto(63.)))]
    #[case("66", WeightClassDto::UnderOrEqual(WeightDto(66.)))]
    #[case("69", WeightClassDto::UnderOrEqual(WeightDto(69.)))]
    #[case("72", WeightClassDto::UnderOrEqual(WeightDto(72.)))]
    #[case("74", WeightClassDto::UnderOrEqual(WeightDto(74.)))]
    #[case("76", WeightClassDto::UnderOrEqual(WeightDto(76.)))]
    #[case("83", WeightClassDto::UnderOrEqual(WeightDto(83.)))]
    #[case("84", WeightClassDto::UnderOrEqual(WeightDto(84.)))]
    #[case("93", WeightClassDto::UnderOrEqual(WeightDto(93.)))]
    #[case("105", WeightClassDto::UnderOrEqual(WeightDto(105.)))]
    #[case("120", WeightClassDto::UnderOrEqual(WeightDto(120.)))]
    #[case("63+", WeightClassDto::Over(WeightDto(63.)))]
    #[case("83+", WeightClassDto::Over(WeightDto(83.)))]
    #[case("84+", WeightClassDto::Over(WeightDto(84.)))]
    #[case("105+", WeightClassDto::Over(WeightDto(105.)))]
    #[case("120+", WeightClassDto::Over(WeightDto(120.)))]
    fn test_deserialize(
        #[case] input: &str,
        #[case] expected: WeightClassDto,
    ) {
        let result: Result<WeightClassDto> = input.parse::<WeightClassDto>();

        assert!(result.is_ok());
        assert_eq!(expected, result.unwrap());
    }

    #[rstest]
    #[case(WeightClassDto::None, String::new())]
    #[case(WeightClassDto::UnderOrEqual(WeightDto(83.)), "83".to_string())]
    #[case(WeightClassDto::Over(WeightDto(120.)), "+120".to_string())]
    fn test_display(
        #[case] input: WeightClassDto,
        #[case] expected: String
    ) {
        let result: String = format!("{}", input);

        assert_eq!(expected, result);
    }
}
