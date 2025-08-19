link:
    cargo run -p wlibwild --bin main

build-objs:
    clang -c basic_objs/main.c -o basic_objs/main.o
    clang -c basic_objs/function.c -o basic_objs/function.o

link-lld:
    nu -c "lld-link basic_objs/main.o basic_objs/function.o /SUBSYSTEM:CONSOLE /entry:main /nodefaultlib kernel32.lib /OUT:myprogram.exe"


dump-exe:
    @llvm-objdump --demangle --syms -C -D     myprogram.exe

run-myprogram:
    nu -c "myprogram.exe"