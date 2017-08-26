use families;

#[derive(Debug, PartialEq, Clone)]
pub struct ReportCard {
    pub sureness: f32,
    pub type_friendly_name: String,
    pub family: families::Family,
    pub known_errors: Vec<ParseError>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: u32,
    pub col: u32,
}

impl ParseError {
    fn new(message: &str, line: u32, col: u32) -> Self {
        ParseError {
            message: String::from(message),
            line: line.into(),
            col: col.into()
        }
    }
}

impl ReportCard {
    pub fn new_with_errors(sureness: f32,
                           type_friendly_name: String,
                           family: families::Family,
                           known_errors: Vec<ParseError>)
                           -> Self {
        ReportCard {
            sureness: sureness,
            type_friendly_name: type_friendly_name,
            family: family,
            known_errors: known_errors,
        }
    }

    pub fn new(sureness: f32, type_friendly_name: String, family: families::Family) -> Self {
        ReportCard {
            sureness: sureness,
            type_friendly_name: type_friendly_name,
            family: family,
            known_errors: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sample_card = ReportCard::new(1.0, "friend".into(), families::Family::UNSTRUCTURED);
        assert_eq!(sample_card,
                   ReportCard {
                       sureness: 1.0,
                       type_friendly_name: "friend".into(),
                       family: families::Family::UNSTRUCTURED,
                       known_errors: vec![],
                   })
    }

    #[test]
    fn new_with_errors() {
        let sample_card = ReportCard::new_with_errors(1.0, "friend".into(), families::Family::UNSTRUCTURED, vec![ParseError::new("message", 1, 2)]);
        assert_eq!(sample_card,
                   ReportCard {
                       sureness: 1.0,
                       type_friendly_name: "friend".into(),
                       family: families::Family::UNSTRUCTURED,
                       known_errors: vec![ParseError {
                           message: "message".into(),
                           line: 1,
                           col: 2
                       }],
                   })
    }

}