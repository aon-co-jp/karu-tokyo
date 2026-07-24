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
<nav><a href="/">TOP</a> <a href="/tourism">観光・リモートワーク</a> <a href="/lifestyle">郊外暮らし・住まい・AUDIO</a> <a href="/industry">IT・AI・AUDIO・貿易</a> <a href="/help">困った時は</a> <a href="{AON_TOKYO_URL}">aon.tokyo</a> <a href="{ARUARU_TOKYO_URL}">aruaru.tokyo</a></nav>
{body}
<footer><p>karu.tokyo — 軽井沢・あきる野市・東京の観光とリモートワーク。 <a href="{GITHUB_ORG_URL}">GitHub (aon-co-jp)</a></p></footer>
</body>
</html>"#
    )
}

#[handler]
fn healthz() -> &'static str {
    healthz_impl()
}

fn healthz_impl() -> &'static str {
    "ok"
}

/// 民間のガン治療法に関する報道記事の紹介セクション(2026-07-24追加)。
/// ユーザーから提供された実際の報道見出し・リンクをそのまま紹介するのみに留め、
/// 独自の医療的な効能・安全性の主張や推奨は一切追加しない。
fn cancer_news_section() -> String {
    r##"<h2>民間のガン治療法に関する報道 / News on Cancer Treatment Research</h2>
<p style="color:#555;font-size:0.9rem;">以下は報道・公開情報の紹介のみで、独自の医療的な効能・安全性の主張は行っていません。 /
The items below are simply introduced as reported information; no independent medical claims are made.</p>
<ul class="linklist">
<li>衝撃波で腫瘍を破壊「メスも針も使わない」肝臓がんの新治療法　大阪公立大の研究チームが特定臨床研究を開始　来年中の薬事承認を目指す<br>
<span style="color:#555;">Destroying Tumors with Shockwaves — a New "No Scalpel, No Needle" Liver Cancer Treatment: Osaka Metropolitan University Research Team Begins Specified Clinical Research, Aiming for Drug/Medical Device Approval Within the Next Year</span><br>
<a href="https://www.facebook.com/masahiro.ishizuka.54?locale=ja_JP" target="_blank" rel="noopener noreferrer">📘 Facebook</a> /
<a href="https://www.youtube.com/watch?v=hRFXYCGX8Fo" target="_blank" rel="noopener noreferrer">▶️ YouTube</a></li>
<li><a href="https://www.youtube.com/watch?v=84EkcJmgmnQ" target="_blank" rel="noopener noreferrer">世界初！からだ自身が"がん治療"　九州大学が開発</a><br>
<span style="color:#555;">A World First! The Body Itself Fights Cancer — Developed by Kyushu University</span></li>
<li><a href="https://aon.tokyo/cancer" target="_blank" rel="noopener noreferrer">民間のガン治療法についての情報は aon.tokyo/cancer をご覧ください</a><br>
<span style="color:#555;">For information on non-clinical/private-sector cancer treatment approaches, see aon.tokyo/cancer.</span></li>
</ul>
"##.to_string()
}

#[handler]
fn top() -> Html<String> {
    top_impl()
}

fn top_impl() -> Html<String> {
    let cancer_news = cancer_news_section();
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

{cancer_news}
"#
    );
    Html(page_shell("karu.tokyo — 軽井沢・あきる野・東京の観光とリモートワーク", &body))
}

#[handler]
fn tourism_page() -> Html<String> {
    tourism_page_impl()
}

fn tourism_page_impl() -> Html<String> {
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
    lifestyle_page_impl()
}

fn lifestyle_page_impl() -> Html<String> {
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
fn help_page() -> Html<String> {
    help_page_impl()
}

fn help_page_impl() -> Html<String> {
    let body = r#"<h1>サイトが見られない・警告が出る場合</h1>

<h2>Google Chromeで「保護されていない通信」と出る場合</h2>
<p>Edge(Windowsの証明書ストアを使用)では正常なのに対し、Chromeは独自の
「Chrome Root Store」という、Windowsとは別の信頼済みルート証明書リストを
持っています。新しいLet's Encryptのルート証明書がまだお使いのChromeの
バージョンに反映されていない可能性があります。</p>
<p><strong>対処法:</strong></p>
<ol>
<li>Chromeを開き、アドレスバーに <code>chrome://settings/help</code> と入力して更新</li>
<li>更新後、Chromeを再起動(タスクマネージャーでプロセスが残っていないか確認)してから再度アクセス</li>
</ol>

<h2>サイトが表示されない場合(DNS_PROBE_FINISHED_NXDOMAIN等)</h2>
<p>お使いのDNS(特にCloudflareの1.1.1.1)が、ドメインの権威サーバーに
一時的に到達できないことがあります。Google(8.8.8.8)・Quad9(9.9.9.9)
など別のDNSでは問題なく解決できることが多いです。</p>
<p><strong>対処法:</strong></p>
<ol>
<li>スマホのモバイル回線(Wi-Fiオフ)で同じURLにアクセスしてみる</li>
<li>Windowsの場合: 設定 → ネットワークとインターネット → プロパティ
(使用中の接続) → DNSサーバーの割り当てを「手動」にし、以下を設定して保存
<ul>
<li><strong>優先DNS</strong>欄: <code>8.8.8.8</code> のみ入力(<code>8.8.4.4</code>と
まとめて<code>/</code>区切りで入力すると「無効なエントリ」エラーになるので注意)</li>
<li><strong>代替DNS</strong>欄: <code>8.8.4.4</code> を別欄に入力</li>
<li>「HTTPS経由のDNS」が「オン(手動テンプレート)」になっている場合は、
まず「オフ」にしてからシンプルな設定で保存を試す</li>
</ul>
</li>
<li>スマホでWi-Fi経由の場合(静的IP化不要): 「プライベートDNS」設定を使うと
DNSだけ変更できます。
<ul>
<li>設定 → ネットワークとインターネット(機種によっては「接続」、または
設定 → Wi-Fi → 詳細設定の中にある場合も)</li>
<li>「プライベートDNS」を探し、「プロバイダのホスト名」を選択</li>
<li><code>dns.google</code> と入力して保存</li>
</ul>
</li>
<li>それでも解決しない場合は、単純にDNSの反映待ち(通常数分〜1時間程度)
であることも多いので、時間を置いて再度アクセス</li>
</ol>
"#;
    Html(page_shell("困った時は | karu.tokyo", body))
}

#[handler]
fn industry_page() -> Html<String> {
    industry_page_impl()
}

fn industry_page_impl() -> Html<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encode_keeps_unreserved_chars() {
        assert_eq!(percent_encode("abc-XYZ_0.9~"), "abc-XYZ_0.9~");
    }

    #[test]
    fn percent_encode_escapes_space_and_multibyte() {
        assert_eq!(percent_encode("a b"), "a%20b");
        // "軽" is U+8EFD -> UTF-8: E8 BB BD
        assert_eq!(percent_encode("軽"), "%E8%BB%BD");
    }

    #[test]
    fn google_search_link_builds_expected_href() {
        let html = google_search_link("ラベル", "軽井沢 観光");
        assert!(html.contains(r#"href="https://www.google.com/search?q=%E8%BB%BD%E4%BA%95%E6%B2%A2%20%E8%A6%B3%E5%85%89""#));
        assert!(html.contains("target=\"_blank\""));
        assert!(html.contains("rel=\"noopener noreferrer\""));
        assert!(html.contains("ラベル"));
    }

    #[test]
    fn google_image_search_link_uses_isch_param() {
        let html = google_image_search_link("画像", "テスト");
        assert!(html.contains("tbm=isch"));
    }

    #[test]
    fn youtube_search_link_uses_search_query_param() {
        let html = youtube_search_link("動画", "テスト");
        assert!(html.contains("search_query="));
        assert!(html.contains("youtube.com/results"));
    }

    #[test]
    fn page_shell_links_to_sister_sites_and_nav() {
        let html = page_shell("t", "<p>body</p>");
        assert!(html.contains(AON_TOKYO_URL));
        assert!(html.contains(ARUARU_TOKYO_URL));
        assert!(html.contains(GITHUB_ORG_URL));
        assert!(html.contains("<title>t</title>"));
        assert!(html.contains("<p>body</p>"));
        assert!(html.contains(r#"href="/tourism""#));
        assert!(html.contains(r#"href="/lifestyle""#));
        assert!(html.contains(r#"href="/industry""#));
        assert!(html.contains(r#"href="/help""#));
    }

    #[test]
    fn healthz_returns_ok() {
        assert_eq!(healthz_impl(), "ok");
    }

    #[test]
    fn top_page_renders_all_section_links() {
        let html = top_impl().0;
        assert!(html.contains("karu.tokyo"));
        assert!(html.contains(r#"href="/tourism""#));
        assert!(html.contains(r#"href="/lifestyle""#));
        assert!(html.contains(r#"href="/industry""#));
    }

    #[test]
    fn tourism_page_renders_areas() {
        let html = tourism_page_impl().0;
        assert!(html.contains("軽井沢"));
        assert!(html.contains("あきる野市"));
        assert!(html.contains("東京"));
    }

    #[test]
    fn lifestyle_page_renders_sections() {
        let html = lifestyle_page_impl().0;
        assert!(html.contains("不動産"));
        assert!(html.contains("オーディオ機器"));
    }

    #[test]
    fn industry_page_renders_sections() {
        let html = industry_page_impl().0;
        assert!(html.contains("IT・AI"));
        assert!(html.contains("AUDIO"));
        assert!(html.contains("貿易"));
    }

    #[test]
    fn help_page_renders_dns_troubleshooting() {
        let html = help_page_impl().0;
        assert!(html.contains("Chrome Root Store"));
        assert!(html.contains("8.8.8.8"));
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let app = Route::new()
        .at("/", get(top))
        .at("/healthz", get(healthz))
        .at("/tourism", get(tourism_page))
        .at("/lifestyle", get(lifestyle_page))
        .at("/industry", get(industry_page))
        .at("/help", get(help_page));

    tracing::info!("karu-tokyo-server listening on 127.0.0.1:4300");
    Server::new(TcpListener::bind("127.0.0.1:4300")).run(app).await
}
