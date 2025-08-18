link:
    cargo run -p wlibwild --bin main

build-objs:
    clang -c basic_objs/main.c -o basic_objs/main.o
    clang -c basic_objs/function.c -o basic_objs/function.o