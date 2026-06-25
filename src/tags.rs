use std::collections::HashSet;

pub fn normalize_tags<I, S>(tags: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for raw in tags {
        for part in raw.as_ref().split(',') {
            let tag = part.trim();
            if tag.is_empty() {
                continue;
            }
            let key = tag.to_lowercase();
            if seen.insert(key) {
                out.push(tag.to_string());
            }
        }
    }

    out
}

pub fn tags_match(tags: &[String], filter: &[String]) -> bool {
    if filter.is_empty() {
        return true;
    }

    filter.iter().all(|wanted| {
        tags.iter()
            .any(|tag| tag.eq_ignore_ascii_case(wanted))
    })
}

pub fn format_tags(tags: &[String]) -> String {
    if tags.is_empty() {
        "-".to_string()
    } else {
        tags.join(", ")
    }
}
