#[test]
fn it_works() {
    crate::jse!(b"{ console.log(\"hello\") }\x00");

    assert_eq!(2 + 2, 4);
}
