extern "C" {
    pub fn emscripten_asm_const_int(
        code: *const u8,
        sigPtr: *const u8,
        argBuf: *const u8,
    ) -> *mut u8;
}

#[test]
fn it_works() {
    static SNIPPET: [u8; 25] = *b"{ console.log(\"hello\") }\x00";
    const EMPTY_SIG: &'static [u8] = b"\x00";

    unsafe {
        emscripten_asm_const_int(
            SNIPPET.as_ptr() as *const _ as *const u8,
            EMPTY_SIG.as_ptr() as *const u8,
            std::ptr::null() as *const u8,
        )
    };

    assert_eq!(2 + 2, 4);
}
