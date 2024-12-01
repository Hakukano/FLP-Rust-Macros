use flp_rust_macros::Partial;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Partial)]
#[partial(
    structs(Ab, Ac, Bc),
    metas(derive(Clone, Deserialize, Serialize), serde(rename_all = "UPPERCASE"))
)]
pub struct Details {
    #[partial(included(Ab, Ac))]
    pub a: u8,
    #[partial(included(Ab, Bc))]
    b: String,
    #[partial(included(Bc, Ac), metas(serde(rename = "cs")))]
    pub c: Vec<String>,
}

#[test]
fn generated_all() {
    let details = Details {
        a: 1,
        b: "b".to_string(),
        c: vec!["c".to_string()],
    };

    let ab = Ab {
        a: 1,
        b: "b".to_string(),
    };
    let bc = Bc {
        b: "b".to_string(),
        c: vec!["c".to_string()],
    };
    let ac = Ac {
        a: 1,
        c: vec!["c".to_string()],
    };

    assert_eq!(details.a, 1);
    assert_eq!(details.b, "b");
    assert_eq!(details.c, vec!["c"]);
    assert_eq!(ab.a, 1);
    assert_eq!(ab.b, "b");
    assert_eq!(ab.clone().a, ab.a);
    assert_eq!(serde_json::to_value(ab).unwrap(), json!({"A": 1, "B": "b"}));
    assert_eq!(bc.b, "b");
    assert_eq!(bc.c, vec!["c"]);
    assert_eq!(
        serde_json::to_value(bc).unwrap(),
        json!({"B": "b", "cs": ["c"]})
    );
    assert_eq!(ac.a, 1);
    assert_eq!(ac.c, vec!["c"]);
    assert_eq!(
        serde_json::to_value(ac).unwrap(),
        json!({"A": 1, "cs": ["c"]})
    );
}
