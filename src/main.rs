use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "fsd", version, about = "FSD (Feature-Sliced Design) folder scaffolder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize an FSD folder structure
    Init(InitArgs),
    /// Add a slice (feature/entity/widget/page/etc.) with optional segments
    Add(AddArgs),
}

#[derive(Parser, Debug)]
struct InitArgs {
    /// Project root directory (defaults to current dir)
    #[arg(long)]
    root: Option<PathBuf>,

    /// Base directory where FSD lives (defaults to "src")
    #[arg(long, default_value = "src")]
    base: PathBuf,

    /// Template style
    #[arg(long, value_enum, default_value_t = InitStyle::Minimal)]
    style: InitStyle,

    /// Include "processes" layer
    #[arg(long)]
    processes: bool,

    /// Don't create anything; only print planned paths
    #[arg(long)]
    dry_run: bool,

    /// Continue if a directory already exists
    #[arg(long)]
    force: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum InitStyle {
    Minimal,
    Full,
}

#[derive(Parser, Debug)]
struct AddArgs {
    /// Project root directory (defaults to current dir)
    #[arg(long)]
    root: Option<PathBuf>,

    /// Base directory where FSD lives (defaults to "src")
    #[arg(long, default_value = "src")]
    base: PathBuf,

    /// Layer to add the slice into (e.g. features, entities, pages, widgets, shared)
    #[arg(value_enum)]
    layer: Layer,

    /// Slice name (e.g. auth, user, profile)
    slice: String,

    /// Create common segments inside the slice (comma-separated)
    ///
    /// Defaults to: api,model,ui,lib,config
    #[arg(long)]
    segments: Option<OsString>,

    /// Don't create anything; only print planned paths
    #[arg(long)]
    dry_run: bool,

    /// Continue if a directory already exists
    #[arg(long)]
    force: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Layer {
    App,
    Processes,
    Pages,
    Widgets,
    Features,
    Entities,
    Shared,
}

impl Layer {
    fn as_dir(self) -> &'static str {
        match self {
            Layer::App => "app",
            Layer::Processes => "processes",
            Layer::Pages => "pages",
            Layer::Widgets => "widgets",
            Layer::Features => "features",
            Layer::Entities => "entities",
            Layer::Shared => "shared",
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_dir())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => cmd_init(args),
        Commands::Add(args) => cmd_add(args),
    }
}

fn cmd_init(args: InitArgs) -> Result<()> {
    let root = args
        .root
        .unwrap_or(std::env::current_dir().context("failed to read current directory")?);

    let base = root.join(&args.base);
    let mut dirs: Vec<PathBuf> = Vec::new();

    // Always ensure base exists.
    dirs.push(base.clone());

    let mut layers = vec![
        Layer::App,
        Layer::Pages,
        Layer::Widgets,
        Layer::Features,
        Layer::Entities,
        Layer::Shared,
    ];
    if args.processes {
        layers.insert(1, Layer::Processes);
    }

    for layer in layers {
        dirs.push(base.join(layer.as_dir()));
    }

    if matches!(args.style, InitStyle::Full) {
        // Opinionated, but common starter structure.
        dirs.extend([
            base.join("shared").join("ui"),
            base.join("shared").join("lib"),
            base.join("shared").join("api"),
            base.join("shared").join("config"),
            base.join("shared").join("assets"),
            base.join("app").join("providers"),
            base.join("app").join("router"),
            base.join("app").join("styles"),
        ]);
    }

    create_dirs(&dirs, args.dry_run, args.force)
        .with_context(|| format!("failed to initialize fsd structure under {}", base.display()))?;
    Ok(())
}

fn cmd_add(args: AddArgs) -> Result<()> {
    let root = args
        .root
        .unwrap_or(std::env::current_dir().context("failed to read current directory")?);

    let base = root.join(&args.base);
    let layer_dir = base.join(args.layer.as_dir());

    let slice = sanitize_name(&args.slice)
        .with_context(|| format!("invalid slice name: {:?}", args.slice))?;

    let slice_dir = layer_dir.join(&slice);

    let segments = parse_segments(args.segments.as_ref(), args.layer);
    let mut dirs = vec![base, layer_dir, slice_dir.clone()];
    for seg in segments {
        // Avoid accidental duplicates like "shared/ui/ui" if user chooses same name.
        if seg == slice.as_str() {
            continue;
        }
        dirs.push(slice_dir.join(seg));
    }

    create_dirs(&dirs, args.dry_run, args.force).with_context(|| {
        format!(
            "failed to add slice '{}' to layer '{}'",
            slice, args.layer
        )
    })?;
    Ok(())
}

fn create_dirs(paths: &[PathBuf], dry_run: bool, force: bool) -> Result<()> {
    // De-dup while preserving order (small n, simple O(n^2) is fine).
    let mut unique: Vec<&PathBuf> = Vec::new();
    for p in paths {
        if !unique.iter().any(|u| u.as_path() == p.as_path()) {
            unique.push(p);
        }
    }

    for p in unique {
        if dry_run {
            println!("{}", p.display());
            continue;
        }

        match fs::create_dir_all(p) {
            Ok(()) => {}
            Err(e) if force && is_already_exists(&e, p) => {}
            Err(e) => return Err(e).with_context(|| format!("creating directory {}", p.display())),
        }
    }

    Ok(())
}

fn is_already_exists(err: &std::io::Error, path: &Path) -> bool {
    // create_dir_all returns AlreadyExists sometimes only if a component is a file.
    // If the directory already exists, it typically succeeds. This helper is defensive.
    if err.kind() == std::io::ErrorKind::AlreadyExists {
        return true;
    }
    path.is_dir()
}

fn sanitize_name(input: &str) -> Result<String> {
    let s = input.trim();
    if s.is_empty() {
        anyhow::bail!("slice name cannot be empty");
    }
    if s.contains(std::path::MAIN_SEPARATOR) || s.contains('/') || s.contains('\\') {
        anyhow::bail!("slice name must not contain path separators");
    }
    if s == "." || s == ".." {
        anyhow::bail!("invalid slice name");
    }
    Ok(s.to_string())
}

fn parse_segments(raw: Option<&OsString>, layer: Layer) -> Vec<&'static str> {
    // Defaults:
    // - shared layer typically uses top-level slices (ui/lib/api/...), so default is no inner segments
    // - other slices commonly have "ui/model/api/lib/config"
    let default_slice = ["api", "model", "ui", "lib", "config"];

    let default = match layer {
        Layer::Shared => [].as_slice(),
        _ => default_slice.as_slice(),
    };

    let Some(raw) = raw else {
        return default.to_vec();
    };

    let s = raw.to_string_lossy();
    let s = s.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") {
        return Vec::new();
    }
    if s.eq_ignore_ascii_case("default") {
        return default.to_vec();
    }

    // Parse comma-separated list; keep only known segments to avoid unexpected filesystem writes.
    let mut out: Vec<&'static str> = Vec::new();
    for part in s.split(',').map(|p| p.trim()).filter(|p| !p.is_empty()) {
        let seg: Option<&'static str> = match part {
            "ui" => Some("ui"),
            "model" => Some("model"),
            "api" => Some("api"),
            "lib" => Some("lib"),
            "config" => Some("config"),
            "assets" => Some("assets"),
            _ => None, // Ignore unknown values intentionally.
        };
        if let Some(seg) = seg {
            if !out.contains(&seg) {
                out.push(seg);
            }
        }
    }
    if out.is_empty() {
        default.to_vec()
    } else {
        out
    }
}
