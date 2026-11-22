mod assets;
mod data;
mod styles;
mod views;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn percent_decode(input: &str) -> String {
    let mut bytes = Vec::with_capacity(input.len());
    let raw = input.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'+' {
            bytes.push(b' ');
            i += 1;
        } else if raw[i] == b'%' && i + 2 < raw.len() {
            let hex = std::str::from_utf8(&raw[i + 1..i + 3]).unwrap_or("");
            if let Ok(val) = u8::from_str_radix(hex, 16) {
                bytes.push(val);
                i += 3;
            } else {
                bytes.push(raw[i]);
                i += 1;
            }
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn get_query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut kv = pair.splitn(2, '=');
        let k = kv.next().unwrap_or("");
        let v = kv.next().unwrap_or("");
        if k == key {
            return Some(percent_decode(v));
        }
    }
    None
}

fn route_request(raw_target: &str) -> (&'static str, &'static str, String) {
    let (raw_path, query) = match raw_target.split_once('?') {
        Some((p, q)) => (p, q),
        None => (raw_target, ""),
    };

    // Support both clean paths (/quienes-somos) and WordPress legacy paths (/index.php/quienes-somos/)
    let normalized = raw_path
        .trim_start_matches("/index.php")
        .trim_end_matches('/');
    let path = if normalized.is_empty() { "/" } else { normalized };

    match path {
        "/assets/dvfu.css" => (
            "200 OK",
            "text/css; charset=utf-8",
            styles::DVFU_STYLESHEET.to_string(),
        ),
        "/" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_home_page(),
        ),
        "/quienes-somos" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_quienes_somos_page(),
        ),
        "/autoridades" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_autoridades_page(),
        ),
        "/nuestro-heroe-eponimo" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_eponimo_page(),
        ),
        "/carreras-de-pregrado" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_careers_index_page(),
        ),
        "/noticias"
        | "/category/noticias"
        | "/category/noticias/page/1"
        | "/category/noticias/page/2"
        | "/category/noticias/page/3"
        | "/category/destacados"
        | "/category/educacion"
        | "/category/sin-categoria" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_news_index_page(),
        ),
        "/revista-ciencia-para-la-vida" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_magazine_page(),
        ),
        "/gaceta-universitaria" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_gazettes_page(false),
        ),
        "/gaceta-extraordinaria" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_gazettes_page(true),
        ),
        "/calendario-academico-2026-2027" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_calendar_page(),
        ),
        "/campus-laboratorios" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_campus_page("Infraestructura Científica y Laboratorios"),
        ),
        "/centros-de-investigacion" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_campus_page("Centros de Investigación UNC – IVIC"),
        ),
        "/laboratorios-de-computacion-avanzados" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_campus_page("Laboratorios de Computación Avanzados"),
        ),
        "/aulas-interactivas" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_campus_page("Aulas Interactivas"),
        ),
        "/servicios-estudiantiles" | "/validacion-de-documentos" => (
            "200 OK",
            "text/html; charset=utf-8",
            views::render_validacion_page(),
        ),
        "/autenticacion-de-carnet" => {
            let q = get_query_param(query, "q");
            (
                "200 OK",
                "text/html; charset=utf-8",
                views::render_carnet_page(q.as_deref()),
            )
        }
        "/sugerencias" => {
            let sent = get_query_param(query, "enviado").is_some();
            (
                "200 OK",
                "text/html; charset=utf-8",
                views::render_sugerencias_page(sent),
            )
        }
        "/buscar" => {
            let q = get_query_param(query, "q").unwrap_or_default();
            (
                "200 OK",
                "text/html; charset=utf-8",
                views::render_search_page(&q),
            )
        }
        _ => {
            if let Some(svg_name) = path.strip_prefix("/assets/svg/") {
                let (ct, body) = assets::render_svg_asset(svg_name);
                return ("200 OK", ct, body);
            }
            if let Some(icon_name) = path.strip_prefix("/assets/icon/") {
                let (ct, body) = assets::render_icon_svg(icon_name);
                return ("200 OK", ct, body);
            }
            // Check /noticias/<slug>
            if let Some(slug) = path.strip_prefix("/noticias/") {
                if let Some(html) = views::render_news_detail_page(slug) {
                    return ("200 OK", "text/html; charset=utf-8", html);
                }
            }
            // Check /carreras/<slug>
            if let Some(slug) = path.strip_prefix("/carreras/") {
                if let Some(html) = views::render_career_detail_page(slug) {
                    return ("200 OK", "text/html; charset=utf-8", html);
                }
            }
            // Check direct root slug for news articles (matching WordPress /index.php/<slug>/)
            let trimmed_slug = path.trim_start_matches('/');
            if let Some(html) = views::render_news_detail_page(trimmed_slug) {
                return ("200 OK", "text/html; charset=utf-8", html);
            }
            if let Some(html) = views::render_career_detail_page(trimmed_slug) {
                return ("200 OK", "text/html; charset=utf-8", html);
            }

            let not_found_body = format!(
                r#"<div class="container" style="padding:50px 15px;">
  <h1>Ruta no encontrada: {path}</h1>
  <hr class="blue-line">
  <p>La página solicitada no existe o ha sido movida. Utiliza el menú superior o regresa al portal principal de la UNC.</p>
  <a href="/" class="button-s">Volver al Inicio →</a>
</div>"#,
                path = path
            );
            (
                "404 Not Found",
                "text/html; charset=utf-8",
                views::render_page_shell("404 No Encontrado", "home", &not_found_body),
            )
        }
    }
}

fn serve_real_asset(filename: &str) -> Option<(&'static str, Vec<u8>)> {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return None;
    }
    let candidates = [
        format!("static/real/{}", filename),
        format!(
            "/home/sam/.gemini/antigravity/scratch/unc-itmo-ru-portal/static/real/{}",
            filename
        ),
    ];
    for p in &candidates {
        if let Ok(bytes) = std::fs::read(p) {
            let ct = if filename.ends_with(".png") {
                "image/png"
            } else if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
                "image/jpeg"
            } else if filename.ends_with(".webp") {
                "image/webp"
            } else if filename.ends_with(".svg") {
                "image/svg+xml; charset=utf-8"
            } else {
                "application/octet-stream"
            };
            return Some((ct, bytes));
        }
    }
    None
}

fn serve_local_font(filename: &str) -> Option<(&'static str, Vec<u8>)> {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return None;
    }
    let candidates = [
        format!("static/fonts/{}", filename),
        format!(
            "/home/sam/.gemini/antigravity/scratch/unc-itmo-ru-portal/static/fonts/{}",
            filename
        ),
    ];
    for p in &candidates {
        if let Ok(bytes) = std::fs::read(p) {
            let ct = if filename.ends_with(".woff2") {
                "font/woff2"
            } else {
                "application/octet-stream"
            };
            return Some((ct, bytes));
        }
    }
    None
}

fn serve_local_video(filename: &str, req_str: &str, stream: &mut TcpStream) -> bool {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return false;
    }
    let candidates = [
        format!("static/video/{}", filename),
        format!(
            "/home/sam/.gemini/antigravity/scratch/unc-itmo-ru-portal/static/video/{}",
            filename
        ),
    ];
    let bytes = match candidates.iter().find_map(|p| std::fs::read(p).ok()) {
        Some(b) => b,
        None => return false,
    };
    let total_len = bytes.len();

    // Parse optional HTTP Range header (e.g. "Range: bytes=0-") for smooth video seeking
    let range_header = req_str
        .lines()
        .find(|l| l.to_ascii_lowercase().starts_with("range:"))
        .and_then(|l| l.split_once(':').map(|(_, v)| v.trim()));

    if let Some(rh) = range_header {
        if let Some(spec) = rh.strip_prefix("bytes=") {
            let mut parts = spec.splitn(2, '-');
            let start = parts
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            let end = parts
                .next()
                .and_then(|s| if s.is_empty() { None } else { s.parse::<usize>().ok() })
                .unwrap_or(total_len.saturating_sub(1))
                .min(total_len.saturating_sub(1));

            if start <= end && start < total_len {
                let slice = &bytes[start..=end];
                let header = format!(
                    "HTTP/1.1 206 Partial Content\r\nContent-Type: video/mp4\r\nAccept-Ranges: bytes\r\nContent-Range: bytes {}-{}/{}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: public, max-age=3600\r\n\r\n",
                    start,
                    end,
                    total_len,
                    slice.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(slice);
                let _ = stream.flush();
                return true;
            }
        }
    }

    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: video/mp4\r\nAccept-Ranges: bytes\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: public, max-age=3600\r\n\r\n",
        total_len
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(&bytes);
    let _ = stream.flush();
    true
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) | Err(_) => return,
        Ok(n) => n,
    };

    let req_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let first_line = req_str.lines().next().unwrap_or("GET / HTTP/1.1");
    let mut parts = first_line.split_whitespace();
    let _method = parts.next().unwrap_or("GET");
    let target = parts.next().unwrap_or("/");

    let (raw_path, _) = match target.split_once('?') {
        Some((p, q)) => (p, q),
        None => (target, ""),
    };

    if let Some(filename) = raw_path.strip_prefix("/assets/video/") {
        if serve_local_video(filename, &req_str, &mut stream) {
            return;
        }
    }

    if let Some(filename) = raw_path.strip_prefix("/assets/fonts/") {
        if let Some((content_type, bytes)) = serve_local_font(filename) {
            let response_header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {len}\r\nConnection: close\r\nCache-Control: public, max-age=31536000\r\n\r\n",
                content_type = content_type,
                len = bytes.len()
            );
            let _ = stream.write_all(response_header.as_bytes());
            let _ = stream.write_all(&bytes);
            let _ = stream.flush();
            return;
        }
    }

    if let Some(filename) = raw_path.strip_prefix("/assets/real/") {
        if let Some((content_type, bytes)) = serve_real_asset(filename) {
            let response_header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {len}\r\nConnection: close\r\nCache-Control: public, max-age=3600\r\n\r\n",
                content_type = content_type,
                len = bytes.len()
            );
            let _ = stream.write_all(response_header.as_bytes());
            let _ = stream.write_all(&bytes);
            let _ = stream.flush();
            return;
        }
    }

    let (status, content_type, body) = route_request(target);
    let body_bytes = body.as_bytes();

    let response_header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {len}\r\nConnection: close\r\nCache-Control: no-cache\r\n\r\n",
        status = status,
        content_type = content_type,
        len = body_bytes.len()
    );

    let _ = stream.write_all(response_header.as_bytes());
    let _ = stream.write_all(body_bytes);
    let _ = stream.flush();
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(dst)?;
    let mut count = 0;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            count += copy_dir_all(&entry.path(), &to)?;
        } else if ty.is_file() {
            std::fs::copy(entry.path(), to)?;
            count += 1;
        }
    }
    Ok(count)
}

fn rewrite_base_path(content: &str, base: &str) -> String {
    if base.is_empty() || base == "/" {
        return content.to_string();
    }
    let b = base.trim_end_matches('/');
    content
        .replace("href=\"/", &format!("href=\"{}/", b))
        .replace("src=\"/", &format!("src=\"{}/", b))
        .replace("action=\"/", &format!("action=\"{}/", b))
        .replace("poster=\"/", &format!("poster=\"{}/", b))
        .replace("url('/assets/", &format!("url('{}/assets/", b))
        .replace("url(\"/assets/", &format!("url(\"{}/assets/", b))
}

fn export_static_site(out_dir: &str, base_path: &str) {
    let out = std::path::Path::new(out_dir);
    let _ = std::fs::remove_dir_all(out);
    std::fs::create_dir_all(out).expect("Failed to create output dir");

    // Create .nojekyll so GitHub Pages serves everything cleanly
    std::fs::write(out.join(".nojekyll"), "").expect("Failed to write .nojekyll");

    // 1. Export rewritten CSS
    let css_dir = out.join("assets");
    std::fs::create_dir_all(&css_dir).expect("Failed to create assets dir");
    let css = rewrite_base_path(styles::DVFU_STYLESHEET, base_path);
    std::fs::write(css_dir.join("dvfu.css"), css).expect("Failed to write dvfu.css");

    // 2. Copy static/real, static/fonts, static/video into <out>/assets/*
    let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut asset_files = 0;
    for sub in ["real", "fonts", "video"] {
        let src = project_root.join("static").join(sub);
        if src.exists() {
            asset_files += copy_dir_all(&src, &css_dir.join(sub)).unwrap_or(0);
        }
    }

    // 3. Collect all HTML routes
    let mut routes: Vec<String> = vec![
        "/".into(),
        "/quienes-somos".into(),
        "/autoridades".into(),
        "/nuestro-heroe-eponimo".into(),
        "/carreras-de-pregrado".into(),
        "/noticias".into(),
        "/revista-ciencia-para-la-vida".into(),
        "/gaceta-universitaria".into(),
        "/gaceta-extraordinaria".into(),
        "/calendario-academico-2026-2027".into(),
        "/campus-laboratorios".into(),
        "/centros-de-investigacion".into(),
        "/laboratorios-de-computacion-avanzados".into(),
        "/aulas-interactivas".into(),
        "/servicios-estudiantiles".into(),
        "/validacion-de-documentos".into(),
        "/autenticacion-de-carnet".into(),
        "/sugerencias".into(),
        "/buscar".into(),
    ];

    for c in data::CAREER_PROGRAMS {
        routes.push(format!("/carreras/{}", c.slug));
    }
    for n in data::NEWS_ARTICLES {
        routes.push(format!("/noticias/{}", n.slug));
    }

    for route in &routes {
        let (_status, _ct, html) = route_request(route);
        let rewritten = rewrite_base_path(&html, base_path);
        let target_file = if route == "/" {
            out.join("index.html")
        } else {
            let dir = out.join(route.trim_start_matches('/'));
            std::fs::create_dir_all(&dir).expect("Failed to create route dir");
            dir.join("index.html")
        };
        std::fs::write(&target_file, rewritten).expect("Failed to write HTML page");
    }

    // Also export 404.html
    let (_, _, not_found) = route_request("/404-not-found");
    std::fs::write(out.join("404.html"), rewrite_base_path(&not_found, base_path))
        .expect("Failed to write 404.html");

    println!(
        "Exported {} HTML pages and {} static assets into '{}' (base_path='{}')",
        routes.len(),
        asset_files,
        out_dir,
        base_path
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--export-static" {
        let out_dir = &args[2];
        let base_path = args.get(3).map(|s| s.as_str()).unwrap_or("");
        export_static_site(out_dir, base_path);
        return;
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "3033".to_string());
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("Error binding to {}: {}", addr, e);
        std::process::exit(1);
    });

    println!("========================================================================");
    println!("  UNC × DVFU Design System Portal (Light & Dark Themes)");
    println!("  Listening on: http://{}", addr);
    println!("  Routes ready:");
    println!("    • /                                  (Homepage in DVFU Architecture)");
    println!("    • /quienes-somos                     (Misión, Visión, Valores)");
    println!("    • /autoridades                       (7 Autoridades Universitarias)");
    println!("    • /nuestro-heroe-eponimo             (Dr. Humberto Fernández-Morán)");
    println!("    • /carreras-de-pregrado              (15 Carreras + Mallas PDF)");
    println!("    • /carreras/<slug>                   (15 Individual Degree Pages)");
    println!("    • /noticias                          (All 30 News Index + Pagination)");
    println!("    • /noticias/<slug>                   (30 Full Verbatim News Articles)");
    println!("    • /revista-ciencia-para-la-vida      (6 Magazine Issues + PDFs)");
    println!("    • /gaceta-universitaria              (Gacetas N.º 1-6 + Extraordinaria)");
    println!("    • /calendario-academico-2026-2027    (Academic Calendar + PDF)");
    println!("    • /campus-laboratorios               (Altos de Pipe Facilities)");
    println!("    • /validacion-de-documentos          (Validation Lists & Bus Routes)");
    println!("    • /autenticacion-de-carnet           (Interactive Student ID Lookup)");
    println!("    • /sugerencias                       (Suggestion Form + Google Map)");
    println!("========================================================================");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                handle_client(stream);
            });
        }
    }
}
