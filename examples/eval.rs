extern crate amath;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let input = args.join(" ");

    println!("{:?}", amath::parse(&input).unwrap().eval(&Default::default()));
}
