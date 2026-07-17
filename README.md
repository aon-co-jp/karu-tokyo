# karu-tokyo-server

`karu.tokyo`のTOPページ。Rust + Poem製、DB非依存の1バイナリ完結サーバー。
軽井沢・あきる野市・東京の観光とリモートワーク、IT・AI・AUDIO・貿易産業のご紹介。
姉妹サイト: [aon.tokyo](https://aon.tokyo/) / [aruaru.tokyo](https://aruaru.tokyo/)

## ページ

- `/` — TOP
- `/tourism` — 観光・リモートワーク
- `/industry` — IT・AI・AUDIO・貿易産業
- `/healthz` — ヘルスチェック

## ビルド・起動

```bash
cargo build --release
./target/release/karu-tokyo-server   # 127.0.0.1:4300
```

## ライセンス

Apache-2.0 OR MIT
