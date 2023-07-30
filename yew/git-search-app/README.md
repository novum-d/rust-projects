### tailwind css

1. tailwind cssのインストール
```
npm install -D tailwindcss
npx tailwindcss init
```

2. テンプレートパスの指定
```
module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
        "./assets/**/*.css",
    ],
// ...
```

3. Tailwind layers

```
// assets/css/main.css
@tailwind base;
@tailwind components;
@tailwind utilities;
```


### daisyUI

1. daisyUIのインストール
```
npm i -D daisyui@latest
```

2. `tailwind.config.js`でプラグイン追加
```
module.exports = {
  //...
  plugins: [require("daisyui")],
}
```

### Trunk
```
cargo install --locked trunk
cargo install --locked wasm-bindgen-cli
```

### Open API
```
npm i -D @openapitools/openapi-generator-cli
```
