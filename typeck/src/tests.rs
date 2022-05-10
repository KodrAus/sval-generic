use crate::*;
use std::collections::HashMap;

#[test]
fn typecheck_simple() {
    assert_eq!(&Type::unit(), Context::new().eval(()));

    assert_eq!(&Type::bool(), Context::new().eval(true));

    assert_eq!(&Type::u8(), Context::new().eval(1u8));
    assert_eq!(&Type::u16(), Context::new().eval(1u16));
    assert_eq!(&Type::u32(), Context::new().eval(1u32));
    assert_eq!(&Type::u64(), Context::new().eval(1u64));
    assert_eq!(&Type::u128(), Context::new().eval(1u128));

    assert_eq!(&Type::i8(), Context::new().eval(-1i8));
    assert_eq!(&Type::i16(), Context::new().eval(-1i16));
    assert_eq!(&Type::i32(), Context::new().eval(-1i32));
    assert_eq!(&Type::i64(), Context::new().eval(-1i64));
    assert_eq!(&Type::i128(), Context::new().eval(-1i128));

    assert_eq!(&Type::f32(), Context::new().eval(1f32));
    assert_eq!(&Type::f64(), Context::new().eval(1f64));

    assert_eq!(&Type::text(), Context::new().eval("Some text"));
}

#[test]
fn typecheck_empty_map() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval(HashMap::<String, ()>::new());

    assert!(!ty.is_complete());

    assert_eq!(&Type::empty_map(), ty);
}

#[test]
fn typecheck_simple_map() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval({
        let mut map = HashMap::new();
        map.insert("a", ());
        map.insert("b", ());
        map
    });

    assert!(ty.is_complete());

    assert_eq!(&Type::map(Type::text(), Type::unit()), ty);
}

#[test]
fn typecheck_nested_map() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval({
        let mut map = HashMap::new();
        map.insert("a", {
            let mut map = HashMap::new();
            map.insert("aa", ());
            map
        });
        map.insert("b", {
            let mut map = HashMap::new();
            map.insert("ba", ());
            map
        });
        map
    });

    assert!(ty.is_complete());

    assert_eq!(
        &Type::map(Type::text(), Type::map(Type::text(), Type::unit())),
        ty
    );
}

#[test]
fn typecheck_empty_seq() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval(&[] as &[()]);

    assert!(!ty.is_complete());

    assert_eq!(&Type::empty_seq(), ty);
}

#[test]
fn typecheck_simple_seq() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval(&[(), ()] as &[()]);

    assert!(ty.is_complete());

    assert_eq!(&Type::seq(Type::unit()), ty);
}

#[test]
fn typecheck_nested_seq() {
    let mut ctxt = Context::new();

    let ty = ctxt.eval(&[&[(), ()] as &[()], &[(), ()]] as &[&[()]]);

    assert!(ty.is_complete());

    assert_eq!(&Type::seq(Type::seq(Type::unit())), ty);
}
