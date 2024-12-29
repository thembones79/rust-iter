fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{:#?}", element);
    }
}
fn print_elements2(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}
fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // let mut colors_iter = colors.iter();
    //
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());

    print_elements(&colors);
    print_elements2(&colors);
}
