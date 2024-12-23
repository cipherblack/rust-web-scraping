use reqwest::Error;
use scraper::{Html, Selector};
use serde::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    title: String,
    body: String,
}

async fn fetch_website(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

async fn fetch_api_data(api_url: &str) -> Result<ApiResponse, Error> {
    let response = reqwest::get(api_url).await?.json::<ApiResponse>().await?;
    Ok(response)
}

fn parse_html(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("h1").unwrap();  // استخراج تمام تگ‌های h1
    let mut titles = Vec::new();

    for element in document.select(&selector) {
        titles.push(element.text().collect::<Vec<_>>().join(""));
    }
    titles
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();  // بارگذاری متغیرهای محیطی از فایل .env

    // URL سایت برای کراول کردن
    let url = "https://example.com";
    let html = fetch_website(url).await?;
    let titles = parse_html(&html);

    println!("Extracted Titles from Website:");
    for title in titles {
        println!("{}", title);
    }

    // URL برای درخواست به API
    let api_url = "https://jsonplaceholder.typicode.com/posts/1";
    let api_response = fetch_api_data(api_url).await?;

    println!("\nAPI Response:");
    println!("Title: {}", api_response.title);
    println!("Body: {}", api_response.body);

    Ok(())
}
