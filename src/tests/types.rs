use crate::language::term::get;
use crate::language::term::PrimType::{Long, Text};
use crate::parse::parse_term;
use crate::ToBoxTerm;
#[test]
fn test1() {
    let t = parse_term(
        r##"
         { x: y }
        "##,
    );
    assert_eq!(t, get("y").field("x").to_box_ok());
}

#[test]
fn test2() {
    let t = parse_term(
        r##"
         { x: #int, y : #int } -> #text
        "##,
    );
    assert_eq!(
        t,
        Long.field("x")
            .and(Long.field("y"))
            .function(Text)
            .to_box_ok()
    );

    println!("{t:?}");
}
