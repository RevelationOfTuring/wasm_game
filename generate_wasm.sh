#!/bin/bash

wasm-pack build --target web

# 会生成一个pkg目录，wasm文件就在其中
# 然后需要将wasm文件通过package.json加入到npm项目中：
# 在www/package.json的依赖中，增加 "wasm_game": "file:../pkg"，然后进入到www文件中执行npm install一下才行
# 其中依赖项名称"wasm_game"建议与Cargo.toml中的package.name一致