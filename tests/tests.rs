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
        let (before, after) = unsafe {
            varargs::varargs(
                0,
                1,
                2,
                3
            )
        };

        assert_eq!(before, 1);
        assert_eq!(after, 3);
    }
}
