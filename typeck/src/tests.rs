use crate::*;
use std::collections::HashMap;

#[test]
fn typecheck_unit() {
    let mut ctxt = Context::new();

    assert_eq!(&Type::unit(), ctxt.eval(()));
}

#[test]
fn typecheck_empty_map() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval(HashMap::<String, i32>::new());

    assert!(!ty.is_complete());

    assert_eq!(&Type::empty_map(), ty);
}

#[test]
fn typecheck_simple_map() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval({
        let mut map = HashMap::new();
        map.insert("a", 1i32);
        map.insert("b", 2);
        map
    });

    assert!(ty.is_complete());

    assert_eq!(&Type::map(Type::text(), Type::i32()), ty);
}

#[test]
fn typecheck_nested_map() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval({
        let mut map = HashMap::new();
        map.insert("a", {
            let mut map = HashMap::new();
            map.insert("aa", 42i32);
            map
        });
        map.insert("b", {
            let mut map = HashMap::new();
            map.insert("ba", 42i32);
            map
        });
        map
    });

    assert!(ty.is_complete());

    assert_eq!(
        &Type::map(Type::text(), Type::map(Type::text(), Type::i32())),
        ty
    );
}
