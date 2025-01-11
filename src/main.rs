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
fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}
fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
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

    // print_elements(&colors);
    // let uppercased = to_uppercase(&colors);
    // println!("{:#?}", uppercased);
    // shorten_strings(&mut colors);
    // print_elements2(&colors);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!(
    //     "Colors: {:#?}, Destination: {:#?}",
    //     &destination, destination
    // );

    // let exploded = explode(&colors);
    //
    // println!("{:#?}", exploded);


    let found_color = find_color_or(&colors,"asdf","Orange");
    println!("{:#?}", found_color);
}
