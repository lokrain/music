use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use directories::ProjectDirs;
use music_score::planner::{
    CadenceKind, PhraseMetadata, ReharmZone, SectionTemplate, TemplateSummary, builtin_template,
    builtin_template_ids, load_template_from_path,
};
use serde::Serialize;

use crate::args::{
    TemplateCommands, TemplateExportArgs, TemplateImportArgs, TemplateListArgs, TemplateShowArgs,
    TemplateSourceFilter, TemplateSourcePriority,
};

const TEMPLATE_ENV_DIR: &str = "MUSIC_CLI_TEMPLATES_DIR";

pub fn run_template_command(command: TemplateCommands) -> Result<()> {
    match command {
        TemplateCommands::List(args) => handle_list(args),
        TemplateCommands::Show(args) => handle_show(args),
        TemplateCommands::Import(args) => handle_import(args),
        TemplateCommands::Export(args) => handle_export(args),
    }
}

fn handle_list(args: TemplateListArgs) -> Result<()> {
    let store = LocalTemplateStore::prepare()?;
    if matches!(args.source, TemplateSourceFilter::All | TemplateSourceFilter::Builtin) {
        println!("Built-in templates:");
        for id in builtin_template_ids() {
            if let Some(template) = builtin_template(id) {
                let summary = TemplateSummary::from(&template);
                print_summary(&summary, "builtin", args.verbose);
            }
        }
        println!();
    }
    if matches!(args.source, TemplateSourceFilter::All | TemplateSourceFilter::Local) {
        println!("Local templates ({}):", store.root.display());
        for record in store.list()? {
            print_summary(&record.summary, "local", args.verbose);
        }
    }
    Ok(())
}

fn handle_show(args: TemplateShowArgs) -> Result<()> {
    let store = LocalTemplateStore::prepare()?;
    let resolved = resolve_template(&store, &args.id, args.source)?;
    let metadata = TemplateMetadataView::from(&resolved.template);
    println!("{}", serde_json::to_string_pretty(&metadata)?);
    if args.raw {
        if let Some(raw_path) = resolved.raw_path.as_ref() {
            println!("\n# raw template ({}):", raw_path.display());
            let contents = fs::read_to_string(raw_path)
                .with_context(|| format!("failed to read template from {}", raw_path.display()))?;
            println!("{}", contents);
        } else {
            println!("\nRaw DSL not available for built-in templates.");
        }
    }
    Ok(())
}

fn handle_import(args: TemplateImportArgs) -> Result<()> {
    let store = LocalTemplateStore::prepare()?;
    let imported = store.import(&args.path, args.force)?;
    println!(
        "Imported template '{}' (version {}) into {}",
        imported.summary.id,
        imported.summary.version,
        store.root.display()
    );
    Ok(())
}

fn handle_export(args: TemplateExportArgs) -> Result<()> {
    let store = LocalTemplateStore::prepare()?;
    let resolved = resolve_template(&store, &args.id, args.source)?;
    let default_ext = resolved.format_ext.clone().unwrap_or_else(|| "json5".to_string());
    let default_name = format!("{}.{}", resolved.template.metadata.id, default_ext);
    let destination = match &args.output {
        Some(path) if path.is_dir() => path.join(&default_name),
        Some(path) => path.clone(),
        None => std::env::current_dir()
            .context("unable to determine current directory")?
            .join(&default_name),
    };
    if destination.exists() && !args.overwrite {
        bail!("{} already exists (use --overwrite)", destination.display());
    }
    if let Some(raw_path) = resolved.raw_path {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create parent directories for {}", destination.display())
            })?;
        }
        fs::copy(&raw_path, &destination).with_context(|| {
            format!(
                "failed to export template '{}' to {}",
                resolved.template.metadata.id,
                destination.display()
            )
        })?;
    } else {
        let doc = TemplateExportDocument::from(&resolved.template);
        let json = serde_json::to_string_pretty(&doc)?;
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create parent directories for {}", destination.display())
            })?;
        }
        let mut file = fs::File::create(&destination)
            .with_context(|| format!("failed to write {}", destination.display()))?;
        file.write_all(json.as_bytes())?;
    }
    println!("Exported template '{}' -> {}", resolved.template.metadata.id, destination.display());
    Ok(())
}

struct ResolvedTemplate {
    template: SectionTemplate,
    raw_path: Option<PathBuf>,
    format_ext: Option<String>,
}

fn resolve_template(
    store: &LocalTemplateStore,
    id: &str,
    source: TemplateSourcePriority,
) -> Result<ResolvedTemplate> {
    match source {
        TemplateSourcePriority::Local => {
            store.load(id)?.ok_or_else(|| anyhow!("template '{}' not found in local registry", id))
        }
        TemplateSourcePriority::Builtin => builtin_template(id)
            .map(|template| ResolvedTemplate {
                template,
                raw_path: None,
                format_ext: Some("json5".into()),
            })
            .ok_or_else(|| anyhow!("built-in template '{}' not found", id)),
        TemplateSourcePriority::Auto => {
            if let Some(local) = store.load(id)? {
                Ok(local)
            } else if let Some(template) = builtin_template(id) {
                Ok(ResolvedTemplate { template, raw_path: None, format_ext: Some("json5".into()) })
            } else {
                Err(anyhow!("template '{}' not found (local or built-in)", id))
            }
        }
    }
}

struct LocalTemplateStore {
    root: PathBuf,
}

struct LocalTemplateRecord {
    summary: TemplateSummary,
    path: PathBuf,
    format_ext: String,
    template: SectionTemplate,
}

impl LocalTemplateStore {
    fn prepare() -> Result<Self> {
        if let Ok(dir) = std::env::var(TEMPLATE_ENV_DIR) {
            let path = PathBuf::from(dir);
            fs::create_dir_all(&path).with_context(|| {
                format!("failed to create template directory {}", path.display())
            })?;
            return Ok(Self { root: path });
        }
        let proj = ProjectDirs::from("com", "lokrain", "music").ok_or_else(|| {
            anyhow!("unable to determine data directory; set {} env var", TEMPLATE_ENV_DIR)
        })?;
        let path = proj.data_dir().join("templates");
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create template directory {}", path.display()))?;
        Ok(Self { root: path })
    }

    fn list(&self) -> Result<Vec<LocalTemplateRecord>> {
        let mut records = Vec::new();
        if !self.root.exists() {
            return Ok(records);
        }
        for entry in fs::read_dir(&self.root)
            .with_context(|| format!("failed to read template directory {}", self.root.display()))?
        {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let path = entry.path();
            if let Some(record) = self.build_record(&path)? {
                records.push(record);
            }
        }
        records.sort_by(|a, b| a.summary.id.cmp(&b.summary.id));
        Ok(records)
    }

    fn load(&self, id: &str) -> Result<Option<ResolvedTemplate>> {
        for ext in POSSIBLE_EXTS {
            let path = self.root.join(format!("{}.{}", id, ext));
            if path.exists()
                && let Some(record) = self.build_record(&path)? {
                    return Ok(Some(ResolvedTemplate {
                        template: record.template,
                        raw_path: Some(record.path),
                        format_ext: Some(record.format_ext),
                    }));
                }
        }
        Ok(None)
    }

    fn build_record(&self, path: &Path) -> Result<Option<LocalTemplateRecord>> {
        let template = match load_template_from_path(path) {
            Ok(template) => template,
            Err(_) => return Ok(None),
        };
        let id = template.metadata.id.trim().to_string();
        if !is_valid_id(&id) {
            return Ok(None);
        }
        let summary = TemplateSummary::from(&template);
        let ext = path
            .extension()
            .and_then(OsStr::to_str)
            .map(|s| s.to_lowercase())
            .unwrap_or_else(|| "json5".into());
        Ok(Some(LocalTemplateRecord {
            summary,
            path: path.to_path_buf(),
            format_ext: ext,
            template,
        }))
    }

    fn import(&self, path: &Path, force: bool) -> Result<LocalTemplateRecord> {
        let template = load_template_from_path(path)
            .with_context(|| format!("failed to parse template from {}", path.display()))?;
        let id = template.metadata.id.trim();
        if id.is_empty() {
            bail!("template metadata id is required");
        }
        if !is_valid_id(id) {
            bail!("template id '{id}' must contain only [a-zA-Z0-9_-]");
        }
        let ext = path
            .extension()
            .and_then(OsStr::to_str)
            .map(|s| s.to_lowercase())
            .ok_or_else(|| anyhow!("unable to infer template file extension"))?;
        if !POSSIBLE_EXTS.contains(&ext.as_str()) {
            bail!("unsupported extension '.{}'", ext);
        }
        let dest = self.root.join(format!("{}.{}", id, ext));
        if dest.exists() && !force {
            bail!("template '{}' already exists (use --force)", id);
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(path, &dest)
            .with_context(|| format!("failed to copy template to {}", dest.display()))?;
        Ok(self.build_record(&dest)?.expect("record exists"))
    }
}

const POSSIBLE_EXTS: [&str; 3] = ["json5", "json", "ron"];

fn is_valid_id(id: &str) -> bool {
    id.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn print_summary(summary: &TemplateSummary, source: &str, verbose: bool) {
    println!(
        "  {:<20} bars:{:<3} ver:{:<3} phrases:{:<2} [{}]",
        summary.id, summary.bars, summary.version, summary.phrases, source
    );
    if verbose {
        println!(
            "      id={} bars={} version={} phrases={} source={}",
            summary.id, summary.bars, summary.version, summary.phrases, source
        );
    }
}

#[derive(Serialize)]
struct TemplateMetadataView {
    id: String,
    version: u16,
    bars: u16,
    phrases: Vec<PhraseMetadataView>,
    tension_curve: Vec<f32>,
    reharm_zones: Vec<ReharmZoneView>,
}

impl From<&SectionTemplate> for TemplateMetadataView {
    fn from(template: &SectionTemplate) -> Self {
        Self {
            id: template.metadata.id.clone(),
            version: template.metadata.version,
            bars: template.bars,
            phrases: template
                .metadata
                .phrases
                .iter()
                .map(PhraseMetadataView::from)
                .collect(),
            tension_curve: template.tension_curve.clone(),
            reharm_zones: template.reharm_zones.iter().map(ReharmZoneView::from).collect(),
        }
    }
}

#[derive(Serialize)]
struct PhraseMetadataView {
    name: String,
    start_bar: u16,
    length: u8,
    cadence: String,
    modulation_hint: Option<String>,
}

impl From<&PhraseMetadata> for PhraseMetadataView {
    fn from(meta: &PhraseMetadata) -> Self {
        Self {
            name: meta.name.clone(),
            start_bar: meta.start_bar,
            length: meta.length,
            cadence: cadence_label(meta.cadence).into(),
            modulation_hint: meta.modulation_hint.clone(),
        }
    }
}

#[derive(Serialize)]
struct ReharmZoneView {
    start_bar: u16,
    end_bar: u16,
    risk: f32,
}

impl From<&ReharmZone> for ReharmZoneView {
    fn from(zone: &ReharmZone) -> Self {
        Self { start_bar: zone.start_bar, end_bar: zone.end_bar, risk: zone.risk }
    }
}

#[derive(Serialize)]
struct TemplateExportDocument {
    id: String,
    version: u16,
    bars: u16,
    phrases: Vec<PhraseMetadataView>,
    tension_curve: Vec<f32>,
    reharm_zones: Vec<ReharmZoneView>,
}

impl From<&SectionTemplate> for TemplateExportDocument {
    fn from(template: &SectionTemplate) -> Self {
        TemplateExportDocument {
            id: template.metadata.id.clone(),
            version: template.metadata.version,
            bars: template.bars,
            phrases: template.metadata.phrases.iter().map(PhraseMetadataView::from).collect(),
            tension_curve: template.tension_curve.clone(),
            reharm_zones: template.reharm_zones.iter().map(ReharmZoneView::from).collect(),
        }
    }
}

fn cadence_label(kind: CadenceKind) -> &'static str {
    match kind {
        CadenceKind::None => "none",
        CadenceKind::Half => "half",
        CadenceKind::Perfect => "perfect",
        CadenceKind::Deceptive => "deceptive",
        CadenceKind::Plagal => "plagal",
    }
}
