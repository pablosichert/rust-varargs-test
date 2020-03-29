#include <stdarg.h>

void variadic_va_list(void* context, void callback(void*, int), int n, va_list arguments) {
    for (int i = 0; i < n; i++) {
        int argument = va_arg(arguments, int);

        callback(context, argument);
    }
}

void variadic(void* context, void callback(void*, int), int n, int arguments, ...) {
    if (n < 1) {
        return;
    }

    callback(context, arguments);

    va_list list;
    va_start(list, arguments);

    variadic_va_list(context, callback, n - 1, list);

    va_end(list);
}
