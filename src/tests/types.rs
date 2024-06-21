use crate::language::term::get;
use crate::parse::parse_term;
use crate::ToArcTerm;
#[test]
fn test1() {
    let t = parse_term(
        r##"
         { x: y }
        "##,
    );
    assert_eq!(t, get("y").field("x").to_arc_ok());
}

#[test]
fn test2() {
    let t = parse_term(
        r##"
         { x: int, y : int } -> int 
        "##,
    );
    assert_eq!(
        t,
        get("int")
            .field("x")
            .and(get("int").field("y"))
            .function(get("int"))
            .to_arc_ok()
    );

    println!("{t:?}");
}
