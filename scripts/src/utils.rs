use case::CaseExt;

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

pub fn snake(s: &str) -> String { escape(s).to_snake() }

// ex. CDPSessionSTORE :-> CdpSessionStore
pub fn loud_to_camel(s: &str) -> String {
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

pub fn loud_to_snake(s: &str) -> String { snake(&loud_to_camel(s)) }

pub fn lower_loud_to_camel(s: &str) -> String { loud_to_camel(&s).to_camel() }

pub fn kebab_to_snake(s: &str) -> String {
    let u = s.replace("-", "_");
    let snake = if u.starts_with("_") {
        format!("neg{}", u)
    } else {
        u
    };
    snake
}

pub fn kebab_to_camel(s: &str) -> String { kebab_to_snake(s).to_camel() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_loud_to_camel() {
        assert_eq!(loud_to_camel("CDPSession"), "CdpSession".to_string());
        assert_eq!(loud_to_camel("JSHandle"), "JsHandle".to_string());
        assert_eq!(
            loud_to_camel("APIRequestContext"),
            "ApiRequestContext".to_string()
        );
        assert_eq!(loud_to_camel("AXNode"), "AxNode".to_string());
        assert_eq!(loud_to_camel("AxisXADog"), "AxisXaDog".to_string());
        assert_eq!(loud_to_camel("AXIS"), "Axis".to_string()); // if last
    }
}
