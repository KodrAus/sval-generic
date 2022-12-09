#![feature(test)]
extern crate test;

#[bench]
fn tag_cmp_ne(b: &mut test::Bencher) {
    let tag_a = sval::Tag::new("sval_tag_to_cmp_1");
    let tag_b = sval::Tag::new("sval_tag_to_cmp_2");

    b.iter(|| tag_a == tag_b)
}

#[bench]
fn tag_cmp_eq(b: &mut test::Bencher) {
    let tag_a = sval::Tag::new("sval_tag_to_cmp_1");
    let tag_b = sval::Tag::new("sval_tag_to_cmp_1");

    b.iter(|| tag_a == tag_b)
}

#[bench]
fn tag_find_1_present(b: &mut test::Bencher) {
    let tags = &[sval::Tag::new("sval_tag_to_cmp_1")];

    let to_find = sval::Tag::new("sval_tag_to_cmp_1");

    b.iter(|| tags.contains(&to_find));
}

#[bench]
fn tag_find_3_present(b: &mut test::Bencher) {
    let tags = &[
        sval::Tag::new("sval_tag_to_cmp_1"),
        sval::Tag::new("sval_tag_to_cmp_2"),
        sval::Tag::new("sval_tag_to_cmp_3"),
    ];

    let to_find = sval::Tag::new("sval_tag_to_cmp_3");

    b.iter(|| tags.contains(&to_find));
}

#[bench]
fn tag_find_1_missing(b: &mut test::Bencher) {
    let tags = &[sval::Tag::new("sval_tag_to_cmp_1")];

    let to_find = sval::Tag::new("sval_tag_to_cmp_2");

    b.iter(|| tags.contains(&to_find));
}

#[bench]
fn tag_find_3_missing(b: &mut test::Bencher) {
    let tags = &[
        sval::Tag::new("sval_tag_to_cmp_1"),
        sval::Tag::new("sval_tag_to_cmp_2"),
        sval::Tag::new("sval_tag_to_cmp_3"),
    ];

    let to_find = sval::Tag::new("sval_tag_to_cmp_4");

    b.iter(|| tags.contains(&to_find));
}
