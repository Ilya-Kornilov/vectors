fn main() {
    println!("Vectors");

    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let s = &v[0];
    println!(" > 0: {s}");
    // let s = v.remove(0);
    let s = v.get(0);
    if let Some(element) = s {
        println!(" > 0: {element}");
    }

    for string in &mut v {
        string.push_str("!");
    }

    for string in &v {
        println!(">_{string}");
    }

    let v2 = vec![1, 2, 3];
    let mut v3 = vec![];
    for int in v2 {
        v3.push(int);
    }
    for int in v3 {
        print!("{int} ")
    }
    println!("");
}
