use std::path::PathBuf;

use codegen_schema::manifest::{Manifest, ManifestSection, ManifestTopic};

use crate::{markdown::MarkdownWriter, navigation::NavigationEntry};

pub fn generate_reference_dir(grammar: &Manifest, repo_root: &PathBuf) -> NavigationEntry {
    let mut sections = Vec::<NavigationEntry>::new();

    for section in &grammar.sections {
        let mut topics = Vec::<NavigationEntry>::new();

        for topic in &section.topics {
            topics.push(NavigationEntry::Page {
                title: topic.title.to_owned(),
                path: topic.path.to_owned(),
                contents: generate_topic_page(grammar, section, topic, repo_root),
            });
        }

        sections.push(NavigationEntry::Directory {
            title: section.title.to_owned(),
            path: section.path.to_owned(),
            children: topics,
        });
    }

    return NavigationEntry::Directory {
        title: "Reference".to_owned(),
        path: "reference".to_owned(),
        children: sections,
    };
}

fn generate_topic_page(
    grammar: &Manifest,
    section: &ManifestSection,
    topic: &ManifestTopic,
    repo_root: &PathBuf,
) -> String {
    let mut page = MarkdownWriter::new();

    page.write_header(1, &topic.title);

    page.write_newline();
    page.write_snippet(
        repo_root,
        &grammar
            .source_location
            .clone()
            .unwrap()
            .path
            .parent()
            .unwrap()
            .join(&section.path)
            .join(&topic.path)
            .join("notes.md"),
    );

    return page.to_string();
}
