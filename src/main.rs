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
fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}
fn main() {
    let mut colors = vec![
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
    shorten_strings(&mut colors);
    print_elements2(&colors);
}
