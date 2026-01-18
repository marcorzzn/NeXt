use clap::Parser;
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// NeXt Compiler: The Future of Documents
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File di input (.nxt)
    #[arg(short, long)]
    input: PathBuf,

    /// File di output (opzionale, default .html)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Leggi il file sorgente
    println!("⚙️  Compilazione di {:?}...", args.input);
    let content = fs::read_to_string(&args.input)
        .with_context(|| format!("Impossibile trovare il file: {:?}", args.input))?;

    // 2. Analizza e trasforma (Parsing)
    let (html_body, metadata) = parse_next(&content);

    // 3. Genera il documento finale con CSS Tipografico
    let final_html = generate_template(&metadata.title, &html_body);

    // 4. Salva il file
    let output_path = args.output.unwrap_or_else(|| args.input.with_extension("html"));
    fs::write(&output_path, final_html)?;

    println!("✅ Documento NeXt generato: {:?}", output_path);
    Ok(())
}

// --- STRUTTURE DATI ---
struct Metadata {
    title: String,
}

// --- PARSER (Il motore logico) ---
fn parse_next(input: &str) -> (String, Metadata) {
    let mut body = String::new();
    let mut title = "Documento Senza Titolo".to_string();
    let mut in_list = false;

    for line in input.lines() {
        let trimmed = line.trim();

        // Salta le righe vuote ma chiudi le liste se aperte
        if trimmed.is_empty() {
            if in_list { body.push_str("</ul>"); in_list = false; }
            continue;
        }

        // --- GRAMMATICA DI NEXT ---

        // 1. Metadati: @title{...}
        if trimmed.starts_with("@title{") {
            title = extract_content(trimmed);
        }
        // 2. Titoli: # e ##
        else if let Some(h1) = trimmed.strip_prefix("# ") {
            body.push_str(&format!("<h1>{}</h1>", parse_style(h1)));
        }
        else if let Some(h2) = trimmed.strip_prefix("## ") {
            body.push_str(&format!("<h2>{}</h2>", parse_style(h2)));
        }
        // 3. Componenti Speciali: @note e @code
        else if trimmed.starts_with("@note{") {
            let content = extract_content(trimmed);
            body.push_str(&format!("<div class='nxt-note'><strong>Nota:</strong> {}</div>", parse_style(&content)));
        }
        else if trimmed.starts_with("@code{") {
            let content = extract_content(trimmed);
            body.push_str(&format!("<pre class='nxt-code'><code>{}</code></pre>", content));
        }
        // 4. Liste: - item
        else if let Some(item) = trimmed.strip_prefix("- ") {
            if !in_list { body.push_str("<ul>"); in_list = true; }
            body.push_str(&format!("<li>{}</li>", parse_style(item)));
        }
        // 5. Paragrafi normali
        else {
            if in_list { body.push_str("</ul>"); in_list = false; }
            body.push_str(&format!("<p>{}</p>", parse_style(trimmed)));
        }
    }

    if in_list { body.push_str("</ul>"); }
    (body, Metadata { title })
}

// Estrae testo tra le graffe { ... }
fn extract_content(text: &str) -> String {
    let start = text.find('{').unwrap_or(0) + 1;
    let end = text.rfind('}').unwrap_or(text.len());
    if start < end { text[start..end].to_string() } else { "".to_string() }
}

// Gestisce *grassetto* e `codice inline`
fn parse_style(text: &str) -> String {
    let mut s = text.to_string();
    // Un approccio semplice di sostituzione (in un motore reale si userebbe uno stack)
    // Nota: questo gestisce solo casi semplici senza nesting
    while let (Some(start), Some(end)) = (s.find('*'), s.rfind('*')) {
        if start == end { break; } // Solo un asterisco
        let before = &s[0..start];
        let middle = &s[start+1..end];
        let after = &s[end+1..];
        s = format!("{}<strong>{}</strong>{}", before, middle, after);
    }
    s
}

// --- TEMPLATE ENGINE (CSS & Layout) ---
fn generate_template(title: &str, body: &str) -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="it">
<head>
    <meta charset="UTF-8">
    <title>{title}</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;800&family=JetBrains+Mono:wght@400&display=swap" rel="stylesheet">
    <style>
        :root {{ --primary: #000; --accent: #2563eb; --bg: #f3f4f6; }}
        body {{
            background: var(--bg);
            font-family: 'Inter', sans-serif;
            margin: 0; padding: 40px;
            display: flex; justify-content: center;
        }}
        .page {{
            background: white;
            width: 210mm; min-height: 297mm; /* A4 */
            padding: 25mm;
            box-shadow: 0 15px 35px rgba(0,0,0,0.1);
            box-sizing: border-box;
        }}
        /* Tipografia */
        h1 {{ font-weight: 800; font-size: 2.8rem; letter-spacing: -1.5px; margin-top: 0; }}
        h2 {{ font-weight: 600; font-size: 1.5rem; margin-top: 2rem; color: #333; border-bottom: 2px solid #eee; padding-bottom: 10px; }}
        p {{ line-height: 1.65; color: #374151; font-size: 1.05rem; margin-bottom: 1.2rem; }}
        li {{ line-height: 1.6; margin-bottom: 0.5rem; }}
        strong {{ color: var(--primary); font-weight: 700; }}
        
        /* Componenti NeXt */
        .nxt-note {{
            background: #eff6ff;
            border-left: 4px solid var(--accent);
            padding: 1rem 1.5rem;
            border-radius: 0 8px 8px 0;
            margin: 1.5rem 0;
            color: #1e40af;
        }}
        .nxt-code {{
            background: #1e293b; color: #f8fafc;
            padding: 1.5rem; border-radius: 8px;
            overflow-x: auto; margin: 1.5rem 0;
        }}
        code {{ font-family: 'JetBrains Mono', monospace; font-size: 0.9em; }}

        /* Modalità Stampa (PDF) */
        @media print {{
            body {{ background: white; padding: 0; }}
            .page {{ box-shadow: none; margin: 0; width: 100%; }}
        }}
    </style>
</head>
<body>
    <div class="page">
        {body}
        <footer style="margin-top: 50px; border-top: 1px solid #eee; padding-top: 20px; font-size: 0.8rem; color: #9ca3af; text-align: center;">
            Documento generato con <strong>NeXt</strong>
        </footer>
    </div>
</body>
</html>
"#, title=title, body=body)
}
