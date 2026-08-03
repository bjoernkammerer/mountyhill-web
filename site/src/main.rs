mod components;
mod site;
mod theme;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const DOMAIN: &str = "www.mountyhill.com";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn dist() -> PathBuf {
    root().join("dist")
}

fn copy_dir(from: &Path, to: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn write_page(path: &str, html: String) -> Result<(), Box<dyn Error>> {
    let full = dist().join(path);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&full, html)?;
    println!("  {}", path);
    Ok(())
}

fn build() -> Result<(), Box<dyn Error>> {
    let out = dist();
    let wasm_dir = out.join("wasm");
    let staged = root().join("target").join("wasm-staging");
    let keep_wasm = wasm_dir.exists();

    if keep_wasm {
        if staged.exists() {
            fs::remove_dir_all(&staged)?;
        }
        copy_dir(&wasm_dir, &staged)?;
    }
    if out.exists() {
        fs::remove_dir_all(&out)?;
    }
    fs::create_dir_all(&out)?;

    println!("seiten");
    write_page("index.html", site::index())?;
    write_page("impressum/index.html", site::impressum())?;
    write_page("datenschutz/index.html", site::datenschutz())?;
    write_page("404.html", site::not_found())?;

    let statics = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    if statics.exists() {
        copy_dir(&statics, &out)?;
        println!("  statische dateien");
    }
    if keep_wasm {
        copy_dir(&staged, &wasm_dir)?;
        println!("  wasm übernommen");
    } else {
        println!("  hinweis: kein wasm in dist/wasm, bitte ./build.sh ausführen");
    }

    fs::write(out.join(".nojekyll"), "")?;
    fs::write(out.join("CNAME"), DOMAIN)?;
    fs::write(
        out.join("robots.txt"),
        format!("User-agent: *\nAllow: /\nSitemap: https://{}/sitemap.xml\n", DOMAIN),
    )?;
    let pages = ["/", "/impressum/", "/datenschutz/"];
    let mut sitemap = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for p in pages.iter() {
        sitemap.push_str(&format!("  <url><loc>https://{}{}</loc></url>\n", DOMAIN, p));
    }
    sitemap.push_str("</urlset>\n");
    fs::write(out.join("sitemap.xml"), sitemap)?;
    println!("  CNAME, robots.txt, sitemap.xml");
    println!("fertig in dist/");
    Ok(())
}

async fn serve(port: u16) -> Result<(), Box<dyn Error>> {
    use tower_http::services::{ServeDir, ServeFile};

    let out = dist();
    let service = ServeDir::new(&out).not_found_service(ServeFile::new(out.join("404.html")));
    let app = axum::Router::new().fallback_service(service);
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("build");
    let port: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(8080);

    build()?;
    if mode == "serve" {
        serve(port).await?;
    }
    Ok(())
}
