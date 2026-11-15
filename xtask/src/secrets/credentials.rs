use std::sync::LazyLock;

use regex::Regex;
use yaml_rust2::{Yaml, YamlLoader};

pub(super) type Match = (usize, usize, &'static str);

fn literal(value: &str) -> bool {
    static TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"\A(?:\$[A-Za-z_][A-Za-z0-9_]*|\$\{[A-Za-z_][A-Za-z0-9_]*\}|\$\([^()\r\n]+\)|\$?\{\{[^{}\r\n]+\}\}|<[^<>\r\n]+>|%[A-Za-z_][A-Za-z0-9_]*%)\z",
        )
        .expect("placeholder regex")
    });
    let value = value.trim();
    let lower = value.to_ascii_lowercase();
    !value.is_empty()
        && !TEMPLATE.is_match(value)
        && ![
            "password",
            "secret",
            "token",
            "placeholder",
            "example",
            "changeme",
            "redacted",
            "your_password",
            "your-password",
            "xxxxx",
            "*****",
            "...",
        ]
        .contains(&lower.as_str())
}

fn yaml_credentials(node: &Yaml, user: bool) -> bool {
    match node {
        Yaml::Hash(map) => map.iter().any(|(key, value)| {
            let key = key.as_str().unwrap_or("");
            let credential = key == "client-key-data"
                || (user
                    && [
                        "token",
                        "password",
                        "id-token",
                        "refresh-token",
                        "access-token",
                        "client-secret",
                    ]
                    .contains(&key));
            (credential && value.as_str().is_some_and(literal))
                || yaml_credentials(value, user || key == "user")
        }),
        Yaml::Array(items) => items.iter().any(|item| yaml_credentials(item, user)),
        _ => false,
    }
}

fn pgpass(line: &str) -> Option<Vec<String>> {
    if line.starts_with('#') {
        return None;
    }
    let mut fields = vec![String::new()];
    let mut chars = line.chars();
    while let Some(character) = chars.next() {
        if character == ':' && fields.len() < 5 {
            fields.push(String::new());
        } else {
            let character = if character == '\\' {
                chars.next()?
            } else {
                character
            };
            fields.last_mut().expect("one field").push(character);
        }
    }
    (fields.len() == 5).then_some(fields)
}

struct Tokens<'a> {
    text: &'a str,
    offset: usize,
}

impl Tokens<'_> {
    fn next(&mut self) -> Option<(usize, String)> {
        let bytes = self.text.as_bytes();
        loop {
            while bytes.get(self.offset).is_some_and(u8::is_ascii_whitespace) {
                self.offset += 1;
            }
            if bytes.get(self.offset) != Some(&b'#') {
                break;
            }
            while bytes.get(self.offset).is_some_and(|byte| *byte != b'\n') {
                self.offset += 1;
            }
        }
        if self.offset == bytes.len() {
            return None;
        }
        let start = self.offset;
        let quoted = bytes[self.offset] == b'"';
        self.offset += usize::from(quoted);
        let mut value = String::new();
        while let Some(character) = self.text[self.offset..].chars().next() {
            if !quoted && character.is_ascii_whitespace() {
                break;
            }
            self.offset += character.len_utf8();
            if quoted && character == '"' {
                return Some((start, value));
            }
            if character == '\\' {
                let escaped = self.text[self.offset..].chars().next()?;
                self.offset += escaped.len_utf8();
                value.push(match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    other => other,
                });
            } else {
                value.push(character);
            }
        }
        (!quoted).then_some((start, value))
    }

    fn skip_macro(&mut self) {
        let rest = &self.text[self.offset..];
        self.offset += rest.find('\n').map_or(rest.len(), |length| length + 1);
        while self.offset < self.text.len() {
            let rest = &self.text[self.offset..];
            let length = rest.find('\n').map_or(rest.len(), |length| length + 1);
            self.offset += length;
            if rest[..length].trim().is_empty() {
                break;
            }
        }
    }
}

fn netrc(text: &str, found: &mut Vec<Match>) {
    let mut tokens = Tokens { text, offset: 0 };
    let mut record = None;
    while let Some((start, token)) = tokens.next() {
        match token.as_str() {
            "machine" => record = tokens.next().map(|_| start),
            "default" => record = Some(start),
            "login" | "account" if record.is_some() => {
                tokens.next();
            }
            "password" if record.is_some() => {
                if let Some((_, value)) = tokens.next()
                    && literal(&value)
                {
                    found.push((
                        record.unwrap_or(start),
                        tokens.offset,
                        "local-netrc-credentials",
                    ));
                }
            }
            "macdef" if record.is_some() => {
                tokens.next();
                tokens.skip_macro();
            }
            _ => record = None,
        }
    }
}

fn markdown_value<'a>(prefix: &str, text: &'a str) -> &'a str {
    static BACKTICKS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"`+").expect("Markdown delimiter regex"));
    let mut delimiter = None;
    for matched in BACKTICKS.find_iter(prefix.rsplit('\n').next().unwrap_or(prefix)) {
        match delimiter {
            None => delimiter = Some(matched.len()),
            Some(length) if length == matched.len() => delimiter = None,
            _ => {}
        }
    }
    if let Some(length) = delimiter
        && let Some(closing) = BACKTICKS.find_iter(text).find(|matched| {
            matched.len() == length
                && text[matched.end()..].chars().next().is_none_or(|next| {
                    next.is_whitespace()
                        || [',', '.', ';', ':', ')', ']', '}', '!', '?'].contains(&next)
                })
        })
    {
        &text[..closing.start()]
    } else {
        text
    }
}

fn assignment_value(text: &str) -> (String, usize) {
    let mut chars = text.char_indices();
    let Some((_, first)) = chars.next() else {
        return (String::new(), 0);
    };
    let end_quote = match first {
        '{' if !text.starts_with("{{") => Some('}'),
        '\'' | '"' => Some(first),
        _ => None,
    };
    if let Some(quote) = end_quote {
        let mut value = String::new();
        let mut chars = chars.peekable();
        while let Some((offset, character)) = chars.next() {
            if character == quote {
                if chars.peek().is_some_and(|(_, next)| *next == quote) {
                    chars.next();
                } else {
                    return (value, offset + character.len_utf8());
                }
            }
            value.push(character);
        }
        (value, text.len())
    } else {
        let length = text
            .find(|character: char| ['\r', '\n', ';', '&', '\'', '"'].contains(&character))
            .unwrap_or(text.len());
        (text[..length].trim().to_owned(), length)
    }
}

fn percent_decode(text: &str) -> String {
    let mut bytes = Vec::new();
    let mut offset = 0;
    while offset < text.len() {
        if text.as_bytes()[offset] == b'%'
            && let Some(pair) = text.get(offset + 1..offset + 3)
            && let Ok(byte) = u8::from_str_radix(pair, 16)
        {
            bytes.push(byte);
            offset += 3;
            continue;
        }
        bytes.push(text.as_bytes()[offset]);
        offset += 1;
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

pub(super) fn scan(text: &str) -> Vec<Match> {
    static AUTH: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?i)\bauthorization["']?[ \t]*:[ \t]*["']?(?:basic|bearer)[ \t]+((?:\$?\{\{[^{}\r\n]+\}\}|[^\s"'`])+)"#)
            .expect("header regex")
    });
    static ASSIGNMENT: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(?i)\b([a-z_][a-z0-9_.-]*)["']?[ \t]*=[ \t]*"#).expect("assignment regex")
    });
    static JDBC: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(?i)\bjdbc:[^\s"'`]+"#).expect("JDBC regex"));
    let mut found = Vec::new();
    if (text.contains("client-key-data") || text.contains("user"))
        && let Ok(documents) = YamlLoader::load_from_str(text)
        && documents
            .iter()
            .any(|document| yaml_credentials(document, false))
    {
        let id = if text.contains("client-key-data") {
            "local-kubeconfig-client-key"
        } else {
            "local-kubeconfig-user-credential"
        };
        found.push((0, text.len(), id));
    }
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if let Some(fields) = pgpass(line.trim_end_matches(['\r', '\n']))
            && (fields[1] == "*" || fields[1].parse::<u16>().is_ok())
            && !fields[0].contains(['\'', '"', '`'])
            && (fields[0].starts_with('/') || !fields[0].contains(char::is_whitespace))
            && literal(&fields[4])
        {
            found.push((
                offset,
                offset + line.trim_end_matches(['\r', '\n']).len(),
                "local-pgpass-credentials",
            ));
        }
        offset += line.len();
    }
    if text.contains("machine") || text.contains("default") {
        netrc(text, &mut found);
    }
    for capture in AUTH.captures_iter(text) {
        let value = capture[1].trim_end_matches(r"\n").trim_end_matches(r"\r");
        if literal(value) {
            let matched = capture.get(0).expect("whole match");
            found.push((matched.start(), matched.end(), "local-http-authorization"));
        }
    }
    for matched in JDBC.find_iter(text) {
        if let Some((_, query)) = matched.as_str().split_once('?') {
            for parameter in query.split(['&', ';']) {
                if let Some((key, value)) = parameter.split_once('=')
                    && ["password", "pwd"]
                        .iter()
                        .any(|name| name.eq_ignore_ascii_case(&percent_decode(key)))
                    && literal(&percent_decode(value))
                {
                    found.push((matched.start(), matched.end(), "local-jdbc-password"));
                }
            }
        }
    }
    for capture in ASSIGNMENT.captures_iter(text) {
        let key = capture[1].to_ascii_lowercase();
        if ["password", "pwd", "pass", "passphrase"].contains(&key.as_str())
            || ["_pass", "_pwd", "_password", "_passphrase"]
                .iter()
                .any(|suffix| key.ends_with(suffix))
        {
            let matched = capture.get(0).expect("whole match");
            let value_text = markdown_value(&text[..matched.start()], &text[matched.end()..]);
            let (value, length) = assignment_value(value_text);
            if literal(&percent_decode(&value)) {
                found.push((
                    matched.start(),
                    matched.end() + length,
                    "local-connection-password",
                ));
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{secrets::Ruleset, util::Ctx};

    fn check(text: &str, expected: Option<&str>) {
        static RULES: LazyLock<Ruleset> = LazyLock::new(|| {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
            Ruleset::load(&Ctx::local(&root)).expect("rules")
        });
        let hits = RULES.scan("docs/notes.txt", text.as_bytes());
        if let Some(id) = expected {
            assert!(
                hits.iter().any(|hit| hit.message.contains(id)),
                "{text:?}: {hits:?}"
            );
        } else {
            assert!(hits.is_empty(), "{text:?}: {hits:?}");
        }
    }

    #[test]
    fn r15_kubeconfig_yaml_scalar_forms_and_placeholders() {
        for key in [
            "client-key-data",
            "token",
            "password",
            "id-token",
            "refresh-token",
            "access-token",
            "client-secret",
        ] {
            for style in ["|", "|-", "|+", ">", ">-", ">2-"] {
                for newline in ["\n", "\r\n"] {
                    for (value, detected) in [
                        ("c2Vuc2l0aXZlLWtleS1ieXRlcw==", true),
                        ("${VAR}", false),
                        ("<password>", false),
                    ] {
                        let text = format!("apiVersion: v1\nkind: Config\nusers:\n- name: admin\n  user:\n    {key}: {style}\n      {value}\n").replace('\n', newline);
                        let expected = detected.then_some(if key == "client-key-data" {
                            "local-kubeconfig-client-key"
                        } else {
                            "local-kubeconfig-user-credential"
                        });
                        check(&text, expected);
                    }
                }
            }
            for value in ["s3cr3t!", "${VAR}", "<password>"] {
                let json = serde_json::json!({"users": [{"user": {key: value}}]}).to_string();
                check(
                    &json,
                    (value == "s3cr3t!").then_some(if key == "client-key-data" {
                        "local-kubeconfig-client-key"
                    } else {
                        "local-kubeconfig-user-credential"
                    }),
                );
            }
        }
    }

    #[test]
    fn r15_pgpass_grammar_and_placeholders() {
        for prefix in [
            "postgres.prod.internal:5432:energy_grid:grid_user",
            "*:*:*:*",
            r"prod\:host:5432:db\:name:user name",
            r"localhost:5432:db:user\\name",
        ] {
            for password in [
                "my database password",
                "s3cr3t!",
                r"escaped\:colon\\slash",
                "p$a$$word",
                "${VAR}",
                "<password>",
            ] {
                let text = format!("{prefix}:{password}\r\n");
                check(
                    &text,
                    literal(password).then_some("local-pgpass-credentials"),
                );
            }
        }
        check("host:5432:database:user\n", None);
        check("ordinary prose: not a credential record\n", None);
        check(
            &format!(
                "#{}:{}:{}:{}:{}\n",
                "localhost", "5432", "db", "user", "s3cr3t!"
            ),
            None,
        );
    }

    #[test]
    fn r15_netrc_grammar_and_placeholders() {
        for prefix in [
            "machine energy-api.example.com",
            "default",
            "machine \"host with spaces\" login \"service user\"",
            "machine host\naccount service",
        ] {
            for password in [
                "s3cr3t!",
                "\"my database password\"",
                "\"escaped \\\"quote\\\" \\\\slash\"",
                "${VAR}",
                "\"<password>\"",
            ] {
                let text = format!("{prefix}\npassword {password}\n");
                check(
                    &text,
                    (!password.contains("VAR") && !password.contains("<password>"))
                        .then_some("local-netrc-credentials"),
                );
            }
        }
        check(
            "machine host login service\ndefault login anonymous\n",
            None,
        );
        check("machine learning is not password syntax\n", None);
        check(
            &format!(
                "machine host\nmacdef init\npassword {}\n\nlogin anonymous\n",
                "s3cr3t!"
            ),
            None,
        );
        check(
            &format!(
                "machine host\nmacdef init\nignored\n\npassword {}\n",
                "s3cr3t!"
            ),
            Some("local-netrc-credentials"),
        );
    }

    #[test]
    fn r15_http_authorization_literals_and_placeholders() {
        for scheme in ["Basic", "Bearer", "bEaReR"] {
            for value in [
                "dXNlcjpzM2NyM3Qh",
                "s3cr3t!",
                "${VAR}",
                "<password>",
                "$TOKEN",
                "{{ token }}",
            ] {
                check(
                    &format!("Authorization: {scheme} {value}\r\n"),
                    literal(value).then_some("local-http-authorization"),
                );
                check(
                    &format!("{{\"Authorization\": \"{scheme} {value}\"}}"),
                    literal(value).then_some("local-http-authorization"),
                );
            }
        }
        check("Authorization: negotiate\n", None);
    }

    #[test]
    fn r15_jdbc_password_query_and_placeholders() {
        for scheme in ["postgresql", "mysql", "sqlserver"] {
            for key in ["password", "PASSWORD", "pwd", "pass%77ord"] {
                for value in [
                    "s3cr3t!",
                    "p%40ss%20word",
                    "${VAR}",
                    "<password>",
                    "%24%7BVAR%7D",
                    "%3Cpassword%3E",
                ] {
                    check(
                        &format!("jdbc:{scheme}://localhost/db?user=grid&{key}={value}\n"),
                        literal(&percent_decode(value)).then_some("local-jdbc-password"),
                    );
                }
            }
        }
        check("jdbc:postgresql://localhost/db?user=grid&ssl=true\n", None);
    }

    #[test]
    fn r15_connection_strings_env_keys_and_placeholders() {
        for key in [
            "Password",
            "Pwd",
            "DB_PASS",
            "DB_PWD",
            "PWD",
            "PASSPHRASE",
            "DB_PASSWORD",
            "SSL_PASSPHRASE",
        ] {
            for value in [
                "s3cr3t!",
                "'my password'",
                "\"semi;colon\"",
                "{semi;colon}}brace}",
                "${VAR}",
                "<password>",
                "{<password>}",
                "\"${VAR}\"",
                "{{password}}",
                "%PASSWORD%",
            ] {
                let (parsed, _) = assignment_value(value);
                let text = format!("Server=localhost;User ID=grid;{key}={value};Database=grid\n");
                check(
                    &text,
                    literal(&parsed).then_some("local-connection-password"),
                );
                check(
                    &format!("export {key}={value}\n"),
                    literal(&parsed).then_some("local-connection-password"),
                );
            }
        }
        check(
            "Server=localhost;Database=grid;Integrated Security=true\n",
            None,
        );
    }

    #[test]
    fn r15v2_01_unquoted_multiword_connection_password() {
        let prefix = "Server=database.internal;Uid=operator;Pwd";
        for value in [
            "secret meadow2026",
            "<secret> meadow2026",
            "... meadow2026",
            "...`meadow2026",
            "<secret>`meadow2026",
            "ordinary meadow2026",
        ] {
            for ending in ["", ";", "\n", "\r\n", ";Database=grid"] {
                check(
                    &format!("{prefix}={value}{ending}"),
                    Some("local-connection-password"),
                );
            }
            check(
                &format!("`{prefix}={value};`"),
                Some("local-connection-password"),
            );
        }
    }

    #[test]
    fn r15v2_unquoted_assignment_consumes_the_complete_value() {
        for value in [
            "secret meadow2026",
            "<secret> meadow2026",
            "... meadow2026",
            "...`meadow2026",
            "<secret>`meadow2026",
            "realvalue`",
            "realvalue...",
            "real multiword password",
        ] {
            for ending in ["", ";Database=grid", "\nnext line", "\r\nnext line"] {
                let (parsed, length) = assignment_value(&format!("{value}{ending}"));
                assert_eq!(parsed, value);
                assert_eq!(length, value.len());
                assert!(literal(&parsed));
            }
        }
    }

    #[test]
    fn r15v2_placeholder_classification_is_anchored() {
        for template in [
            "<secret>",
            "...",
            "${VAR}",
            "$VAR",
            "$(command)",
            "{{password}}",
            "${{ secrets.PASSWORD }}",
            "%PASSWORD%",
            "secret",
        ] {
            assert!(!literal(template), "{template:?}");
            for value in [
                format!("{template} meadow2026"),
                format!("meadow2026 {template}"),
                format!("{template}-meadow2026"),
                format!("meadow2026-{template}"),
                format!("{template}`meadow2026"),
            ] {
                assert!(literal(&value), "{value:?}");
            }
        }
    }

    #[test]
    fn r15v2_markdown_connection_placeholder_controls() {
        let key = "Pwd";
        for delimiter in ["`", "``", "```"] {
            for template in ["<secret>", "...", "${VAR}", "${{ secrets.PASSWORD }}"] {
                for wrapper in [("", ""), ("\"", "\""), ("'", "'"), ("{", "}")] {
                    let escaped = if wrapper.0 == "{" {
                        template.replace('}', "}}")
                    } else {
                        template.to_owned()
                    };
                    let value = format!("{}{escaped}{}", wrapper.0, wrapper.1);
                    for ending in [".", ",", ";", ")", "", "\nMore documentation."] {
                        check(
                            &format!("Use {delimiter}{key}={value}{delimiter}{ending}"),
                            None,
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn r15v2_literal_assignment_boundaries() {
        let key = "Pwd";
        for value in ["realvalue`", "realvalue...", "real multiword password"] {
            check(&format!("{key}={value}"), Some("local-connection-password"));
            check(
                &format!("Use `{key}={value}` in a fixture."),
                Some("local-connection-password"),
            );
        }
        check(
            &format!("Use ``{key}=...`meadow2026`` in a fixture."),
            Some("local-connection-password"),
        );
    }

    #[test]
    fn r15v2_whole_value_placeholders_only() {
        let key = "Pwd";
        let header = "Authorization";
        for template in [
            "<secret>",
            "...",
            "${VAR}",
            "$VAR",
            "$(command)",
            "{{password}}",
            "${{ secrets.PASSWORD }}",
            "%PASSWORD%",
            "secret",
        ] {
            check(&format!("{header}: Bearer {template}"), None);
            for value in [
                format!("{template}-meadow2026"),
                format!("meadow2026{template}"),
            ] {
                check(
                    &format!("{header}: Bearer {value}"),
                    Some("local-http-authorization"),
                );
            }
            for wrapper in ["", "\"", "'"] {
                let value = format!("{wrapper}{template}{wrapper}");
                check(&format!("{key}={value};"), None);
                check(&format!("Use `{key}={value}` in examples."), None);
                check(&format!("Use ``{key}={value}`` in examples."), None);
                for literal in [
                    format!("{template}-meadow2026"),
                    format!("meadow2026{template}"),
                    format!("{template} meadow2026"),
                    format!("{template}meadow2026{template}"),
                ] {
                    check(
                        &format!("{key}={wrapper}{literal}{wrapper};"),
                        Some("local-connection-password"),
                    );
                }
            }
        }
    }

    #[test]
    fn r15v2_bare_authority_password_boundary() {
        let authority = ["user:", "realpass", "@host"].concat();
        for text in [
            authority.clone(),
            format!("Use `{authority}` in a fixture."),
        ] {
            check(&text, Some("local-bare-authority-credentials"));
        }
        let template = ["user:", "password", "@host"].concat();
        check(&format!("Use `{template}` in examples."), None);
    }

    #[test]
    fn documentation_authorization_placeholders_and_descriptions() {
        for scheme in ["Basic", "Bearer"] {
            for value in ["<token>", "<EXAMPLE_TOKEN>", "<EXAMPLE_BASE64>", "..."] {
                check(&format!("Use `Authorization: {scheme} {value}`."), None);
                for newline in [r"\n", r"\r\n"] {
                    check(
                        &format!("printf 'Authorization: {scheme} {value}{newline}'"),
                        None,
                    );
                }
            }
            check(
                &format!("Rule descriptions mention `Authorization: {scheme}` headers."),
                None,
            );
            check(
                &format!("Use `Authorization: {scheme} s3cr3t!` instead."),
                Some("local-http-authorization"),
            );
            check(
                &format!("Use `Authorization: {scheme} <token>s3cr3t!` instead."),
                Some("local-http-authorization"),
            );
            check(
                &format!(r"printf 'Authorization: {scheme} s3cr3t!\n'"),
                Some("local-http-authorization"),
            );
        }
        check("`Authorization: Bearer` and `Authorization: Basic`", None);
        check("Authorization: Bearer\nordinary prose\n", None);
        let header = "Authorization";
        check(
            &format!("`{header}: Bearer <token>`; `{header}: Bearer s3cr3t!`"),
            Some("local-http-authorization"),
        );
    }

    #[test]
    fn documentation_connection_placeholders_and_descriptions() {
        for key in ["Password", "Pwd", "DB_PASS", "DB_PWD", "PASSPHRASE"] {
            for value in ["<secret>", "<EXAMPLE_PASSWORD>", "...", "\"...\"", "{...}"] {
                check(
                    &format!("The `{key}={value}` attribute is documented."),
                    None,
                );
            }
            check(&format!("Describe `{key}=` without a value."), None);
            check(
                &format!("A literal `{key}=s3cr3t!` remains forbidden."),
                Some("local-connection-password"),
            );
            check(
                &format!("`{key}=<secret>` and `{key}=s3cr3t!`"),
                Some("local-connection-password"),
            );
        }
        let key = "Password";
        let abbreviation = "Pwd";
        check(
            &format!("Parameters `{key}=...`, `{abbreviation}=...` are examples."),
            None,
        );
        check(
            &format!("`{key}=`, `{abbreviation}=` name the attributes."),
            None,
        );
        check(
            &format!(
                "`{key}=<secret>` or `{abbreviation}=<secret>` (e.g., `Server=...;{abbreviation}=...;`)."
            ),
            None,
        );
    }

    #[test]
    fn documentation_jdbc_placeholders_and_literals() {
        let url = "jdbc:postgresql://host/db";
        let key = "password";
        for value in [
            "<password>",
            "<EXAMPLE_PASSWORD>",
            "%3Cpassword%3E",
            "...",
            "%2E%2E%2E",
        ] {
            check(&format!("Use `{url}?{key}={value}` in examples."), None);
        }
        check(
            &format!("`jdbc:<driver>://<host>:<port>/<database>?user=<user>&{key}=<password>`"),
            None,
        );
        check(
            &format!("Use `{url}?{key}=s3cr3t!` in a fixture."),
            Some("local-jdbc-password"),
        );
        check(
            &format!("`{url}?{key}=<password>` and `{}=s3cr3t!`", "DB_PASS"),
            Some("local-connection-password"),
        );
    }

    #[test]
    fn documentation_uri_templates_are_narrowly_exempted() {
        let separator = "://";
        for template in [
            format!("scheme{separator}user:password@host"),
            format!("<scheme>{separator}user:password@host"),
            format!("postgres{separator}<user>:<password>@host/db"),
            format!("postgres{separator}user:...@host/db"),
        ] {
            check(
                &format!("The `{template}` template describes URI credentials."),
                None,
            );
        }
        for literal in [
            format!("scheme{separator}user:s3cr3t!@host"),
            format!("scheme{separator}user:password@production.example"),
            format!("postgres{separator}user:password@host"),
            format!("scheme{separator}other:password@host"),
            format!("scheme{separator}user:password@host.example"),
            format!("postgres{separator}user:...s3cr3t!@host/db"),
        ] {
            check(
                &format!("A literal `{literal}` is forbidden."),
                Some("local-uri-credentials"),
            );
        }
        check(
            &format!(
                "`scheme{separator}user:password@host` and `postgres{separator}user:s3cr3t!@host/db`"
            ),
            Some("local-uri-credentials"),
        );
    }
}
