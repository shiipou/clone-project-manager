use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub fn add_project(
    target_path: PathBuf,
    workspace: PathBuf,
    host: String,
    group: String,
    name: String,
    debug: bool,
) {
    let categories_path = target_path.with_file_name("categories.json");
    let categories_data = fs::read_to_string(&categories_path).unwrap_or_else(|_| "[]".to_string());
    let mut categories: Vec<Value> =
        serde_json::from_str(&categories_data).expect("Categories JSON was not well-formed");
    let (category_id, categories_changed) = ensure_category_path(&mut categories, &host, &group);

    let projects_data = fs::read_to_string(&target_path).unwrap_or_else(|_| "[]".to_string());
    let mut projects: Vec<Value> =
        serde_json::from_str(&projects_data).expect("Projects JSON was not well-formed");
    let project_changed = upsert_project(&mut projects, &workspace, &name, category_id);

    if categories_changed {
        write_json(&categories_path, &categories);
    }
    if project_changed {
        write_json(&target_path, &projects);
    }

    if debug {
        let action = if project_changed {
            "Project added or updated"
        } else {
            "Project already exists"
        };
        println!("{} {:?} in category {}.", action, workspace, category_id);
    }
}

fn ensure_category_path(categories: &mut Vec<Value>, host: &str, group: &str) -> (u64, bool) {
    let mut parent_id = 0;
    let mut changed = false;

    for category_name in std::iter::once(host)
        .chain(group.split('/'))
        .filter(|name| !name.is_empty())
    {
        let existing_id = categories.iter().find_map(|category| {
            let same_parent = category.get("_parentId").and_then(Value::as_u64) == Some(parent_id);
            let same_name = category.get("_name").and_then(Value::as_str) == Some(category_name);
            (same_parent && same_name)
                .then(|| category.get("_id").and_then(Value::as_u64))
                .flatten()
        });

        if let Some(category_id) = existing_id {
            parent_id = category_id;
            continue;
        }

        let category_id = categories
            .iter()
            .filter_map(|category| category.get("_id").and_then(Value::as_u64))
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        categories.push(json!({
            "_id": category_id,
            "_parentId": parent_id,
            "_name": category_name,
            "children": []
        }));
        parent_id = category_id;
        changed = true;
    }

    (parent_id, changed)
}

fn upsert_project(
    projects: &mut Vec<Value>,
    workspace: &Path,
    name: &str,
    category_id: u64,
) -> bool {
    let project_path = workspace.to_string_lossy();
    if let Some(project) = projects
        .iter_mut()
        .find(|project| project.get("_path").and_then(Value::as_str) == Some(&project_path))
    {
        let mut changed = false;
        if project.get("_name").and_then(Value::as_str) != Some(name) {
            project["_name"] = json!(name);
            changed = true;
        }
        if project.get("_categoryId").and_then(Value::as_u64) != Some(category_id) {
            project["_categoryId"] = json!(category_id);
            changed = true;
        }
        return changed;
    }

    let next_id = projects
        .iter()
        .filter_map(|project| project.get("_id").and_then(Value::as_u64))
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    projects.push(json!({
        "_id": next_id,
        "_path": project_path,
        "_name": name,
        "_categoryId": category_id
    }));
    true
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    let updated_data = serde_json::to_string_pretty(value).expect("Failed to serialize JSON");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Unable to create project manager directory");
    }
    fs::write(path, updated_data).expect("Unable to write file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reuses_the_existing_parcours_emploi_category_tree() {
        let mut categories = vec![
            json!({ "_id": 3, "_parentId": 1, "_name": "mobile-factory", "children": [] }),
            json!({ "_id": 4, "_parentId": 3, "_name": "mes-offres-mobile", "children": [] }),
            json!({ "_id": 5, "_parentId": 3, "_name": "transverse-mobile", "children": [] }),
            json!({ "_id": 6, "_parentId": 5, "_name": "packages", "children": [] }),
            json!({ "_id": 7, "_parentId": 0, "_name": "git-scm.pole-emploi.intra", "children": [] }),
            json!({ "_id": 1, "_parentId": 7, "_name": "france_travail", "children": [] }),
        ];

        let (category_id, changed) = ensure_category_path(
            &mut categories,
            "git-scm.pole-emploi.intra",
            "france_travail/mobile-factory/mes-offres-mobile",
        );

        assert_eq!(category_id, 4);
        assert!(!changed);
        assert_eq!(categories.len(), 6);
    }

    #[test]
    fn creates_missing_categories_and_assigns_existing_project_to_leaf() {
        let mut categories = vec![];
        let (category_id, changed) =
            ensure_category_path(&mut categories, "github.com", "owner/team");
        let mut projects = vec![json!({
            "_id": 1,
            "_path": "/work/project",
            "_name": "project",
            "_categoryId": 0
        })];

        assert!(changed);
        assert_eq!(category_id, 3);
        assert!(upsert_project(
            &mut projects,
            Path::new("/work/project"),
            "project",
            category_id
        ));
        assert_eq!(projects[0]["_categoryId"], category_id);
        assert_eq!(categories[1]["_parentId"], 1);
        assert_eq!(categories[2]["_parentId"], 2);
    }
}
