# Rest-by-Rust

🚲🚲🚲 RustでRESTfulAPIを実装してみる。  

![成果物](./.development/img/fruit.gif)  

## 実行方法

```shell
# デバグ実行
cargo run

# 通常ビルド
cargo build --release --target-dir ./bin
./bin/release/rest-by-rust
```

以下のパスにアクセスしてみる。  

- /
- /api/hello (GET)
- /api/hello (POST) - JSON形式でnameプロパティに値をセット
- /api/hello (PUT)
- /api/hello (DELETE)
