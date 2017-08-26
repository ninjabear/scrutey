use families;
use strategies::strategy::StrategyType;
use ReportCard;
use serde_json;
use serde_json::{Error, Value};

const JSON_ID_NAME: &'static str = "JSON_ONLY";
const FRIENDLY_NAME: &'static str = "JSON";

pub struct JsonStrategy {}

impl StrategyType for JsonStrategy {

    fn id(&self) -> String {
        String::from(JSON_ID_NAME)
    }

    fn child_of_id(&self) -> Option<String> {
        None
    }

    fn family(&self) -> families::Family {
        families::Family::JSON
    }

    fn parse(&self, input: &str) -> ReportCard {
        let parsed: Result<Value, Error> = serde_json::from_str(input);
        if parsed.is_ok() {
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
        assert_eq!(JSON_ID_NAME, JsonStrategy {}.id());
    }

    #[test]
    fn child_of_id() {
        assert_eq!(None, JsonStrategy {}.child_of_id());
    }

    #[test]
    fn family() {
        assert_eq!(families::Family::JSON, JsonStrategy{}.family());
    }

    #[test]
    fn parse_valid_json() {
        let json = r#"
        {
            "foo": "bar"
        }
        "#;
        let strategy = JsonStrategy {};
        let report_card = strategy.parse(json);
        assert_eq!(report_card.family, strategy.family());
        assert_eq!(report_card.known_errors, vec![]);
        assert_eq!(report_card.sureness, 1.0);
        assert_eq!(report_card.type_friendly_name, FRIENDLY_NAME);
    }

    #[test]
    fn parse_invalid_json() {
        let json = "notjson";
        let strategy = JsonStrategy {};
        let report_card = strategy.parse(json);
        assert_eq!(report_card.family, strategy.family());
        assert_eq!(report_card.known_errors, vec![]);
        assert_eq!(report_card.sureness, 0.0);
        assert_eq!(report_card.type_friendly_name, FRIENDLY_NAME);
    }

}