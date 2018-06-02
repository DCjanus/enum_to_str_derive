#[macro_use]
extern crate enum_to_str_derive;

#[derive(EnumToStr)]
enum Foo {
    Foo,
    #[ETS(value = r#"Can you please tell me what does "foo" mean"#)]
    Question,
}

#[test]
fn basic_test() {
    debug_assert_eq!("Foo", Foo::Foo.enum_to_str());
    debug_assert_eq!(
        r#"Can you please tell me what does "foo" mean"#,
        Foo::Question.enum_to_str()
    );
}
