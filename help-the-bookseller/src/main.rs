fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len() == 0 || list_cat.len() == 0 {
        return "".to_string();
    }

    let results: Vec<String> = list_cat
        .iter()
        .map(|category| {
            let category_points = &list_art
                .iter()
                .filter(|book| book.starts_with(category))
                .fold(0, |acc, book| {
                    let split: Vec<String> = book.split(" ").map(|x| x.to_string()).collect();
                    let points = split[1].parse::<u64>().unwrap();
                    return acc + points;
                });

            return format!("({} : {})", category, category_points);
        })
        .collect();

    return results.join(" - ");
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
