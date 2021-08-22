use std::cmp::min;
use std::collections::hash_map::Keys;
use std::collections::HashMap;

pub fn best_string_match<T>(elem: &str, items: Keys<String, T>) -> String {
    items
        .reduce(|x, y| {
            if edit_distance(x, elem) < edit_distance(y, elem) {
                x
            } else {
                y
            }
        })
        .unwrap()
        .to_string()
}

pub fn edit_distance(str_a: &str, str_b: &str) -> u32 {
    // distances[i][j] = distance between a[..i] and b[..j]
    let mut distances = vec![vec![0; str_b.len() + 1]; str_a.len() + 1];
    // Initialize cases in which one string is empty
    for j in 0..=str_b.len() {
        distances[0][j] = j as u32;
    }
    for (i, item) in distances.iter_mut().enumerate() {
        item[0] = i as u32;
    }
    for i in 1..=str_a.len() {
        for j in 1..=str_b.len() {
            distances[i][j] = min(distances[i - 1][j] + 1, distances[i][j - 1] + 1);
            if str_a.as_bytes()[i - 1] == str_b.as_bytes()[j - 1] {
                distances[i][j] = min(distances[i][j], distances[i - 1][j - 1]);
            } else {
                distances[i][j] = min(distances[i][j], distances[i - 1][j - 1] + 1);
            }
        }
    }
    distances[str_a.len()][str_b.len()]
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
            (*result.entry(unabbrevate(state)).or_default())[match status.as_str() {
                "Confirmed" => 0,
                "Recovered" => 1,
                "Deceased" => 2,
                _ => panic!("This shouldn't have happened"),
            }] = format_num_string(value);
        }
    }
    result
}

#[inline]
fn format_num_string(mut num_string: String) -> String {
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
        "an" => "Andaman and Nicobar",
        "ap" => "Andhra Pradesh",
        "ar" => "Arunachal Pradesh",
        "as" => "Assam",
        "br" => "Bihar",
        "ch" => "Chandigarh",
        "ct" => "Chattisgarh",
        "dd" => "Daman and Diu",
        "dl" => "Delhi",
        "dn" => "Dadra and Nagar Haveli",
        "ga" => "Goa",
        "gj" => "Gujarat",
        "hp" => "Himachal Pradesh",
        "hr" => "Haryana",
        "jh" => "Jharkhand",
        "jk" => "Jammu and Kashmir",
        "ka" => "Karnataka",
        "kl" => "Kerala",
        "la" => "Lakshwadeep",
        "ld" => "Ladakh",
        "mh" => "Maharashtra",
        "ml" => "Meghalaya",
        "mn" => "Manipur",
        "mp" => "Madhya Pradesh",
        "mz" => "Mizoram",
        "nl" => "Nagaland",
        "or" => "Odisha",
        "pb" => "Punjab",
        "py" => "Puducherry",
        "rj" => "Rajasthan",
        "sk" => "Sikkim",
        "tg" => "Telengana",
        "tn" => "Tamil Nadu",
        "tr" => "Tripura",
        "tt" => "Total",
        "un" => "Unknown",
        "up" => "Uttar Pradesh",
        "ut" => "Uttarakhand",
        "wb" => "West Bengal",
        _ => "",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_num1() {
        assert_eq!(
            "12,322,333".to_string(),
            format_num_string("12322333".to_string())
        );
    }
    #[test]
    fn test_format_num2() {
        assert_eq!("2,333".to_string(), format_num_string("2333".to_string()));
    }
    #[test]
    fn test_format_num3() {
        assert_eq!("333".to_string(), format_num_string("333".to_string()));
    }
    #[test]
    fn test_format_num4() {
        assert_eq!(
            "322,333".to_string(),
            format_num_string("322333".to_string())
        );
    }

    #[test]
    fn equal_strings() {
        assert_eq!(0, edit_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, edit_distance("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, edit_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, edit_distance("My Cat", "My Case"));
        assert_eq!(7, edit_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, edit_distance("Test_Case_#3", "Case #3"));
    }
}
