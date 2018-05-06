extern crate amath;

fn main() {
    let input = "f(7)";

    let mut context = amath::Context::new();

    context.add_function("f".to_owned(), |args, context| {
        if args.len() != 1 {
            Err(format!("Number of arguments must be 1, not {}", args.len()))
        }
        else {
            args[0].multiply(&amath::Value::Int(2), context)
                .map_err(|e| std::error::Error::description(&e).to_owned())
        }
    });

    println!("{:?}", amath::parse(&input).unwrap().eval(&context));
}
