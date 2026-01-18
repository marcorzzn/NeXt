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
        
        if t.is_empty() { 
            if in_list { body.push_str("</ul>"); in_list = false; }
            if in_table { body.push_str("</table>"); in_table = false; }
            continue; 
        }

        if !t.starts_with('|') && in_table { body.push_str("</table>"); in_table = false; }
        if !t.starts_with("- ") && in_list { body.push_str("</ul>"); in_list = false; }

        if t.starts_with("@title{") { title = extract(t); }
        else if let Some(s) = t.strip_prefix("# ") { body.push_str(&format!("<h1>{}</h1>", fmt(s))); }
        else if let Some(s) = t.strip_prefix("## ") { body.push_str(&format!("<h2>{}</h2>", fmt(s))); }
        else if t.starts_with('|') {
            if !in_table { body.push_str("<table style='width:100%; border-collapse: collapse; margin: 20px 0;'>"); in_table = true; }
            body.push_str("<tr>");
            for cell in t.split('|').filter(|c| !c.trim().is_empty()) {
                body.push_str(&format!("<td style='border: 1px solid #ddd; padding: 8px;'>{}</td>", fmt(cell.trim())));
            }
            body.push_str("</tr>");
        }
        else if let Some(s) = t.strip_prefix("- ") {
            if !in_list { body.push_str("<ul>"); in_list = true; }
            body.push_str(&format!("<li>{}</li>", fmt(s)));
        }
        else { body.push_str(&format!("<p>{}</p>", fmt(t))); }
    }
    
    if in_list { body.push_str("</ul>"); }
    if in_table { body.push_str("</table>"); }
    (body, title)
}

fn extract(s: &str) -> String { 
    s.split_once('{').and_then(|(_,r)| r.rsplit_once('}')).map(|(i,_)| i).unwrap_or("").into() 
}

fn fmt(text: &str) -> String {
    let mut s = text.replace("**", "<b>");
    s = s.replace("**", "</b>");
    
    if s.contains('$') {
        let parts: Vec<&str> = s.split('$').collect();
        let mut new_s = String::new();
        for (i, part) in parts.iter().enumerate() {
            if i % 2 == 1 { 
                // Qui usiamo r# per evitare problemi con le slash
                new_s.push_str(&format!(r" \({}\) ", part)); 
            } else {
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
        body {{ font-family: sans-serif; padding: 20px; line-height: 1.6; }}
        h1 {{ border-bottom: 2px solid black; }}
        table {{ width: 100%; }}
        td {{ border: 1px solid #ccc; padding: 5px; }}
    </style>
</head>
<body>
    {1}
</body>
</html>"#, title, body)
}
