use regex::Regex;

pub fn test() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter("aa,bb cc") {
        println!("{:?}", caps.get(1).unwrap().as_str());
    }
}