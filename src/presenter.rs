use ReportCard;
use serde_json::{self, Value};
use families::Family;
use base64;

fn get_header(t: &str) -> String {
    format!("Scrutey thinks this is {}:\n\n", t)
}

fn get_response(friendly_name: &str, body: &str) -> String {
    format!("{}{}", get_header(friendly_name), body)
}

pub fn present(input: &str, report_card: &Option<ReportCard>) -> String {

    fn nonsense(input: &str) -> String {
        get_response("nonsense", input)
    }

    if let Some(ref card) = *report_card {

        match card.family {
            Family::JSON => {
                let p: Option<Value> = serde_json::from_str(input).ok();
                let pretty_json: String = match p {
                    Some(s) => serde_json::to_string_pretty(&s).unwrap_or(input.to_owned()),
                    _ => input.to_owned(),
                };
                get_response(&card.type_friendly_name, &pretty_json)
            }
            Family::BASE64 => {
                let p = base64::decode(input.trim()).ok();

                if let Some(ref decoded) = p {
                    get_response(&card.type_friendly_name,
                                 &String::from_utf8_lossy(decoded).into_owned())
                } else {
                    get_response(&card.type_friendly_name, input)
                }
            }
            Family::UNSTRUCTURED => nonsense(input),
        }

    } else {
        nonsense(input)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use report_card::ReportCard;
    use families;
    use presenter;
    use base64;

    #[test]
    fn test_get_header() {
        assert_eq!("Scrutey thinks this is garbage:\n\n", get_header("garbage"));
        assert_eq!("Scrutey thinks this is hardcore pornography:\n\n",
                   get_header("hardcore pornography"));
    }

    #[test]
    fn none_is_garbage() {
        let given = "hello world";
        let result = present(given, &None);

        let expected = format!("{}{}", get_header("nonsense"), given);

        assert_eq!(expected, result);
    }

    #[test]
    fn json_is_pretty_printed_when_possible() {
        let given_format = Some(ReportCard::new(1.0, "JSON".into(), families::Family::JSON));

        let sample_json = json!({"hello": "world", "second_line": "second line go"});

        let sample_json_pretty = serde_json::to_string_pretty(&sample_json)
            .expect("couldn't build test data");
        let sample_json_compact = serde_json::to_string(&sample_json)
            .expect("couldn't build test data");

        assert_ne!(sample_json_pretty, sample_json_compact);

        let result = present(&sample_json_compact, &given_format);
        let expected = format!("{}{}", get_header("JSON"), "SOME HIGHLIGHTED PRETTY PRINTED JSON SHOULD BE HERE");//sample_json_pretty);

        assert_eq!(expected, result);
    }

    #[test]
    fn json_is_printed_as_is_if_errored() {
        let given_format = Some(ReportCard::new(1.0, "BROKENJSON".into(), families::Family::JSON));
        let sample_json = "{";
        let result = present(&sample_json, &given_format);
        assert_eq!(format!("{}{}", get_header("BROKENJSON"), sample_json), result)
    }

    #[test]
    fn good_base64_is_decoded() {
        let given_format = Some(ReportCard::new(1.0, "Base64".into(), families::Family::BASE64));
        let sample_base64 = " aGVsbG8gd29ybGQ=  ";
        let result = present(sample_base64, &given_format);

        let expected = format!("{}{}", get_header("Base64"), "hello world");
        assert_eq!(expected, result);
    }

    #[test]
    fn bad_base64_is_printed_as_is() {
        let given_format = Some(ReportCard::new(1.0, "Base64".into(), families::Family::BASE64));
        let sample_base64 = "!!!";
        let result = present(sample_base64, &given_format);

        let expected = format!("{}{}", get_header("Base64"), "!!!");
        assert_eq!(expected, result);
    }

    #[test]
    fn unstructed_prints_is_garbage() {
        let given_format =
            Some(ReportCard::new(1.0, "unstructured".into(), families::Family::UNSTRUCTURED));
        let given = "hello world";
        let result = present(given, &given_format);

        let expected = format!("{}{}", get_header("nonsense"), given);

        assert_eq!(expected, result);
    }

    // and if there are errors in the reportcard?
}