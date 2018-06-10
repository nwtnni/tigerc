pub fn escape_str<'input>(s: &'input str) -> String {
    s[1..(s.len() - 1)].to_string()
}
