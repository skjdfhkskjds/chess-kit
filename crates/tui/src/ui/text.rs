/// sanitize converts terminal control characters into visible text.
///
/// UCI engine identity, diagnostics, and raw protocol lines are untrusted
/// process output. Escaping control characters prevents them from reaching a
/// terminal backend as active escape or control sequences.
///
/// @param: value - untrusted text
/// @return: terminal-safe visible text
pub(super) fn sanitize(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\u{1b}' => output.push_str("\\x1b"),
            character if character.is_control() => {
                output.push_str("\\u{");
                output.push_str(&u32::from(character).to_string());
                output.push('}');
            }
            character => output.push(character),
        }
    }
    output
}
