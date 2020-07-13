mod tdjson;

fn main() {
    let tdlib = tdjson::Tdlib::new();
    println!("{}", tdlib.receive(16.0));
    tdlib.destroy();
}
