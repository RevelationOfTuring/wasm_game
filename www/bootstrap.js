// bootstrap.js用于捕获wasm中抛出的错误
import("./index.js").catch((e) => {
    console.error("Error: ", e);
})

// 添加完该文件后，要去修改webpack.config.js，把output.filename从index.js改为bootstrap.js