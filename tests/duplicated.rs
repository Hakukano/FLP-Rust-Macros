use flp_rust_macros::Duplicated;

#[derive(Clone)]
struct Old {
    a: u8,
    b: String,
    c: Vec<String>,
}

#[derive(Clone, Duplicated)]
#[duplicated(target = Old)]
struct New {
    a: u8,
    b: String,
    c: Vec<String>,
}

#[test]
fn from() {
    let old = Old {
        a: 1,
        b: "b".to_string(),
        c: vec!["c".to_string()],
    };

    let new = New::from(old.clone());

    assert_eq!(new.a, old.a);
    assert_eq!(new.b, old.b);
    assert_eq!(new.c, old.c);
}

#[test]
fn to() {
    let new = New {
        a: 1,
        b: "b".to_string(),
        c: vec!["c".to_string()],
    };

    let old: Old = new.clone().into();

    assert_eq!(old.a, new.a);
    assert_eq!(old.b, new.b);
    assert_eq!(old.c, new.c);
}
