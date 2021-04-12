extern "C" {
    pub fn emscripten_asm_const_int(
        code: *const u8,
        sigPtr: *const u8,
        argBuf: *const u8,
    ) -> *mut u8;
}
