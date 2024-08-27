use zed_extension_api as zed;
use std::sync::Arc;

impl JustifyTextExtension {
    fn new() -> Self {
        JustifyTextExtension {}
    }

    fn justify_text(&self, text: &str, line_width: usize) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut result = String::new();
        let mut line = Vec::new();
        let mut line_length = 0;

        for word in words {
            if line_length + word.len() + line.len() > line_width {
                if !line.is_empty() {
                    let spaces_to_add = line_width - line_length;
                    let gaps = line.len() - 1;
                    if gaps > 0 {
                        let space_per_gap = spaces_to_add / gaps;
                        let extra_spaces = spaces_to_add % gaps;

                        for (i, word) in line.iter().enumerate() {
                            result.push_str(word);
                            if i < gaps {
                                let spaces = if i < extra_spaces {
                                    space_per_gap + 1
                                } else {
                                    space_per_gap
                                };
                                result.push_str(&" ".repeat(spaces));
                            }
                        }
                    } else {
                        result.push_str(&line[0]);
                        result.push_str(&" ".repeat(spaces_to_add));
                    }
                    result.push('\n');
                }
                line.clear();
                line_length = 0;
            }

            line.push(word);
            line_length += word.len();
        }

        if !line.is_empty() {
            result.push_str(&line.join(" "));
            result.push_str(&" ".repeat(line_width - line_length));
        }

        result
    }
}

impl zed::Extension for JustifyTextExtension {
    fn init(self: Arc<Self>, cx: &mut zed::ExtensionContext) {
        cx.add_command(
            "justify_text",
            zed::Command::new(
                move |cx: &mut zed::CommandContext| {
                    if let Some(editor) = cx.active_editor() {
                        if let Some(selections) = editor.selections() {
                            let mut new_selections = Vec::new();
                            for selection in selections.iter() {
                                if let Some(range) = selection.range() {
                                    if let Some(text) = editor.text(range) {
                                        let justified_text = self.justify_text(&text, 80);  // Default to 80 chars width
                                        editor.edit([(range, justified_text)]);
                                        new_selections.push(zed::Selection::point(range.end));
                                    }
                                }
                            }
                            editor.set_selections(new_selections);
                        }
                    }
                    Ok(())
                }
            )
        );
    }
}

zed::register_extension!(JustifyTextExtension::new());
