use std::collections::HashMap;

use scraper::Selector;
use serde::{Deserialize, Serialize};

use crate::Session;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Folder {
    name: String,
    id: u64,
    index: u64,
}

enum Action {
    Delete(u32),
    Save(String),
    AutoSort,
    Reorder(Vec<Folder>),
}
impl Session {
    pub async fn folder_info(&self) -> anyhow::Result<Vec<Folder>> {
        let html = self
            .get_html("https://upload.e-hentai.org/managefolders")
            .await?;

        let selector = Selector::parse("#t >tbody > tr").unwrap();
        Ok(html
            .select(&selector)
            .rev()
            .skip(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .skip(1)
            .map(|v| {
                let mut children = v.child_elements();
                let name = children.next().unwrap().child_elements().next().unwrap();
                let id = name
                    .attr("name")
                    .unwrap_or_default()
                    .replace("fn", "")
                    .parse()
                    .unwrap();
                let name = name.attr("value").unwrap_or_default().to_owned();
                let selected = children
                    .next()
                    .unwrap()
                    .child_elements()
                    .next()
                    .unwrap()
                    .child_elements()
                    .find(|v| v.attr("selected").is_some())
                    .unwrap()
                    .inner_html()
                    .parse();
                Folder {
                    name,
                    id,
                    index: selected.unwrap(),
                }
            })
            .collect())
    }

    pub async fn create_folder(&self, folder: &str) -> anyhow::Result<()> {
        self.folder_actions(Action::Save(folder.to_owned())).await
    }

    pub async fn delete_folder(&self, id: u32) -> anyhow::Result<()> {
        self.folder_actions(Action::Delete(id)).await
    }

    pub async fn auto_reorder_folder(&self) -> anyhow::Result<()> {
        self.folder_actions(Action::AutoSort).await
    }

    pub async fn reorder_folder(&self, info: Vec<Folder>) -> anyhow::Result<()> {
        self.folder_actions(Action::Reorder(info)).await
    }

    async fn folder_actions(&self, action: Action) -> anyhow::Result<()> {
        let url = "https://upload.e-hentai.org/managefolders";

        let mut form = HashMap::new();
        form.insert("do_autosort".to_owned(), "".to_owned());
        form.insert("do_delete".to_owned(), "".to_owned());
        form.insert("do_save".to_owned(), "".to_owned());
        form.insert("fname".to_owned(), "".to_owned());
        let mut info = None;
        if !matches!(action, Action::Reorder(_)) {
            info = Some(self.folder_info().await?);
        }

        match action {
            Action::Delete(del) => {
                form.insert("do_delete".to_owned(), del.to_string());
            }
            Action::Save(s) => {
                form.insert("fname".to_owned(), s);
                form.insert("do_save".to_owned(), "1".to_owned());
            }
            Action::AutoSort => {
                form.insert("do_save".to_owned(), "1".to_owned());
                form.insert("do_autosort".to_owned(), "1".to_owned());
            }
            Action::Reorder(folder) => {
                form.insert("do_save".to_owned(), "1".to_owned());
                info = Some(folder);
            }
        }

        for f in info.unwrap().iter() {
            form.insert(format!("fn{}", f.id), f.name.to_string());
            form.insert(format!("fs{}", f.id), f.index.to_string());
        }

        self.form(url, &form).await?;
        Ok(())
    }
}
