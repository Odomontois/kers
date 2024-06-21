use crate::parse::parse_term;
use crate::{Term, ToArcTerm, Type};

#[test]
fn test1() {
    let t = parse_term(
        r##"
         { x: y }
        "##,
    );
    assert_eq!(
        t,
        Type::Field {
            name: "x".into(),
            typ: Term::get("y").to_arc_term()
        }
        .to_arc_ok()
    );
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
        Type::Function {
            dom: Type::Field {
                name: "x".into(),
                typ: Term::get("int").to_arc_term()
            }
            .to_arc_term(),
            codom: Term::get("int").to_arc_term()
        }
        .to_arc_ok()
    );

    println!("{t:?}");
}
