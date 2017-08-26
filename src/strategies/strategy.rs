// trait making all strategies the same shape

// strategy id (uniquie identifier)
// family (if you were to render it with a syntax highlighter)
// child of id (if a subset of a parent type, e.g. self describing json is a child of json)

use families;
use ReportCard;

pub trait StrategyType {
    fn id(&self) -> String;
    fn child_of_id(&self) -> Option<String>;
    fn family(&self) -> families::Family;
    fn parse(&self, &str) -> ReportCard;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStrategy {}

    impl StrategyType for TestStrategy {
        fn id(&self) -> String {
            String::from("id")
        }

        fn child_of_id(&self) -> Option<String> {
            None
        }

        fn family(&self) -> families::Family {
            families::Family::UNSTRUCTURED
        }

        fn parse(&self, _: &str) -> ReportCard {
            ReportCard::new(0.0, "none".into(), self.family())
        }
    }

    #[test]
    fn has_id() {
        let id = TestStrategy {}.id();
        assert_eq!(id, "id");
    }

    #[test]
    fn has_child_of_id() {
        let child_of_id = TestStrategy {}.child_of_id();
        assert_eq!(child_of_id, None);
    }

    #[test]
    fn has_family() {
        let family = TestStrategy {}.family();
        assert_eq!(family, families::Family::UNSTRUCTURED);
    }

    #[test]
    fn parse() {
        let parsed = TestStrategy {}.parse("hello world");
        assert_eq!(parsed,
                   ReportCard::new(0.0, "none".into(), TestStrategy {}.family()))
    }
}