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
    println!("⚙️  Compilazione NeXt v1.3 (Completa)...");

    let content = fs::read_to_string(&args.input)
        .with_context(|| format!("File non trovato: {:?}", args.input))?;

    let (html, title) = parse_next(&content);
    let full_html = template(&title, &html);

    let out = args.output.unwrap_or_else(|| args.input.with_extension("html"));
    fs::write(&out, full_html)?;

    println!("✅ Documento completo generato: {:?}", out);
    Ok(())
}

fn parse_next(text: &str) -> (String, String) {
    let mut body = String::new();
    let mut title = "Documento NeXt".into();
    let mut in_list = false;
    let mut in_table = false;

    for line in text.lines() {
        let t = line.trim();
        
        // 1. Gestione righe vuote: chiudono liste e tabelle
        if t.is_empty() { 
            if in_list { body.push_str("</ul>"); in_list = false; }
            if in_table { body.push_str("</table>"); in_table = false; }
            continue; 
        }

        // 2. Chiusura automatica se cambia il tipo di contenuto
        // Se eravamo in una tabella ma la riga non inizia con '|', chiudiamo la tabella
        if !t.starts_with('|') && in_table {
            body.push_str("</table>");
            in_table = false;
        }
        // Se eravamo in una lista ma la riga non inizia con '-', chiudiamo la lista
        if !t.starts_with("- ") && in_list {
            body.push_str("</ul>");
            in_list = false;
        }

        // --- 3. PARSING DEI COMANDI ---
        
        // Metadati
        if t.starts_with("@title{") { 
            title = extract(t); 
        }
        // Titoli
        else if let Some(s) = t.strip_prefix("# ") { 
            body.push_str(&format!("<h1>{}</h1>", fmt(s))); 
        }
        else if let Some(s) = t.strip_prefix("## ") { 
            body.push_str(&format!("<h2>{}</h2>", fmt(s))); 
        }
        
        // Tabelle
        else if t.starts_with('|') {
            if !in_table { body.push_str("<table class='nxt-table'>"); in_table = true; }
            body.push_str("<tr>");
            let cells: Vec<&str> = t.split('|').filter(|c| !c.trim().is_empty()).collect();
            for cell in cells {
                body.push_str(&format!("<td>{}</td>", fmt(cell.trim())));
            }
            body.push_str("</tr>");
        }
        
        // Liste
        else if let Some(s) = t.strip_prefix("- ") {
            if !in_list { body.push_str("<ul>"); in_list = true; }
            body.push_str(&format!("<li>{}</li>", fmt(s)));
        }
        
        // Componenti Speciali
        else if t.starts_with("@note{") { 
            body.push_str(&format!("<div class='note'>{}</div>", fmt(&extract(t)))); 
        }
        else if t.starts_with("@code{") { 
            body.push_str(&format!("<pre><code>{}</code></pre>", extract(t))); 
        }
        else if t.starts_with("@image{") { 
            let url = extract(t);
            body.push_str(&format!("<img src='{}' class='nxt-image' alt='NeXt Image'>", url)); 
        }
        // Paragrafo standard
        else { 
            body.push_str(&format!("<p>{}</p>", fmt(t))); 
        }
    }
    
    // Pulizia finale (chiusura tag rimasti aperti alla fine del file)
    if in_list { body.push_str("</ul>"); }
    if in_table { body.push_str("</table>"); }
    
    (body, title)
}

// Estrae il testo tra le parentesi graffe { ... }
fn extract(s: &str) -> String { 
    s.split_once('{').and_then(|(_,r)| r.rsplit_once('}')).map(|(i,_)| i).unwrap_or("").into() 
}

// Formatta il testo (Matematica $...$ e Grassetto *...*)
fn fmt(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '$' => {
                let mut math = String::new();
                let mut closed = false;
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == '$' { closed = true; break; }
                    math.push(next);
                }
                // Trasforma $...$ in \(...\) per KaTeX
                if closed { result.push_str(&format!("\\({}\\)", math)); } 
                else { result.push('$'); result.push_str(&math); }
            }
            '*' => {
                let mut content = String::new();
                let mut closed = false;
                while let Some(&next) = chars.peek() {
                    if next == '*' { chars.next(); closed = true; break; }
                    content.push(chars.next().unwrap());
                }
                if closed { result.push_str(&format!("<strong>{}</strong>", content)); } 
                else { result.push('*'); result.push_str(&content); }
            }
            _ => result.push(c),
        }
    }
    result
}

// Genera l'HTML finale con CSS e script KaTeX
fn template(title: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>{0}</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body);"></script>
    
    <style>
        body {{ font-family: 'Segoe UI', sans-serif; max-width: 210mm; margin: 40px auto; padding: 40px; line-height: 1.6; color: #333; }}
        h1 {{ border-bottom: 2px solid #000; padding-bottom: 10px; }} 
        h2 {{ color: #444; margin-top: 30px; }}
        
        /* Note */
        .note {{ background: #e0f2fe; padding: 15px; border-left: 5px solid #0284c7; margin: 20px 0; border-radius: 4px; }}
        
        /* Codice */
        pre {{ background: #1e293b; color: #fff; padding: 15px; border-radius: 5px; overflow-x: auto; }}
        code {{ font-family: 'Consolas', monospace; }}
        
        /* Liste */
        li {{ margin-bottom: 5px; }}
        
        /* Immagini */
        .nxt-image {{ max-width: 100%; height: auto; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); display: block; margin: 20px auto; }}
        
        /* Tabelle */
        .nxt-table {{ width: 100%; border-collapse: collapse; margin: 20px 0; font-size: 0.95em; }}
        .nxt-table td {{ border: 1px solid #ddd; padding: 12px; }}
        .nxt-table tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .nxt-table tr:first-child {{ background-color: #1e293b; color: white; font-weight: bold; }}

        @media print {{ body {{ margin: 0; max-width: 100%; }} }}
    </style>
</head>
<body>
    {1}
</body>
</html>"#, title, body)
}