fn work(v: &str) {
    println!("work on v: {}", v);
}

fn for_each(v: &Vec<&str>) {
    for i in 0..v.len() {
        work(&v[i]);
    }
}

fn main() {
    let mut vec = vec!["v1", "v2", "v3"];
    vec.push("v4");
    vec.push("v5");
    for_each(&vec);
}