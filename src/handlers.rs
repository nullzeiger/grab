use crate::app;
use crate::cli::{AddArgs, RemoveArgs, SearchArgs};
use crate::error::{GrabError, Result};
use crate::models::App;
use crate::ui;

pub fn handle_list() -> Result<()> {
    let apps = app::list_apps()?;

    if apps.is_empty() {
        println!("No apps stored yet. Use `grab add` to add one.");
        return Ok(());
    }

    println!();
    ui::print_apps(&apps);
    Ok(())
}

pub async fn handle_check() -> Result<()> {
    app::check_apps().await
}

pub async fn handle_download() -> Result<()> {
    app::download_apps(None).await
}

pub fn handle_add(args: AddArgs) -> Result<()> {
    let app = if args.interactive || args.are_any_fields_missing() {
        collect_app_interactively(args)?
    } else {
        args.into_app()?
    };

    app::add_app(app)?;
    println!("App added successfully!");
    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<()> {
    let apps = app::list_apps()?;
    let app_to_delete = apps.iter().find(|(idx, _)| *idx == args.index);

    match app_to_delete {
        Some((_, app)) => {
            println!("You are about to delete:\n  Name: {}", app.name);
            if ui::confirm_action("Are you sure you want to delete this app?")? {
                app::remove_app(args.index)?;
                println!("App deleted successfully!");
            } else {
                println!("Deletion cancelled.");
            }
            Ok(())
        }
        None => Err(GrabError::NotFound(format!(
            "No app found at index {}.",
            args.index
        ))),
    }
}

pub fn handle_search(args: SearchArgs) -> Result<()> {
    let query = args.query.filter(|q| !q.trim().is_empty()).ok_or_else(|| {
        GrabError::InvalidInput("Please provide a non-empty search query.".to_string())
    })?;

    let results = app::search_apps(&query)?;

    if results.is_empty() {
        println!("No apps found matching '{}'", query);
    } else {
        println!("Found {} app(s) matching '{}':\n", results.len(), query);
        ui::print_apps(&results);
    }

    Ok(())
}

fn collect_app_interactively(args: AddArgs) -> Result<App> {
    println!("Adding New App");

    let name = get_or_prompt(args.name, "App name")?;
    let owner = get_or_prompt(args.owner, "Repo owner")?;
    let repo = get_or_prompt(args.repo, "Repo name")?;
    let asset_pattern = get_or_prompt(args.asset_pattern, "Asset pattern")?;
    let version_flag = get_or_prompt(args.version_flag, "Version flag")?;

    println!();
    App::new(name, owner, repo, asset_pattern, version_flag)
}

fn get_or_prompt(field: Option<String>, label: &str) -> Result<String> {
    match field {
        Some(val) => Ok(val),
        None => Ok(ui::prompt_for_input(label, true)?),
    }
}
