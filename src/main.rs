use clap::Parser;
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("⚙️  Compilazione NeXt: {:?}", args.input);

    let content = fs::read_to_string(&args.input)
        .with_context(|| format!("File non trovato: {:?}", args.input))?;

    let (html, title) = parse_next(&content);
    let full_html = template(&title, &html);

    let out = args.output.unwrap_or_else(|| args.input.with_extension("html"));
    fs::write(&out, full_html)?;

    println!("✅ Fatto! Creato: {:?}", out);
    Ok(())
}

fn parse_next(text: &str) -> (String, String) {
    let mut body = String::new();
    let mut title = "Documento NeXt".into();
    let mut in_list = false;

    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() { 
            if in_list { body.push_str("</ul>"); in_list = false; }
            continue; 
        }

        if t.starts_with("@title{") { title = extract(t); }
        else if let Some(s) = t.strip_prefix("# ") { body.push_str(&format!("<h1>{}</h1>", fmt(s))); }
        else if let Some(s) = t.strip_prefix("## ") { body.push_str(&format!("<h2>{}</h2>", fmt(s))); }
        else if let Some(s) = t.strip_prefix("- ") {
            if !in_list { body.push_str("<ul>"); in_list = true; }
            body.push_str(&format!("<li>{}</li>", fmt(s)));
        }
        else if t.starts_with("@note{") { body.push_str(&format!("<div class='note'>{}</div>", fmt(&extract(t)))); }
        else if t.starts_with("@code{") { body.push_str(&format!("<pre><code>{}</code></pre>", extract(t))); }
        else { 
            if in_list { body.push_str("</ul>"); in_list = false; }
            body.push_str(&format!("<p>{}</p>", fmt(t))); 
        }
    }
    if in_list { body.push_str("</ul>"); }
    (body, title)
}

fn extract(s: &str) -> String { s.split_once('{').and_then(|(_,r)| r.rsplit_once('}')).map(|(i,_)| i).unwrap_or("").into() }

fn fmt(s: &str) -> String {
    let mut res = s.to_string();
    while let (Some(a), Some(b)) = (res.find('*'), res.rfind('*')) {
        if a == b { break; }
        res = format!("{}<strong>{}</strong>{}", &res[..a], &res[a+1..b], &res[b+1..]);
    }
    res
}

fn template(title: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html><html><head><title>{0}</title><style>
    body {{ font-family: sans-serif; max-width: 210mm; margin: 40px auto; padding: 40px; line-height: 1.6; box-shadow: 0 0 10px #ccc; }}
    h1 {{ border-bottom: 2px solid #333; }} .note {{ background: #eef; padding: 10px; border-left: 5px solid #33a; }}
    pre {{ background: #333; color: #fff; padding: 10px; border-radius: 5px; }}
    @media print {{ body {{ box-shadow: none; margin: 0; }} }}
    </style></head><body>{1}</body></html>"#, title, body)
}