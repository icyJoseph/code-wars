use std::collections::HashMap;

fn parse_books(books: Vec<&str>) -> Vec<(String, u64)> {
    let mut result: Vec<(String, u64)> = Vec::new();
    for book in books.iter() {
        let split: Vec<String> = book.split(" ").map(|x| x.to_string()).collect();
        let (code, points) = (split[0].to_string(), split[1].parse::<u64>().unwrap());
        result.push((code, points));
    }
    return result;
}

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    // your code
    if list_art.len() == 0 || list_cat.len() == 0 {
        return "".to_string();
    }

    let mut stock: HashMap<String, u64> = HashMap::new();

    let parsed_books = parse_books(list_art);

    for cat in &list_cat {
        stock.insert(cat.to_string(), 0);
    }

    for parsed_book in parsed_books {
        let (book, points) = parsed_book;
        let category = match book.get(0..1) {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };

        let current = match stock.get(&category) {
            Some(val) => *val,
            None => 0,
        };

        stock.insert(category, points + current);
    }

    let mut result = Vec::new();

    for cat in &list_cat {
        let value = match stock.get(*cat) {
            Some(v) => *v,
            None => 0,
        };

        let print = "(".to_string() + cat + " : " + &value.to_string() + ")";
        result.push(print);
    }

    return result.join(" - ");
}

fn main() {
    let books = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let categories = vec!["A", "B", "C", "D"];
    let result = stock_list(books, categories);

    println!("{}", result);
}

#[test]
fn test_stock_list() {
    let books = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let categories = vec!["A", "B"];
    assert_eq!(stock_list(books, categories), "(A : 200) - (B : 1140)");
}

#[test]
fn test_empty_lists() {
    let mut books = vec![];
    let mut categories = vec!["A", "B"];

    assert_eq!(stock_list(books, categories), "");

    books = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
    categories = vec![];

    assert_eq!(stock_list(books, categories), "");
}

#[test]
fn test_category_without_books() {
    let books = vec!["CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let categories = vec!["A", "B", "C", "D"];

    assert_eq!(
        stock_list(books, categories),
        "(A : 0) - (B : 1140) - (C : 500) - (D : 600)"
    );
}
