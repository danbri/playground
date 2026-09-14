#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color { Black, White, Red, Blue, Green }

impl Color {
    fn name(self) -> &'static str {
        match self { Self::Black => "black", Self::White => "white", Self::Red => "red", Self::Blue => "blue", Self::Green => "green" }
    }
}

#[derive(Clone, Debug)]
pub struct BoxStyle {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub padding: u32,
    pub background: Color,
    pub color: Color,
}

impl Default for BoxStyle {
    fn default() -> Self { Self { width: None, height: None, padding: 0, background: Color::White, color: Color::Black } }
}

#[derive(Clone, Debug)]
pub enum Node { Box(BoxStyle, Vec<Node>), Text(String) }

#[derive(Clone, Copy, Debug)]
pub struct Environment { pub viewport_width: u32, pub glyph_width: u32, pub line_height: u32 }
impl Default for Environment { fn default() -> Self { Self { viewport_width: 120, glyph_width: 8, line_height: 16 } } }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DisplayItem {
    Rect { x: u32, y: u32, w: u32, h: u32, color: Color },
    Text { x: u32, y: u32, text: String, color: Color },
}

#[derive(Clone, Debug)]
struct LaidOut { width: u32, height: u32, items: Vec<DisplayItem> }

fn wrap_text(env: Environment, width: u32, s: &str) -> Vec<String> {
    let limit = (width / env.glyph_width.max(1)).max(1) as usize;
    let mut lines: Vec<String> = Vec::new();
    for word in s.split(' ').filter(|w| !w.is_empty()) {
        match lines.last_mut() {
            Some(last) if last.len() + 1 + word.len() <= limit => {
                last.push(' '); last.push_str(word);
            }
            _ => lines.push(word.to_owned()),
        }
    }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

fn fault() -> Option<String> { std::env::var("STAGE_MINUS_ONE_FAULT").ok() }

fn layout_node(env: Environment, node: &Node, x: u32, y: u32, available_width: u32, inherited_color: Color) -> LaidOut {
    match node {
        Node::Text(s) => {
            let mut lines = wrap_text(env, available_width, s);
            if fault().as_deref() == Some("drop_text") { lines = vec![String::new()]; }
            let items = lines.iter().enumerate().map(|(i, line)| DisplayItem::Text {
                x, y: y + i as u32 * env.line_height, text: line.clone(), color: inherited_color
            }).collect::<Vec<_>>();
            LaidOut { width: available_width, height: lines.len() as u32 * env.line_height, items }
        }
        Node::Box(style, children) => {
            let outer_width = style.width.unwrap_or(available_width);
            let inner_width = outer_width.saturating_sub(2 * style.padding);
            let mut cursor_y = y + style.padding;
            let mut child_items = Vec::new();
            for child in children {
                let child_y = if fault().as_deref() == Some("wrong_sibling_y") && !child_items.is_empty() { cursor_y.saturating_sub(1) } else { cursor_y };
                let laid = layout_node(env, child, x + style.padding, child_y, inner_width, style.color);
                cursor_y += laid.height;
                child_items.extend(laid.items);
            }
            let content_height = cursor_y - (y + style.padding);
            let natural_height = content_height + 2 * style.padding;
            let outer_height = style.height.unwrap_or(natural_height);
            let mut w = outer_width;
            if fault().as_deref() == Some("off_by_one_width") { w = w.saturating_add(1); }
            let bg = DisplayItem::Rect { x, y, w, h: outer_height, color: style.background };
            let mut items = Vec::with_capacity(1 + child_items.len());
            if fault().as_deref() == Some("wrong_paint_order") { child_items.push(bg); items = child_items; }
            else { items.push(bg); items.extend(child_items); }
            LaidOut { width: outer_width, height: outer_height, items }
        }
    }
}

pub fn render(env: Environment, doc: &Node) -> Vec<DisplayItem> {
    layout_node(env, doc, 0, 0, env.viewport_width, Color::Black).items
}

pub fn fixture(name: &str) -> Option<Node> {
    match name {
        "nested" => Some(Node::Box(BoxStyle { width: Some(120), padding: 10, background: Color::Red, color: Color::Black, ..Default::default() }, vec![
            Node::Text("hello world hello world".into()),
            Node::Box(BoxStyle { width: Some(80), height: Some(20), background: Color::Blue, color: Color::White, ..Default::default() }, vec![]),
            Node::Text("tail".into()),
        ])),
        "inherit" => Some(Node::Box(BoxStyle { width: Some(96), padding: 8, background: Color::Green, color: Color::White, ..Default::default() }, vec![
            Node::Text("alpha beta gamma delta".into()),
        ])),
        _ => None,
    }
}

pub fn render_text(env: Environment, doc: &Node) -> String {
    render(env, doc).into_iter().map(|item| match item {
        DisplayItem::Rect { x, y, w, h, color } => format!("rect {x} {y} {w} {h} {}", color.name()),
        DisplayItem::Text { x, y, text, color } => format!("text {x} {y} {} |{text}|", color.name()),
    }).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrapping_moves_following_sibling() {
        let out = render_text(Environment::default(), &fixture("nested").unwrap());
        assert!(out.contains("text 10 10 black |hello world|"));
        assert!(out.contains("text 10 26 black |hello world|"));
        assert!(out.contains("rect 10 42 80 20 blue"));
        assert!(out.contains("text 10 62 black |tail|"));
    }
    #[test]
    fn inherited_color_reaches_text() {
        let out = render_text(Environment::default(), &fixture("inherit").unwrap());
        assert!(out.contains("white |alpha beta|"));
    }
}
