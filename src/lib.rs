use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_next_to_html(content: &str) -> String {
    let (html_body, title) = parse_next_internal(content);
    generate_template(&title, &html_body)
}

fn parse_next_internal(text: &str) -> (String, String) {
    let mut body = String::new();
    let mut title = "Documento NeXt".into();
    let mut in_list = false;
    let mut in_table = false;

    for line in text.lines() {
        let t = line.trim();
        
        // Gestione chiusura automatica liste/tabelle su riga vuota
        if t.is_empty() { 
            if in_list { body.push_str("</ul>"); in_list = false; }
            if in_table { body.push_str("</table>"); in_table = false; }
            continue; 
        }

        // Chiusura contestuale se cambia il tipo di elemento
        if !t.starts_with('|') && in_table { body.push_str("</table>"); in_table = false; }
        if !t.starts_with("- ") && in_list { body.push_str("</ul>"); in_list = false; }

        // Parsing delle righe
        if t.starts_with("@title{") { 
            title = extract(t); 
        }
        else if let Some(s) = t.strip_prefix("# ") { 
            body.push_str(&format!("<h1>{}</h1>", fmt(s))); 
        }
        else if let Some(s) = t.strip_prefix("## ") { 
            body.push_str(&format!("<h2>{}</h2>", fmt(s))); 
        }
        else if t.starts_with('|') {
            if !in_table { 
                body.push_str("<table style='width:100%; border-collapse: collapse; margin: 20px 0;'>"); 
                in_table = true; 
            }
            body.push_str("<tr>");
            for cell in t.split('|').filter(|c| !c.trim().is_empty()) {
                body.push_str(&format!("<td style='border: 1px solid #ddd; padding: 8px;'>{}</td>", fmt(cell.trim())));
            }
            body.push_str("</tr>");
        }
        else if let Some(s) = t.strip_prefix("- ") {
            if !in_list { 
                body.push_str("<ul>"); 
                in_list = true; 
            }
            body.push_str(&format!("<li>{}</li>", fmt(s))); 
        }
        else { 
            body.push_str(&format!("<p>{}</p>", fmt(t))); 
        }
    }
    
    // Chiusura finale se il file finisce mentre siamo dentro una lista/tabella
    if in_list { body.push_str("</ul>"); }
    if in_table { body.push_str("</table>"); }
    
    (body, title)
}

// Estrae il testo tra le parentesi graffe: @title{Titolo} -> Titolo
fn extract(s: &str) -> String { 
    s.split_once('{')
     .and_then(|(_,r)| r.rsplit_once('}'))
     .map(|(i,_)| i)
     .unwrap_or("")
     .into() 
}

// Funzione di formattazione avanzata (Grassetto Sicuro + LaTeX)
fn fmt(text: &str) -> String {
    let mut s = String::new();
    
    // --- 1. GESTIONE GRASSETTO (Safety Check) ---
    // Dividiamo la stringa ogni volta che troviamo "**"
    let parts: Vec<&str> = text.split("**").collect();
    let total_parts = parts.len();
    
    // I separatori sono sempre uno in meno delle parti (es. "A**B" -> 2 parti, 1 sep)
    let separators_count = if total_parts > 0 { total_parts - 1 } else { 0 };
    
    // Se i separatori sono dispari, significa che uno è rimasto aperto (1, 3, 5...)
    // Esempio: "Ciao **mondo" -> 1 separatore (dispari) -> ERRORE
    // Esempio: "Ciao **mondo**" -> 2 separatori (pari) -> OK
    let is_unbalanced = separators_count % 2 != 0;

    for (i, part) in parts.iter().enumerate() {
        s.push_str(part);

        // Dobbiamo inserire un tag o un "**" dopo questa parte?
        if i < separators_count {
            // SE siamo all'ultimo separatore E il totale è dispari (sbilanciato)
            // ALLORA trattalo come testo normale, non come HTML
            if i == separators_count - 1 && is_unbalanced {
                s.push_str("**"); 
            } else {
                // ALTRIMENTI procedi con l'alternanza b / /b
                if i % 2 == 0 {
                    s.push_str("<b>");
                } else {
                    s.push_str("</b>");
                }
            }
        }
    }
    
    // --- 2. GESTIONE MATEMATICA (LaTeX) ---
    if s.contains('$') {
        let math_parts: Vec<&str> = s.split('$').collect();
        let mut new_s = String::new();
        
        // Stessa logica di sicurezza per i dollari, se necessario
        // Qui usiamo una logica semplice alternata
        for (i, part) in math_parts.iter().enumerate() {
            if i % 2 == 1 { 
                // Parte matematica: avvolgiamo in \( ... \) per KaTeX
                new_s.push_str(&format!(r" \({}\) ", part)); 
            } else {
                // Parte testo normale
                new_s.push_str(part);
            }
        }
        return new_s;
    }
    
    s
}

pub fn generate_template(title: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{0}</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body);"></script>
    <style>
        body {{ font-family: sans-serif; padding: 20px; line-height: 1.6; max-width: 800px; margin: 0 auto; }}
        h1 {{ border-bottom: 2px solid #333; padding-bottom: 10px; color: #333; }}
        h2 {{ color: #0056b3; margin-top: 30px; }}
        table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
        td, th {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        li {{ margin-bottom: 5px; }}
        code {{ background: #f4f4f4; padding: 2px 5px; border-radius: 3px; font-family: monospace; }}
    </style>
</head>
<body>
    {1}
</body>
</html>"#, title, body)
}