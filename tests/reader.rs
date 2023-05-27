use harp::reader::read;

#[test]
fn read_numbers() {
    let n = read("3.14159 -312.0 420");
    println!("{:?}", n)
}
