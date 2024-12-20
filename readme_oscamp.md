# 进度
练习五 完成


# 结果展示

1. 切换分支
```
git checkout linux_apps_exercise
```

2. 将arceos文件夹中的hello_app文件夹放至与arceos平级的路径

3. 生成app
```bash
cd hello_app
./start_app.sh
```

4. 运行内核
```bash
cd arceos
make run
```

# 预期结果
```
       d8888                            .d88888b.   .d8888b.
      d88888                           d88P" "Y88b d88P  Y88b
     d88P888                           888     888 Y88b.
    d88P 888 888d888  .d8888b  .d88b.  888     888  "Y888b.
   d88P  888 888P"   d88P"    d8P  Y8b 888     888     "Y88b.
  d88P   888 888     888      88888888 888     888       "888
 d8888888888 888     Y88b.    Y8b.     Y88b. .d88P Y88b  d88P
d88P     888 888      "Y8888P  "Y8888   "Y88888P"   "Y8888P"

arch = riscv64
platform = riscv64-qemu-virt
target = riscv64gc-unknown-none-elf
smp = 1
build_mode = release
log_level = warn

Load payload ...
run code address [0xffffffc080100000]
Load payload ok!
ABI Table: 0xffffffc080249000
Execute app ...
Hello, Apps!
puts
[ABI:Terminate] Terminate Apps!
```
Hello、字符串输出、退出功能均正常