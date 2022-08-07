pub fn escape(s: &str) -> String {
    match s {
        // keywords https://doc.rust-lang.org/book/appendix-01-keywords.html
        "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else"
        | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "Self" | "self"
        | "static" | "struct" | "super" | "trait" | "true" | "type" | "union" | "unsafe"
        | "use" | "where" | "while" => {
            format!("r#{}", s)
        }
        // reserved
        "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "try"
        | "typeof" | "unsized" | "virtual" | "yield" => {
            format!("r#{}", s)
        }
        _ => s.into()
    }
}

// ex. CDPSessionSTORE :-> CdpSessionStore
pub fn fix_loud_camel(s: &str) -> String {
    let us = s
        .chars()
        .map(|c| c.is_ascii_uppercase())
        .collect::<Vec<_>>();
    if us.len() <= 1 {
        return s.to_ascii_uppercase();
    }
    let mut spans = Vec::<(usize, usize)>::new();
    let it = {
        let a = us.iter();
        let mut b = us.iter();
        b.next();
        a.zip(b)
    };
    let mut ctx = (false, 0);
    for (i, (cur, peek)) in it.enumerate() {
        match (ctx.0, cur, peek) {
            (false, false, false) => {}
            (false, false, true) => {}
            (false, true, false) => {}
            (false, true, true) => {
                ctx = (true, i);
            }
            (true, false, false) => {}
            (true, false, true) => {}
            (true, true, false) => {
                spans.push((ctx.1, i));
                ctx = (false, 0);
            }
            (true, true, true) => {}
        }
    }
    if ctx.0 {
        spans.push((ctx.1, us.len()));
    }
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if spans.iter().any(|&(start, stop)| start < i && i < stop) {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fix_loud_camel() {
        assert_eq!(fix_loud_camel("CDPSession"), "CdpSession".to_string());
        assert_eq!(fix_loud_camel("JSHandle"), "JsHandle".to_string());
        assert_eq!(
            fix_loud_camel("APIRequestContext"),
            "ApiRequestContext".to_string()
        );
        assert_eq!(fix_loud_camel("AXNode"), "AxNode".to_string());
        assert_eq!(fix_loud_camel("AxisXADog"), "AxisXaDog".to_string());
        assert_eq!(fix_loud_camel("AXIS"), "Axis".to_string()); // if last
    }
}
