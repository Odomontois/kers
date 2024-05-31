use crate::parse::parse_term;

#[test]
fn test1() {
    let t = parse_term(
        r##"
         {
            x: y
         }
        "##,
    ).unwrap();
    println!("{t:?}");
}
