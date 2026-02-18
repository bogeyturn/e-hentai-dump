use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde::Deserialize;

use crate::api::info::ImagePage;

#[derive(Debug, Deserialize)]
struct Manifest {
    items: Vec<ManifestItem>,
}

#[derive(Debug, Deserialize)]
struct ManifestItem {
    #[serde(default)]
    name: Option<String>,
    width: u32,
    height: u32,

    x: u32,
    y: u32,

    file: String,
}

pub fn parse_image_pages(
    json_path: impl AsRef<Path>,
    base_url: &str,
    offset: u32,
) -> Result<Vec<ImagePage>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(json_path)?;
    let manifest: Manifest = serde_json::from_str(&text)?;

    let base = base_url.trim_end_matches('/');

    let pages = manifest
        .items
        .into_iter()
        .enumerate()
        .map(|(idx, it)| {
            let id = (idx as u32)
                .checked_add(1)
                .and_then(|v| v.checked_add(offset))
                .ok_or("id overflow")?;

            let name = it.name.unwrap_or_else(|| format!("{id}")); // fallback if name missing

            let url = format!(
                "url({}/{}) -{}px -{}px no-repeat",
                base, it.file, it.x, it.y
            );

            Ok(ImagePage {
                id,
                width: it.width,
                height: it.height,
                key: "".to_owned(),
                ratio: (it.width, it.height),
                name,
                url,
            })
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    Ok(pages)
}

fn venv_python(venv_dir: &Path) -> PathBuf {
    if cfg!(windows) {
        venv_dir.join("Scripts").join("python.exe")
    } else {
        venv_dir.join("bin").join("python")
    }
}

fn run_ok(cmd: &mut Command) -> io::Result<()> {
    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("command failed with status: {status}"),
        ));
    }
    Ok(())
}

fn ensure_venv_and_pip(venv_dir: &Path) -> io::Result<PathBuf> {
    let py = venv_python(venv_dir);

    if !py.exists() {
        if let Some(parent) = venv_dir.parent() {
            if parent != Path::new("") {
                let _ = fs::create_dir_all(parent);
            }
        }

        run_ok(
            Command::new("python3")
                .arg("-m")
                .arg("venv")
                .arg(venv_dir)
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit()),
        )?;
    }

    let pip_ok = Command::new(&py)
        .args(["-m", "pip", "--version"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if !pip_ok {
        run_ok(
            Command::new(&py)
                .args(["-m", "ensurepip", "--upgrade"])
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit()),
        )?;
    }

    run_ok(
        Command::new(&py)
            .args(["-m", "pip", "install", "Pillow", "requests", "numpy"])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit()),
    )?;

    Ok(py)
}

pub fn run_hitomi(
    gid: u64,
    page: u64,
    size: u64,
    port: u16,
) -> Result<Vec<ImagePage>, Box<dyn std::error::Error>> {
    let dir: PathBuf = ["hit", &gid.to_string()].iter().collect();
    let out_path = dir.join(format!("{page}.webp"));
    let json_path = dir.join(format!("{page}.json"));
    let offset = (page - 1).checked_mul(size).ok_or("offset overflow")?;
    if json_path.exists() && out_path.exists() {
        let v = parse_image_pages(
            json_path,
            &format!("http://127.0.0.1:{}/imgs/{gid}/", port),
            offset as u32,
        )?;
        return Ok(v);
    }
    let venv = Path::new("./venv");
    let py = ensure_venv_and_pip(venv)?;
    if page == 0 {
        return Err("page must be >= 1".into());
    }

    fs::create_dir_all(&dir)?;

    let mut cmd = Command::new(py);
    let status = cmd
        .arg("-c")
        .arg(include_str!("../hitomi.py"))
        .arg("--gid")
        .arg(gid.to_string())
        .arg("--offset")
        .arg(offset.to_string())
        .arg("--size")
        .arg(size.to_string())
        .arg("--out")
        .arg(&out_path)
        .arg("--json-path")
        .arg(&json_path)
        .output()?;

    if !status.status.success() {
        println!("{}", String::from_utf8_lossy(&status.stderr));
        return Err("hitomiy.py failed".into());
    }

    let v = parse_image_pages(
        json_path,
        &format!("http://127.0.0.1:{}/imgs/{gid}/", port),
        offset as u32,
    )?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_hitomi() {
        let gid = 3791189;
        let page = 1;
        let size = 50;

        let result = run_hitomi(gid, page, size, 8888).unwrap();
    }
}
