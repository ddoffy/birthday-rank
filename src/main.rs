use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use askama::Template;
use std::error::Error;

#[derive(Clone)]
struct AppState {
    ranks: HashMap<String, u32>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    months: &'a [&'a str],
    selected_month: Option<&'a str>,
    selected_day: Option<i32>,
    rank: Option<u32>,
    error: Option<&'a str>,
}

impl<'a> IndexTemplate<'a> {
    fn is_selected_month(&self, m: &str) -> bool {
        self.selected_month == Some(m)
    }

    fn is_selected_day(&self, d: &i32) -> bool {
        self.selected_day == Some(*d)
    }
}

#[derive(Deserialize)]
struct Params {
    month: Option<String>,
    day: Option<i32>,
}

const MONTHS: &[&str] = &[
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load data
    let mut ranks = HashMap::new();
    // We assume the CSV is in the current working directory
    let mut rdr = csv::Reader::from_path("birthday_ranks_2026.csv")?;
    for result in rdr.records() {
        let record = result?;
        let date_str = record.get(0).ok_or("Missing date")?.to_string();
        let rank: u32 = record.get(1).ok_or("Missing rank")?.parse()?;
        ranks.insert(date_str, rank);
    }
    println!("Loaded {} ranks", ranks.len());

    let state = Arc::new(AppState { ranks });

    let app = Router::new()
        .route("/", get(handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6464").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Params>,
) -> Html<String> {
    let mut rank = None;
    let mut error = None;

    if let (Some(month), Some(day)) = (&params.month, params.day) {
        // Format key as "Month DD" to match CSV format (e.g., "January 01")
        let key = format!("{} {:02}", month, day);
        if let Some(r) = state.ranks.get(&key) {
            rank = Some(*r);
        } else {
            // Try without leading zero just in case, though CSV seems to have them
            let key_simple = format!("{} {}", month, day);
             if let Some(r) = state.ranks.get(&key_simple) {
                rank = Some(*r);
            } else {
                error = Some("Date not found in database");
            }
        }
    }

    let template = IndexTemplate {
        months: MONTHS,
        selected_month: params.month.as_deref(),
        selected_day: params.day,
        rank,
        error,
    };

    Html(template.render().unwrap_or_else(|_| "Error rendering template".to_string()))
}
