use scraper::{ElementRef, Html, Selector};

/// Find elements by CSS selector and filter by text content
pub fn select_contains<'a>(
    document: &'a Html,
    css_selector: &str,
    text_contains: &str,
) -> Vec<ElementRef<'a>> {
    let selector = Selector::parse(css_selector).expect("Invalid CSS selector");
    document
        .select(&selector)
        .filter(|el| el.text().any(|text| text.contains(text_contains)))
        .collect()
}

pub fn get_next_sibling<'a>(element: ElementRef<'a>) -> Option<ElementRef<'a>> {
    let mut next = element.next_sibling();

    while let Some(node) = next {
        if let Some(sibling_el) = ElementRef::wrap(node) {
            return Some(sibling_el);
        }
        next = node.next_sibling(); // Skip non-element nodes
    }

    None
}
