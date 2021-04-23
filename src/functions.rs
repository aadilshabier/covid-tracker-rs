use std::collections::hash_map::Keys;
use std::collections::HashMap;

pub fn best_string_match(elem: &str, items: Keys<String, [String; 3]>) -> String {
    let elem_counter = &(as_counter(elem));
    items
        .reduce(|x, y| {
            if hmap_similarity(&as_counter(x), elem_counter)
                > hmap_similarity(&as_counter(y), elem_counter)
            {
                x
            } else {
                y
            }
        })
        .unwrap()
        .to_string()
}

fn as_counter(string: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    string.chars().for_each(|chr| {
        *result.entry(chr).or_default() += 1;
    });

    result
}

fn hmap_similarity(hmap1: &HashMap<char, usize>, hmap2: &HashMap<char, usize>) -> f32 {
    let numerator: usize = hmap1
        .iter()
        .map(|(k, v)| (*v) * (*hmap2.get(k).unwrap_or(&0)))
        .sum();
    let denominator = ((hmap1.values().map(|x| (*x).pow(2)).sum::<usize>()
        * hmap2.values().map(|x| (*x).pow(2)).sum::<usize>()) as f32)
        .sqrt();

    numerator as f32 / denominator
}

// convert to vector containing values like
// "Karnataka" : ("1,232", "1,533", "21")
pub fn reorder_data(cases_vec: Vec<HashMap<String, String>>) -> HashMap<String, [String; 3]> {
    let mut result: HashMap<String, [String; 3]> = HashMap::with_capacity(39);

    for mut cases in cases_vec {
        cases.remove("dateymd");
        cases.remove("date");
        let status = cases.remove("status").unwrap();

        for (state, value) in cases {
            match status.as_str() {
                "Confirmed" => {
                    (*result.entry(unabbrevate(state)).or_default())[0] = format_num(value);
                }
                "Recovered" => {
                    (*result.entry(unabbrevate(state)).or_default())[1] = format_num(value);
                }
                "Deceased" => {
                    (*result.entry(unabbrevate(state)).or_default())[2] = format_num(value);
                }
                _ => {}
            };
        }
    }
    result
}

fn format_num(mut num_string: String) -> String {
    let length = num_string.len();
    for i in (1..length).rev() {
        if (length - i) % 3 == 0 {
            num_string.insert(i, ',');
        }
    }

    num_string
}

fn unabbrevate(abbrev: String) -> String {
    match abbrev.as_str() {
        "an" => "Andaman and Nicobar".to_string(),
        "ap" => "Andhra Pradesh".to_string(),
        "ar" => "Arunachal Pradesh".to_string(),
        "as" => "Assam".to_string(),
        "br" => "Bihar".to_string(),
        "ch" => "Chandigarh".to_string(),
        "ct" => "Chattisgarh".to_string(),
        "dd" => "Daman and Diu".to_string(),
        "dl" => "Delhi".to_string(),
        "dn" => "Dadra and Nagar Haveli".to_string(),
        "ga" => "Goa".to_string(),
        "gj" => "Gujarat".to_string(),
        "hp" => "Himachal Pradesh".to_string(),
        "hr" => "Haryana".to_string(),
        "jh" => "Jharkhand".to_string(),
        "jk" => "Jammu and Kashmir".to_string(),
        "ka" => "Karnataka".to_string(),
        "kl" => "Kerala".to_string(),
        "la" => "Lakshwadeep".to_string(),
        "ld" => "Ladakh".to_string(),
        "mh" => "Maharashtra".to_string(),
        "ml" => "Meghalaya".to_string(),
        "mn" => "Manipur".to_string(),
        "mp" => "Madhya Pradesh".to_string(),
        "mz" => "Mizoram".to_string(),
        "nl" => "Nagaland".to_string(),
        "or" => "Odisha".to_string(),
        "pb" => "Punjab".to_string(),
        "py" => "Puducherry".to_string(),
        "rj" => "Rajasthan".to_string(),
        "sk" => "Sikkim".to_string(),
        "tg" => "Telengana".to_string(),
        "tn" => "Tamil Nadu".to_string(),
        "tr" => "Tripura".to_string(),
        "tt" => "Total".to_string(),
        "un" => "Unknown".to_string(),
        "up" => "Uttar Pradesh".to_string(),
        "ut" => "Uttarakhand".to_string(),
        "wb" => "West Bengal".to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_num1() {
        assert_eq!("12,322,333".to_string(), format_num("12322333".to_string()));
    }
    #[test]
    fn test_format_num2() {
        assert_eq!("2,333".to_string(), format_num("2333".to_string()));
    }
    #[test]
    fn test_format_num3() {
        assert_eq!("333".to_string(), format_num("333".to_string()));
    }
    #[test]
    fn test_format_num4() {
        assert_eq!("322,333".to_string(), format_num("322333".to_string()));
    }
}
