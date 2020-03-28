#![feature(c_variadic)]

pub unsafe extern "C" fn varargs(_: u32, mut args: ...) -> (u32, u32) {
    let before = args.arg::<u32>();

    args.as_va_list().arg::<u32>();

    let after = args.arg::<u32>();

    (before, after)
}
