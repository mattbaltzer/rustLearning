fn main() {
    let mut city_names:Vec<&str> = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    // match needed to be here in order to return a string. Calling pop() without it wouldn't return anything
    // Rust wanted us to use enums in order to prevent an issue from occuring
    let last_city = match city_names.pop() { 
            Some(inner_value) => { inner_value }
            None => { "" }
        };

    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

        city_names.push(last_city);

    println!("Here is the full list of cities:");

    for name in city_names.iter() {
        println!("{}",name);
    }
}