# PORTING.md — karu-tokyo お引越しファイル

> このファイル1枚で、他プロジェクトへ `karu-tokyo` を導入・移設できます。
> 対象バージョン: 0.1.0(2026-07-17、新規作成)。

## 0. このリポジトリのスコープ

`karu.tokyo` のTOPページ。軽井沢・あきる野市・東京の観光とリモート
ワーク、郊外暮らし・住まい・AUDIO・映画、IT・AI・AUDIO・貿易産業を
紹介するサイト。Rust + Poem製、DB非依存の1バイナリ完結サーバー。
[`aruaru-tokyo-server`](https://github.com/aon-co-jp/aruaru-tokyo-server)・
[`aon-tokyo`](https://github.com/aon-co-jp/aon-tokyo)と同じ技術スタック・
実装方針(DB非依存、1バイナリ完結、テンプレートエンジン不使用の
サーバーサイド文字列組み立てHTML)を踏襲した姉妹サイト。

## 1. 持っていくもの(ファイル一覧)

```
karu.tokyo/
├── Cargo.toml / Cargo.lock
├── src/               # ルーティング・各ページのHTML組み立て
├── PORTING.md(本ファイル)
├── CLAUDE.md
└── README.md
```

丸ごと移設する場合はフォルダごとコピーして `cargo build --release` が
通れば移設成功。他の姉妹サイト(aruaru-tokyo-server/aon-tokyo)と
同一構成のため、それらのPORTING.mdがあれば手順はほぼ共通。

## 2. ビルド・起動

```bash
cargo build --release
./target/release/karu-tokyo-server   # 127.0.0.1:4300
```

姉妹サイトとポートが衝突しないよう、`aruaru-tokyo-server`は4100、
`aon-tokyo`は4200、`karu.tokyo`は4300を使用している。移設・複製する
場合は空いているポート番号を選ぶこと。

## 3. ページ構成

- `/` — TOP
- `/tourism` — 軽井沢・あきる野市・東京の観光とリモートワーク
- `/lifestyle` — 郊外暮らし・住まい・AUDIO・映画(不動産・工務店・保険・自動車/バイク・アンプ/スピーカー/DDC/USB-DAC売買/レンタル・貿易/文化交流)
- `/industry` — IT・AI・AUDIO・貿易産業
- `/healthz` — ヘルスチェック

## 4. デプロイ

VPS上で`cargo build --release`、systemdサービス化、`127.0.0.1:4300`に
バインド。HTTPS化はcertbot(webroot方式)。**既知の落とし穴**: 初回の
certbot実行時、nginx側に`/.well-known/acme-challenge/`のlocationが
無いとRustバックエンドへプロキシされ404で失敗する(aon.tokyo/aon.co.jp・
karu.tokyo双方で実際に発生した既知の問題)。vhost設定で必ずwebroot用
locationを先に用意してからcertbotを実行すること。

## 5. 命名規約

- クレート名・バイナリ名: `karu-tokyo-server`
- リポジトリ名: `karu-tokyo`(ローカルフォルダ名は`karu.tokyo`)

## 6. 移植・拡張時の注意

新しいページを追加する場合も、テンプレートエンジンを導入せず
サーバーサイドでの文字列組み立てHTML方針を維持すること
(aruaru-tokyo-server/aon-tokyoとの一貫性のため)。DBには依存させず
1バイナリ完結の構成を崩さないこと。
