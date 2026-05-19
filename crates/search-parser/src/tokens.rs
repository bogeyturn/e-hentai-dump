pub fn parse_tokens(input: &str) -> Vec<(String, bool)> {
    let mut items = vec![];
    let mut builder = vec![];
    let mut quoted = false;
    for char in input.chars() {
        if char == '"' {
            if builder.last() == Some(&'\\') {
                builder.pop();
                builder.push(char);
                continue;
            }
            quoted = !quoted;
            if !quoted {
                items.push((builder.drain(..).collect::<String>(), true));
            }
            continue;
        }
        if char == ' ' && !quoted {
            if builder.is_empty() {
                continue;
            }
            items.push((builder.drain(..).collect::<String>(), false));
            continue;
        }
        builder.push(char);
    }
    items.push((builder.drain(..).collect::<String>(), false));
    items
}
