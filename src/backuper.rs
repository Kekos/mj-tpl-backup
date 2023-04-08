use crate::client::MjClient;
use crate::mj_domain::Template;
use std::fs;
use std::path::PathBuf;

pub fn backup(mj_client: &MjClient, path: PathBuf) {
    for template in mj_client.get_templates().unwrap().data {
        backup_template(mj_client, &template, &path);
    }
}

fn backup_template(mj_client: &MjClient, template: &Template, path: &PathBuf) {
    for content in mj_client.get_template_content(template.id).unwrap().data {
        let mut file_path = path.clone();

        if let Some(html) = content.html_part {
            let mut html_path = file_path.clone();
            html_path.push(format!("{}.html", template.id));

            if let Err(error) = fs::write(html_path.clone(), html) {
                panic!(
                    "Could not write backup file `{}`: {}",
                    html_path.to_str().unwrap(),
                    error
                );
            }
        }

        if let Some(text) = content.text_part {
            let mut txt_path = file_path.clone();
            txt_path.push(format!("{}.txt", template.id));

            if let Err(error) = fs::write(txt_path.clone(), text) {
                panic!(
                    "Could not write backup file `{}`: {}",
                    txt_path.to_str().unwrap(),
                    error
                );
            }
        }
    }
}
