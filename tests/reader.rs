use harp::{nodes::{Node, functions::Exp}, reader::read};

#[test]
fn read_positive_numbers() {
    let n = read("1.2 412 3.1415926").unwrap();
    println!("{}", n.to_string());
    match n {
        harp::nodes::Node::Do(xs) => {
            assert_eq!(xs[0], Node::n(1.2));
            assert_eq!(xs[1], Node::n(412.0));
            assert_eq!(xs[2], Node::n(3.1415926));
        }
        _ => panic!(),
    }
}

// #[test]
// fn read_expressions() {
//     let exp = read("(+ 1 2 (/ 1 2) 5)").unwrap();
//     match exp {
//         Node::Do(xs) => {
//             match &xs[0] {
//                 Node::Exp(Exp::Call(call)) => {
//                 },
//                 _ => panic!()
//             }
//         }
//         _ => panic!(),
//     }
// }
