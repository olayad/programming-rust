fn main() {
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages{
        println!("l:{}, l.len:{}, : {}", l, l.len(),
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            })
    }
}
