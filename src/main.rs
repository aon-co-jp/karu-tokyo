//! karu.tokyo — Rust + Poem 版TOPページ。
//! aruaru-tokyo-server / aon-tokyo と同じ技術スタック・実装方針を踏襲する:
//! DB非依存・1バイナリ完結・サーバーサイド文字列組み立てHTML(テンプレート
//! エンジン不使用)。
//!
//! テーマ: 軽井沢・あきる野市・東京を含む日本の観光とリモートワークを
//! メインに、IT・AI・AUDIO・貿易産業を紹介するサイト。
//!
//! 「クリックで検索」リンクの方針はaon-tokyoと同じ(長い検索結果URLを
//! 直接貼らず、検索エンジン自身の`?q=`/`?search_query=`形式の短いURLを
//! 組み立てる)。

use poem::listener::TcpListener;
use poem::web::Html;
use poem::{get, handler, Route, Server};

const ARUARU_TOKYO_URL: &str = "https://aruaru.tokyo/";
const AON_TOKYO_URL: &str = "https://aon.tokyo/";
const GITHUB_ORG_URL: &str = "https://github.com/aon-co-jp";

fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

fn google_search_link(label: &str, query: &str) -> String {
    format!(
        r#"<a href="https://www.google.com/search?q={}" target="_blank" rel="noopener noreferrer">🔎 {}</a>"#,
        percent_encode(query),
        label
    )
}

fn google_image_search_link(label: &str, query: &str) -> String {
    format!(
        r#"<a href="https://www.google.com/search?tbm=isch&q={}" target="_blank" rel="noopener noreferrer">🖼️ {}</a>"#,
        percent_encode(query),
        label
    )
}

fn youtube_search_link(label: &str, query: &str) -> String {
    format!(
        r#"<a href="https://www.youtube.com/results?search_query={}" target="_blank" rel="noopener noreferrer">▶️ {}</a>"#,
        percent_encode(query),
        label
    )
}

fn page_shell(title: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
body {{ font-family: -apple-system, "Hiragino Sans", "Yu Gothic", sans-serif; max-width: 780px; margin: 2rem auto; padding: 0 1rem; line-height: 1.7; color: #222; }}
h1 {{ font-size: 1.6rem; }}
h2 {{ font-size: 1.2rem; margin-top: 2rem; border-bottom: 2px solid #eee; padding-bottom: 0.3rem; }}
a {{ color: #222; }}
a:visited {{ color: #222; }}
nav a {{ margin-right: 1rem; }}
ul.linklist li {{ margin-bottom: 0.5rem; }}
footer {{ margin-top: 3rem; font-size: 0.85rem; color: #777; }}
</style>
</head>
<body>
<nav><a href="/">TOP</a> <a href="/tourism">観光・リモートワーク</a> <a href="/lifestyle">郊外暮らし・住まい・AUDIO</a> <a href="/industry">IT・AI・AUDIO・貿易</a> <a href="{AON_TOKYO_URL}">aon.tokyo</a> <a href="{ARUARU_TOKYO_URL}">aruaru.tokyo</a></nav>
{body}
<footer><p>karu.tokyo — 軽井沢・あきる野市・東京の観光とリモートワーク。 <a href="{GITHUB_ORG_URL}">GitHub (aon-co-jp)</a></p></footer>
</body>
</html>"#
    )
}

#[handler]
fn healthz() -> &'static str {
    "ok"
}

#[handler]
fn top() -> Html<String> {
    let body = format!(
        r#"<h1>karu.tokyo</h1>
<p>軽井沢・あきる野市・東京を含む日本の観光とリモートワークをメインに、
郊外でのんびりオーディオや映画を楽しみながらリモートワークする暮らし、
IT・AI・AUDIO・貿易産業をご紹介するサイトです。
姉妹サイト <a href="{AON_TOKYO_URL}">aon.tokyo</a>・<a href="{ARUARU_TOKYO_URL}">aruaru.tokyo</a> と連携しています。</p>

<h2>観光・リモートワーク</h2>
<p><a href="/tourism">→ 軽井沢・あきる野市・東京の観光とリモートワークのご紹介</a></p>

<h2>郊外暮らし・住まい・AUDIO・映画</h2>
<p><a href="/lifestyle">→ 不動産・工務店・保険・自動車/バイク・オーディオ機器売買/レンタル・貿易/文化交流のご紹介</a></p>

<h2>IT・AI・AUDIO・貿易産業</h2>
<p><a href="/industry">→ IT・AI・AUDIO・貿易産業のご紹介</a></p>
"#
    );
    Html(page_shell("karu.tokyo — 軽井沢・あきる野・東京の観光とリモートワーク", &body))
}

#[handler]
fn tourism_page() -> Html<String> {
    let areas = ["軽井沢", "あきる野市", "東京"];
    let area_links: String = areas
        .iter()
        .map(|a| format!("<li>{}</li>", google_search_link(&format!("{a} 観光 リモートワーク移住"), &format!("{a} 観光 リモートワーク 移住"))))
        .collect();

    let body = format!(
        r#"<h1>観光・リモートワーク</h1>
<p>軽井沢の避暑地としての魅力、あきる野市の自然と都心近接性、東京の
都市機能——それぞれの土地でのリモートワーク移住・二拠点生活の可能性を
ご紹介します。</p>

<h2>地域別ご紹介</h2>
<ul class="linklist">
{area_links}
</ul>

<h2>軽井沢 牧場・レストラン・ホテル・ペンション</h2>
<ul class="linklist">
<li>{ranch_photo}</li>
<li>{ranch_video}</li>
<li>{restaurant_photo}</li>
<li>{restaurant_video}</li>
<li>{hotel_photo}</li>
<li>{hotel_video}</li>
<li>{pension_photo}</li>
<li>{pension_video}</li>
</ul>

<h2>関連情報</h2>
<ul class="linklist">
<li>{telework}</li>
<li>{coworking}</li>
</ul>
"#,
        ranch_photo = google_image_search_link("軽井沢 牧場(画像)", "軽井沢 牧場"),
        ranch_video = youtube_search_link("軽井沢 牧場", "軽井沢 牧場"),
        restaurant_photo = google_image_search_link("軽井沢 綺麗なレストラン(画像)", "軽井沢 レストラン 絶景"),
        restaurant_video = youtube_search_link("軽井沢 レストラン", "軽井沢 レストラン"),
        hotel_photo = google_image_search_link("軽井沢 ホテル(画像)", "軽井沢 ホテル"),
        hotel_video = youtube_search_link("軽井沢 ホテル", "軽井沢 ホテル"),
        pension_photo = google_image_search_link("軽井沢 ペンション(画像)", "軽井沢 ペンション"),
        pension_video = youtube_search_link("軽井沢 ペンション", "軽井沢 ペンション"),
        telework = youtube_search_link("軽井沢 リモートワーク 移住 体験談", "軽井沢 リモートワーク 移住 体験談"),
        coworking = google_search_link("あきる野市 コワーキングスペース", "あきる野市 コワーキングスペース"),
    );
    Html(page_shell("観光・リモートワーク | karu.tokyo", &body))
}

#[handler]
fn lifestyle_page() -> Html<String> {
    let body = format!(
        r#"<h1>郊外暮らし・住まい・AUDIO・映画</h1>
<p>軽井沢・あきる野市など郊外でのんびりと、本格オーディオや映画を楽しみながら
リモートワークする——そんな暮らし方をご紹介します。
姉妹サイト <a href="{AON_TOKYO_URL}">aon.tokyo</a> のオーディオ紹介ページもあわせてご覧ください。</p>

<h2>不動産・建設(オーディオルーム対応の工務店)</h2>
<ul class="linklist">
<li>{real_estate}</li>
<li>{koumuten}</li>
</ul>

<h2>保険・自動車・バイク</h2>
<ul class="linklist">
<li>{insurance}</li>
<li>{car_bike}</li>
</ul>

<h2>オーディオ機器の売買・レンタル・リース</h2>
<ul class="linklist">
<li>{amp_speaker}</li>
<li>{ddc_dac}</li>
</ul>

<h2>オンライン貿易・文化交流</h2>
<ul class="linklist">
<li>{trade_culture}</li>
</ul>
"#,
        real_estate = google_search_link("軽井沢 あきる野市 別荘 中古物件", "軽井沢 あきる野市 別荘 中古物件"),
        koumuten = google_search_link("オーディオルーム 防音室 対応 工務店", "オーディオルーム 防音室 対応 工務店"),
        insurance = google_search_link("別荘 二拠点生活 火災保険 地震保険", "別荘 二拠点生活 火災保険 地震保険"),
        car_bike = google_search_link("郊外暮らし 自動車 バイク 中古車", "郊外暮らし 自動車 バイク 中古車"),
        amp_speaker = google_search_link("アンプ スピーカー 中古 売買 レンタル", "アンプ スピーカー 中古 売買 レンタル"),
        ddc_dac = google_search_link("DDC USB-DAC 中古 売買 レンタル リース", "DDC USB-DAC 中古 売買 レンタル リース"),
        trade_culture = google_search_link("オーディオ機器 海外 オンライン貿易 文化交流", "オーディオ機器 海外 オンライン貿易 文化交流"),
    );
    Html(page_shell("郊外暮らし・住まい・AUDIO・映画 | karu.tokyo", &body))
}

#[handler]
fn industry_page() -> Html<String> {
    let body = format!(
        r#"<h1>IT・AI・AUDIO・貿易産業</h1>
<p>IT・AI開発、本格オーディオ(JBL・B&amp;W等)、そして貿易産業——
軽井沢・あきる野市・東京を拠点とした事業活動をご紹介します。
詳細は姉妹サイト <a href="{AON_TOKYO_URL}">aon.tokyo</a> もあわせてご覧ください。</p>

<h2>IT・AI</h2>
<ul class="linklist">
<li>{ai_it}</li>
</ul>

<h2>AUDIO</h2>
<ul class="linklist">
<li>{audio}</li>
</ul>

<h2>貿易産業</h2>
<ul class="linklist">
<li>{trade}</li>
</ul>
"#,
        ai_it = google_search_link("AI・IT開発 最新動向", "AI IT開発 最新動向"),
        audio = youtube_search_link("JBL B&W 大型スピーカー レビュー", "JBL B&W 大型スピーカー レビュー"),
        trade = google_search_link("日本 貿易産業 最新動向", "日本 貿易産業 最新動向"),
    );
    Html(page_shell("IT・AI・AUDIO・貿易産業 | karu.tokyo", &body))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let app = Route::new()
        .at("/", get(top))
        .at("/healthz", get(healthz))
        .at("/tourism", get(tourism_page))
        .at("/lifestyle", get(lifestyle_page))
        .at("/industry", get(industry_page));

    tracing::info!("karu-tokyo-server listening on 127.0.0.1:4300");
    Server::new(TcpListener::bind("127.0.0.1:4300")).run(app).await
}
