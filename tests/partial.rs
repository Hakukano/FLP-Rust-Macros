use flp_rust_macros::Partial;
use utoipa::ToSchema;

#[derive(Partial)]
#[partial(structs(Ab, Ac, Bc), derives(Clone, ToSchema))]
pub struct Details {
    #[partial(included(Ab, Ac))]
    pub a: u8,
    #[partial(included(Ab, Bc))]
    b: &'static str,
    #[partial(included(Bc, Ac))]
    pub c: Vec<&'static str>,
}

#[test]
fn generated_all() {
    let details = Details {
        a: 1,
        b: "b",
        c: vec!["c"],
    };

    let ab = Ab { a: 1, b: "b" };
    let bc = Bc {
        b: "b",
        c: vec!["c"],
    };
    let ac = Ac { a: 1, c: vec!["c"] };

    assert_eq!(details.a, 1);
    assert_eq!(details.b, "b");
    assert_eq!(details.c, vec!["c"]);
    assert_eq!(ab.a, 1);
    assert_eq!(ab.b, "b");
    assert_eq!(ab.clone().a, ab.a);
    assert_eq!(Ab::schema().0, "Ab");
    assert_eq!(bc.b, "b");
    assert_eq!(bc.c, vec!["c"]);
    assert_eq!(Bc::schema().0, "Bc");
    assert_eq!(ac.a, 1);
    assert_eq!(ac.c, vec!["c"]);
    assert_eq!(Ac::schema().0, "Ac");
}
