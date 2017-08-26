use families;
use strategies::strategy::StrategyType;
use ReportCard;
use regex::Regex;

const B64_ID_NAME: &'static str = "B64_ONLY";
const FRIENDLY_NAME: &'static str = "Base64";

pub struct Base64Strategy {}

impl StrategyType for Base64Strategy {
    fn id(&self) -> String {
        String::from(B64_ID_NAME)
    }

    fn child_of_id(&self) -> Option<String> {
        None
    }

    fn family(&self) -> families::Family {
        families::Family::BASE64
    }

    fn parse(&self, input: &str) -> ReportCard {
        let re = Regex::new(r"^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{4}|[A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)$").unwrap();

        if re.is_match(input) {
            ReportCard::new(1.0, FRIENDLY_NAME.into(), self.family())
        } else {
            ReportCard::new(0.0, FRIENDLY_NAME.into(), self.family())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use families;
    

    #[test]
    fn id() {
        assert_eq!(Base64Strategy {}.id(), B64_ID_NAME);
    }

    #[test]
    fn child_of_id() {
        assert_eq!(None, Base64Strategy {}.child_of_id());
    }

    #[test]
    fn family() {
        assert_eq!(Base64Strategy {}.family(), families::Family::BASE64);
    }

    #[test]
    fn parse_good() {
        let good_base64 = "aGVsbG8gd29ybGQ=";
        let report_card = Base64Strategy{}.parse(good_base64);
        assert_eq!(report_card.family, Base64Strategy{}.family());
        assert_eq!(report_card.known_errors, vec![]);
        assert_eq!(report_card.sureness, 1.0);
        assert_eq!(report_card.type_friendly_name, FRIENDLY_NAME);
    }

    #[test]
    fn parse_nonsense() {
        let bad_b64 = "potato";
        let report_card =  Base64Strategy{}.parse(bad_b64);
        assert_eq!(report_card.family, Base64Strategy{}.family());
        assert_eq!(report_card.known_errors, vec![]);
        assert_eq!(report_card.sureness, 0.0);
        assert_eq!(report_card.type_friendly_name, FRIENDLY_NAME);
    }

}