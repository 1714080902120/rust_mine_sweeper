#[test]
fn test_macro() {
    use my_macro::{ HelloMacro };
    trait HelloMacro {
        fn hello_macro() -> String {
            String::from("666")
        }
    }

    #[derive(HelloMacro)]
    struct A;

    assert_eq!(A::hello_macro(), "A".to_string());
}