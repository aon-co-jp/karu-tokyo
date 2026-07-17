# 開発方針＆開発環境ルール(karu-tokyo)

作業ドライブは`F:\open-runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。

## このリポジトリの役割(2026-07-17新設)

`karu.tokyo`のTOPページ。軽井沢・あきる野市・東京を含む日本の観光と
リモートワークをメインに、IT・AI・AUDIO・貿易産業を紹介するサイト。
[`aruaru-tokyo-server`](https://github.com/aon-co-jp/aruaru-tokyo-server)・
[`aon-tokyo`](https://github.com/aon-co-jp/aon-tokyo)と同じ技術スタック・
実装方針(Rust+Poem、DB非依存、1バイナリ完結、テンプレートエンジン不使用の
サーバーサイド文字列組み立てHTML)を踏襲した姉妹サイト。相互リンクで連携。

## ページ構成

- `/` — TOP
- `/tourism` — 軽井沢・あきる野市・東京の観光とリモートワークのご紹介
- `/lifestyle` — 郊外暮らし・住まい・AUDIO・映画(不動産・工務店・保険・
  自動車/バイク・アンプ/スピーカー/DDC/USB-DAC売買/レンタル・貿易/文化交流)
- `/industry` — IT・AI・AUDIO・貿易産業のご紹介
- `/healthz` — ヘルスチェック

## デプロイ

VPS上で`cargo build --release`、systemdサービス化、`127.0.0.1:4300`にバインド
(aruaru-tokyo-serverの4100・aon-tokyoの4200と衝突しないポート)。

## 関連プロジェクト

- [aon-tokyo](https://github.com/aon-co-jp/aon-tokyo) / [aon-co-jp](https://github.com/aon-co-jp/aon-co-jp)
- [aruaru-tokyo-server](https://github.com/aon-co-jp/aruaru-tokyo-server)
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本

## HANDOFF

- **2026-07-17 本番HTTPS公開完了**: DNS(ValueDomain、AレコードをConoHa
  `160.251.237.162`へ設定)反映後、certbot証明書取得(webroot方式)、
  443番vhost追加。**初回のcertbot実行はnginx側に
  `/.well-known/acme-challenge/`のlocationが無く、Rustバックエンドへ
  プロキシされ404で失敗**——aon.tokyo/aon.co.jpのセットアップ時と
  同じ既知の落とし穴。webroot用locationを追加後、証明書取得成功。
  VPS自身から`curl https://karu.tokyo/`で200・正しいタイトルを確認済み
  (外部の一部パブリックDNSはまだキャッシュ反映中)。
- **2026-07-17 lifestyle ページ追加**: 「郊外でオーディオ・映画を
  楽しみながらリモートワーク」というaon.tokyoと同種のイメージを
  `/lifestyle`ページとして追加(不動産・工務店・保険・自動車/バイク・
  オーディオ機器売買/レンタル・貿易/文化交流)。
- **2026-07-17 新規作成**: aruaru-tokyo-server/aon-tokyoと同じ構成でゼロから新設。
  ローカル(WSL Ubuntu)で`cargo build`成功、実バイナリを起動し`/`・
  `/tourism`・`/industry`・`/healthz`すべて200を確認済み。
