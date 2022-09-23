# console-in-ucore-test

### 功能描述：
对rcore中的console模块进行了封装。为了避免对原本console模块的修改，增加了新的crate——rconsole。在rconsole中通过调用sbi模块中函数“console_putchar()”实现了console中定义的Console trait，然后定义了“con_init()”函数用于console的初始化，并定义了函数“info()”、“trace()”、“warn()”、“error()”对console中的相应函数进行了封装。

对于暴露给ucore调用的函数需

```rust
#[no_mangle]
pub extern "C" fn con_init(){
    console::init_console(&Console);
    log::set_max_level(log::LevelFilter::Trace);
}
```

### 编译：
进入rconre目录后进行编译，修改Cargo.toml，设置"crate-type"为“["staticlib"]”，然后使用命令“cargo build --target riscv64gc-unknown-none-elf”进行编译，可以得到“librconsole.a”静态库。

在Cargo.toml添加“[lib]”的相关配置

```
[lib]

crate-type = ["staticlib"]

name = "rconsole"

```

### 测试：
修改ucore项目代码，在main()函数中添加调用con_init()等函数的代码，要在C程序中调用rust实现的函数需要先通过extern关键字引入。
将静态库拷贝到ucore项目中，修改Makefile，添加对静态库的编译，然后编译。

### 结果：
ucore运行正常，能够显示不同级别的log信息。

