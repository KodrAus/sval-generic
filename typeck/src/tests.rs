use crate::*;
use std::collections::HashMap;

#[test]
fn typecheck_simple() {
    assert_eq!(Type::unit(), type_of_val(()));

    assert_eq!(Type::bool(), type_of_val(true));

    assert_eq!(Type::u8(), type_of_val(1u8));
    assert_eq!(Type::u16(), type_of_val(1u16));
    assert_eq!(Type::u32(), type_of_val(1u32));
    assert_eq!(Type::u64(), type_of_val(1u64));
    assert_eq!(Type::u128(), type_of_val(1u128));

    assert_eq!(Type::i8(), type_of_val(-1i8));
    assert_eq!(Type::i16(), type_of_val(-1i16));
    assert_eq!(Type::i32(), type_of_val(-1i32));
    assert_eq!(Type::i64(), type_of_val(-1i64));
    assert_eq!(Type::i128(), type_of_val(-1i128));

    assert_eq!(Type::f32(), type_of_val(1f32));
    assert_eq!(Type::f64(), type_of_val(1f64));

    assert_eq!(Type::text(), type_of_val("Some text"));
}

#[test]
fn typecheck_empty_map() {
    let ty = type_of_val(HashMap::<String, ()>::new());

    assert!(!ty.is_complete());

    assert_eq!(Type::empty_map(), ty);
}

#[test]
fn typecheck_simple_map() {
    let ty = type_of_val({
        let mut map = HashMap::new();
        map.insert("a", ());
        map.insert("b", ());
        map
    });

    assert!(ty.is_complete());

    assert_eq!(Type::map(Type::text(), Type::unit()), ty);
}

#[test]
fn typecheck_nested_map() {
    let ty = type_of_val({
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
        map.insert("c", HashMap::new());
        map
    });

    assert!(ty.is_complete());

    assert_eq!(
        Type::map(Type::text(), Type::map(Type::text(), Type::unit())),
        ty
    );
}

#[test]
fn extend_empty_map() {
    let map = {
        let mut map = HashMap::new();
        map.insert("a", ());
        map.insert("b", ());
        map
    };

    let mut ctxt = Context::new();

    ctxt.eval(HashMap::<String, ()>::new());
    let extended_ty = ctxt.eval(&map);

    assert_eq!(&type_of_val(&map), extended_ty);
}

#[test]
fn typecheck_empty_seq() {
    let ty = type_of_val(&[] as &[()]);

    assert!(!ty.is_complete());

    assert_eq!(Type::empty_seq(), ty);
}

#[test]
fn typecheck_simple_seq() {
    let ty = type_of_val(&[(), ()] as &[()]);

    assert!(ty.is_complete());

    assert_eq!(Type::seq(Type::unit()), ty);
}

#[test]
fn typecheck_nested_seq() {
    let ty = type_of_val(&[&[(), ()] as &[()], &[]] as &[&[()]]);

    assert!(ty.is_complete());

    assert_eq!(Type::seq(Type::seq(Type::unit())), ty);
}

#[test]
fn extend_empty_seq() {
    let mut ctxt = Context::new();

    ctxt.eval(&[] as &[()]);
    let extended_ty = ctxt.eval(&[(), ()] as &[()]);

    assert_eq!(&type_of_val(&[(), ()] as &[()]), extended_ty);
}

#[test]
fn typecheck_dynamic() {
    let ty = type_of_val(&42i32 as &dyn sval_dynamic::Value);

    assert!(ty.is_complete());

    assert_eq!(Type::dynamic(), ty);
}

#[test]
fn extend_dynamic() {
    let mut ctxt = Context::new();

    let ty_from_i32 = ctxt.eval(&42i32 as &dyn sval_dynamic::Value);

    assert_eq!(&Type::dynamic(), ty_from_i32);

    let ty_from_bool = ctxt.eval(&true as &dyn sval_dynamic::Value);

    assert_eq!(&Type::dynamic(), ty_from_bool);
}

#[test]
fn typecheck_record() {
    use sval_derive::*;

    #[derive(Value)]
    struct Record {
        a: i32,
        b: bool,
    }

    let ty = type_of_val(&Record { a: 42, b: true });

    assert_eq!(
        Type::record(
            None,
            Some(Label::new("Record")),
            [
                (Label::new("a"), Type::i32()),
                (Label::new("b"), Type::bool()),
            ]
        ),
        ty
    );
}

#[test]
fn typecheck_record_id() {
    struct Record {
        a: i32,
        b: bool,
    }

    impl Record {
        const ID: sval::Id =
            sval::Id::new(uuid::uuid!("ab12bf2d-5b74-4451-9c2d-33e3e5be6212").into_bytes());
    }

    impl sval::Value for Record {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            stream.record_begin(
                sval::Tag::Identified(Self::ID, Some(sval::Label::new("Record"))),
                Some(2),
            )?;

            stream.record_value_begin(sval::Label::new("a"))?;
            sval::Value::stream(&self.a, stream)?;
            stream.record_value_end(sval::Label::new("a"))?;

            stream.record_value_begin(sval::Label::new("b"))?;
            sval::Value::stream(&self.b, stream)?;
            stream.record_value_end(sval::Label::new("b"))?;

            stream.record_end(sval::Tag::Identified(
                Self::ID,
                Some(sval::Label::new("Record")),
            ))
        }
    }

    let mut ctxt = Context::new();

    ctxt.eval(&Record { a: 42, b: true });

    println!("{:?}", ctxt);
}
