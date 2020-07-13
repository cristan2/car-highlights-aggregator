pub fn is_hugo_markdown_title() -> () {
    unimplemented!()
}

pub fn is_section_title() -> () {
    unimplemented!()
}

pub fn is_sub_section_title() -> () {
    unimplemented!()
}

pub fn is_separator() -> () {
    unimplemented!()
}

pub fn make_markdown_url(url_text: &str, url_link: &str) -> String {
    format!("[{text}]({url})", text=url_text, url=url_link)
}