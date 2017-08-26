// use all the strategies and get report cards
use strategies::base64::Base64Strategy;
use strategies::json::JsonStrategy;
use strategies::strategy::StrategyType;
use ReportCard;

pub trait Checker {
    fn check(&self, check: &str) -> Vec<ReportCard>;
}

pub struct NaiveChecker {
    strategy_register: Vec<Box<StrategyType>>
}

impl Checker for NaiveChecker {

    fn check(&self, check: &str) -> Vec<ReportCard> {
        self.strategy_register.iter() 
        .map(|s| s.parse(check) )
        .collect()
    }

}

impl NaiveChecker {
    pub fn new() -> Self {
     NaiveChecker {
        strategy_register: vec![Box::new(Base64Strategy {}), Box::new(JsonStrategy {}) ]
     }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use families;
    use std::cmp::Ordering::Equal;

    fn get_ordered_results(input: &str) -> Vec<ReportCard> {
        let chkr = NaiveChecker::new();
        let mut results = chkr.check(input);
        results.sort_by(|b,a| a.sureness.partial_cmp(&b.sureness).unwrap_or(Equal));

        results
    }

    fn sureness_limit(sureness: f32) {
        assert!(sureness > 0.9)
    }

    #[test]
    fn register() {
        let checker = NaiveChecker::new();
        assert_eq!(checker.strategy_register.len(), 2); 

        let jsonStrategyInRegister = checker.strategy_register.iter()
                                                              .map(|s| s.id()) 
                                                              .find(|s| s == &JsonStrategy{}.id())
                                                              .is_some();

        let b64StrategyInRegister = checker.strategy_register.iter()
                                                             .map(|s| s.id()) 
                                                             .find(|s| s == &Base64Strategy{}.id())
                                                             .is_some();

        assert!(jsonStrategyInRegister);
        assert!(b64StrategyInRegister);
    }

    #[test]
    fn check_json() {
        let json = "{\"hello\": \"world\"}";
        let results = get_ordered_results(json);
        assert_eq!(results[0].family, families::Family::JSON);
        sureness_limit(results[0].sureness)
    }

    #[test]
    fn check_b64() {
        let base64 = "aGVsbG8gd29ybGQ=";
        let results = get_ordered_results(base64);
        assert_eq!(results[0].family, families::Family::BASE64);
        sureness_limit(results[0].sureness);
    }

    #[test]
    fn check_garbage() {
        let garbage = "asoidjfas'odifujasd[ofiuasdf";
        let results = get_ordered_results(garbage);
  
        let no_sure_results = results.iter()
                                     .map(|r| r.sureness)
                                     .all(|s| s < 0.1);
        
        assert!(no_sure_results); 
    }

}