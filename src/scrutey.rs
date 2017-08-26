use checker::{Checker, NaiveChecker};
use ReportCard;
use std::cmp::Ordering::Equal;
use presenter;

// main app, with command line args collected 

pub fn scrutinize<T: Checker>(input: Result<String,String>, checker: &T) -> Result<String,String> {

    let raw_input = try!(input);
    let string_to_test = raw_input.trim();

    // run all the checks

    let mut report_cards = checker.check(&string_to_test);

    if report_cards.len() < 1 {
        return Err("internal error; checker returned no results".into());
    }

    report_cards.sort_by(|b,a| a.sureness.partial_cmp(&b.sureness).unwrap_or(Equal));

    // pick the top result
    let top = Some(report_cards[0].clone());

    // run the reportcard through the presenter
    let formatted_result = presenter::present(&string_to_test, &top);
      
    // return the result
    Ok(formatted_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use report_card::ReportCard;
    use families;
    use presenter;

    struct MockChecker {}
    impl Checker for MockChecker {
        fn check(&self, input: &str) -> Vec<ReportCard> {
            vec![
                ReportCard::new(0.0, "base64".into(), families::Family::BASE64), // base64
                ReportCard::new(1.0, "json".into(), families::Family::JSON), // json
            ]
        }
    }   
    impl MockChecker {
        pub fn new() -> Self { MockChecker {} }
    } 

    #[test]
    fn input_errors_rejected() {
        let chk = MockChecker::new();
        assert_eq!(Err("error message".to_owned()), scrutinize(Err("error message".to_owned()), &chk));
    }

    #[test]
    fn return_error_if_no_results() {
        struct EmptyChecker {}
        impl Checker for EmptyChecker {
            fn check(&self, input: &str) -> Vec<ReportCard> {
                vec![]
            }
        }

        let bad_checker = EmptyChecker {};

        assert_eq!(Err("internal error; checker returned no results".to_owned()), scrutinize(Ok("any data".to_owned()), &bad_checker))
    }

    #[test]
    fn top_card_presented() {
        let chk = MockChecker::new();
        let json = "{\"hello\": \"world\"}";
        let sample = Some(ReportCard::new(1.0, "json".into(), families::Family::JSON));
        
        let presented = presenter::present(json, &sample);
        let result = scrutinize(Ok(json.into()), &chk).expect("bad test config");
                      
        assert_eq!(presented, result);    
    }
    
}