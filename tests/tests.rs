macro_rules! test {
    ($item:item) => {
        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        $item

        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        $item
    }
}

test! {
    fn varargs() {
        let args = unsafe {
            varargs::varargs(
                0,
                1,
                2,
                3
            )
        };

        assert_eq!(args, [0, 1, 2, 3]);
    }
}
