//! Human-evidence gates (NFR-3.AC3/AC5, ADR 0001), checked automatically.
//!
//! Both gates run from `main` (`.github/workflows/trusted-gates.yml`, a
//! `pull_request_target` workflow that never builds or runs pull-request
//! code) and compare against the current tip of `main`, never the event's
//! possibly stale base (F1, ADV23R-08).
//!
//! `independent-review` passes only when the trusted evidence tree (`main`)
//! holds, for commits of this pull request:
//! - an approving review whose author and reviewer role/vendor pair is
//!   allowed by the trusted matrix (`xtask/policy/review.toml`): the vendors
//!   differ, the roles are compatible, and Gemini never approves
//!   (F6, ADV23R-07);
//! - at least one adversarial report with a valid header; and
//! - a triage record (GPT or coordinator) that gives every finding ID of every
//!   adversarial report on this pull request a closing disposition.
//!
//! The approving review and the triage record must still be current: between
//! the commit they name and the head, no tree entry (blob or mode) that the
//! pull request owns may change, except review reports (F6).
//!
//! `policy-approval` (R1, D-74): when a pull request changes a protected policy
//! path (including every workflow and action), the policy owner's latest review
//! must be APPROVED on the exact head. GitHub never lets an author approve
//! their own pull request, so a policy change opened by the owner cannot pass:
//! agents open pull requests from the separate machine account. The gate also
//! rejects any pull-request workflow job that reuses a trusted gate's name.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use toml_edit::{DocumentMut, Item};
use yaml_rust2::{Yaml, YamlLoader};

use crate::util::{Ctx, Finding, git, glob_match, is_sha, string_array};

const REVIEWS: &str = "docs/requirements/reviews/";
/// Trusted review matrix.
pub const MATRIX: &str = "xtask/policy/review.toml";
/// Trusted repository policy.
pub const REPOSITORY: &str = "xtask/policy/repository.toml";

/// A `role / provider` header field.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Party {
    /// Normalised role, e.g. `code generation`.
    pub role: String,
    /// The provider text after the last `/`.
    pub provider: String,
}

/// Parsed machine-checked review header.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewHeader {
    /// Reviewed commit.
    pub head: Option<String>,
    /// Verdict word, lower case.
    pub verdict: Option<String>,
    /// Author role and provider.
    pub author: Option<Party>,
    /// Reviewer role and provider.
    pub reviewer: Option<Party>,
    /// Triager role and provider (triage records only).
    pub triager: Option<Party>,
    /// Report a triage record dispositions (triage records only).
    pub triaged: Option<String>,
}

fn field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let cleaned = line.trim_start_matches(['-', '*', ' ', '_']);
    let rest = cleaned.strip_prefix(name)?;
    let rest = rest.trim_start_matches(['*', '_']).strip_prefix(':')?;
    Some(rest.trim().trim_start_matches(['*', '_']).trim())
}

fn normalise_words(text: &str) -> String {
    text.split(|c: char| !(c.is_alphanumeric() || c == '-'))
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join(" ")
}

fn party(value: &str) -> Option<Party> {
    let (role, provider) = value.rsplit_once('/')?;
    let role = normalise_words(role);
    let provider = provider.trim().trim_end_matches('.').trim().to_owned();
    (!role.is_empty() && !provider.is_empty()).then_some(Party { role, provider })
}

/// Parses the header fields of a review report (first 40 lines).
#[must_use]
pub fn parse_header(text: &str) -> ReviewHeader {
    let mut header = ReviewHeader::default();
    for line in text.lines().take(40) {
        if let Some(value) = field(line, "Reviewed head") {
            let sha: String = value
                .trim_matches(['`', ' ', '.'])
                .chars()
                .take(40)
                .collect();
            if is_sha(&sha) {
                header.head = Some(sha);
            }
        } else if let Some(value) = field(line, "Verdict") {
            let word: String = value
                .trim_start_matches(['*', '_', '`'])
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '-')
                .collect();
            header.verdict = Some(word.to_lowercase());
        } else if let Some(value) = field(line, "Author role/provider") {
            header.author = party(value);
        } else if let Some(value) = field(line, "Reviewer role/provider") {
            header.reviewer = party(value);
        } else if let Some(value) = field(line, "Triager role/provider") {
            header.triager = party(value);
        } else if let Some(value) = field(line, "Triaged report") {
            header.triaged = Some(value.trim_matches(['`', ' ', '.']).to_owned());
        }
    }
    header
}

/// One allowed independent-review pairing.
#[derive(Debug, Clone, Default)]
struct Pairing {
    author_roles: Vec<String>,
    author_vendors: Vec<String>,
    reviewer_roles: Vec<String>,
    reviewer_vendors: Vec<String>,
}

/// The trusted review matrix.
#[derive(Debug, Clone, Default)]
pub struct Matrix {
    vendors: BTreeMap<String, Vec<String>>,
    independent: Vec<Pairing>,
    adversarial_roles: Vec<String>,
    adversarial_vendors: Vec<String>,
    triage_roles: Vec<String>,
    triage_vendors: Vec<String>,
    dispositions: Vec<String>,
}

impl Matrix {
    /// Parses the matrix.
    ///
    /// # Errors
    ///
    /// Fails on malformed TOML or a missing section.
    pub fn parse(text: &str) -> Result<Self, String> {
        let document: DocumentMut = text.parse().map_err(|e| format!("{MATRIX}: {e}"))?;
        let list = |item: Option<&Item>| item.map(string_array).unwrap_or_default();
        let vendors: BTreeMap<String, Vec<String>> = document
            .get("vendors")
            .and_then(Item::as_table_like)
            .ok_or(format!("{MATRIX}: [vendors] missing"))?
            .iter()
            .map(|(key, value)| (key.to_owned(), string_array(value)))
            .collect();
        let independent = document
            .get("independent")
            .and_then(Item::as_array_of_tables)
            .ok_or(format!("{MATRIX}: [[independent]] missing"))?
            .iter()
            .map(|table| Pairing {
                author_roles: list(table.get("author_roles")),
                author_vendors: list(table.get("author_vendors")),
                reviewer_roles: list(table.get("reviewer_roles")),
                reviewer_vendors: list(table.get("reviewer_vendors")),
            })
            .collect();
        let section = |name: &str, key: &str| list(document.get(name).and_then(|t| t.get(key)));
        let matrix = Self {
            vendors,
            independent,
            adversarial_roles: section("adversarial", "reviewer_roles"),
            adversarial_vendors: section("adversarial", "reviewer_vendors"),
            triage_roles: section("triage", "roles"),
            triage_vendors: section("triage", "vendors"),
            dispositions: section("triage", "dispositions"),
        };
        if matrix.adversarial_roles.is_empty()
            || matrix.triage_roles.is_empty()
            || matrix.dispositions.is_empty()
        {
            return Err(format!(
                "{MATRIX}: adversarial and triage sections are required"
            ));
        }
        Ok(matrix)
    }

    /// The single vendor a provider names, if exactly one.
    #[must_use]
    pub fn vendor(&self, provider: &str) -> Option<String> {
        let words: BTreeSet<String> = provider
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|word| !word.is_empty())
            .map(str::to_ascii_lowercase)
            .collect();
        let found: Vec<&String> = self
            .vendors
            .iter()
            .filter(|(_, names)| names.iter().any(|name| words.contains(name)))
            .map(|(vendor, _)| vendor)
            .collect();
        match found.as_slice() {
            [vendor] => Some((*vendor).clone()),
            _ => None,
        }
    }

    fn party_vendor(&self, party: Option<&Party>) -> Option<(String, String)> {
        let party = party?;
        Some((party.role.clone(), self.vendor(&party.provider)?))
    }

    /// Why an approving header is not acceptable independent review, if so.
    fn independent_problem(&self, header: &ReviewHeader) -> Option<String> {
        let Some((author_role, author_vendor)) = self.party_vendor(header.author.as_ref()) else {
            return Some("author role/provider missing or names no single vendor".to_owned());
        };
        let Some((reviewer_role, reviewer_vendor)) = self.party_vendor(header.reviewer.as_ref())
        else {
            return Some("reviewer role/provider missing or names no single vendor".to_owned());
        };
        if author_vendor == reviewer_vendor {
            return Some(format!("author and reviewer are both {author_vendor}"));
        }
        let allowed = self.independent.iter().any(|pairing| {
            pairing.author_roles.contains(&author_role)
                && pairing.author_vendors.contains(&author_vendor)
                && pairing.reviewer_roles.contains(&reviewer_role)
                && pairing.reviewer_vendors.contains(&reviewer_vendor)
        });
        (!allowed).then(|| {
            format!(
                "{author_role}/{author_vendor} reviewed by {reviewer_role}/{reviewer_vendor} is not an allowed pairing ({MATRIX})"
            )
        })
    }

    fn is_adversarial_role(&self, header: &ReviewHeader) -> bool {
        header
            .reviewer
            .as_ref()
            .is_some_and(|party| self.adversarial_roles.contains(&party.role))
    }

    fn adversarial_valid(&self, header: &ReviewHeader) -> bool {
        self.party_vendor(header.reviewer.as_ref())
            .is_some_and(|(role, vendor)| {
                self.adversarial_roles.contains(&role) && self.adversarial_vendors.contains(&vendor)
            })
    }

    fn triager_valid(&self, header: &ReviewHeader) -> bool {
        self.party_vendor(header.triager.as_ref())
            .is_some_and(|(role, vendor)| {
                self.triage_roles.contains(&role) && self.triage_vendors.contains(&vendor)
            })
    }
}

const APPROVING: &[&str] = &["approve", "approved", "merge-ready"];

/// Finding IDs of an adversarial report: the first token of each level-3+
/// heading when it looks like `ADV23R-01` (letters/digits, a hyphen, digits).
#[must_use]
pub fn finding_ids(text: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for line in text.lines() {
        let level = line.chars().take_while(|c| *c == '#').count();
        if level < 3 || !line[level..].starts_with(' ') {
            continue;
        }
        let rest = line[level..].trim().trim_start_matches(['*', '`']);
        let rest = rest
            .split_once(". ")
            .filter(|(number, _)| number.chars().all(|c| c.is_ascii_digit()))
            .map_or(rest, |(_, after)| after);
        let token: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        let looks_like_id = token.starts_with(|c: char| c.is_ascii_uppercase())
            && token.contains('-')
            && token.ends_with(|c: char| c.is_ascii_digit());
        if looks_like_id {
            found.insert(token);
        }
    }
    found
}

/// Dispositions in a triage record's tables: first cell ID, second cell the
/// disposition's first word, lower case.
#[must_use]
pub fn dispositions(text: &str) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    for line in text.lines() {
        let cells: Vec<&str> = line.trim().trim_matches('|').split('|').collect();
        let [id, disposition, ..] = cells.as_slice() else {
            continue;
        };
        let id = id.trim().trim_matches(['*', '`', ' ']);
        let word: String = disposition
            .trim()
            .trim_matches(['*', '`', '_'])
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        if !id.is_empty() && !word.is_empty() {
            found.insert(id.to_owned(), word.to_ascii_lowercase());
        }
    }
    found
}

/// Evaluates review evidence.
///
/// `reports` are (path, text) from the trusted tree; `commits` are the pull
/// request's commits; `current(sha)` says whether nothing the pull request
/// owns has changed (blob or mode) between `sha` and the head.
#[must_use]
pub fn evaluate(
    reports: &[(String, String)],
    commits: &BTreeSet<String>,
    current: &dyn Fn(&str) -> bool,
    matrix: &Matrix,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut notes = Vec::new();
    let mut approved = false;
    let mut adversarial: Vec<(&str, BTreeSet<String>)> = Vec::new();
    let mut valid_adversarial = false;
    let mut triages = Vec::new();
    for (path, text) in reports {
        let header = parse_header(text);
        if header.triager.is_some() {
            triages.push((path.as_str(), text.as_str(), header));
            continue;
        }
        let cites_pr = header.head.as_ref().is_some_and(|h| commits.contains(h))
            || (header.head.is_none()
                && text
                    .split(|c: char| !c.is_ascii_hexdigit())
                    .any(|word| commits.contains(word)));
        if !cites_pr {
            continue;
        }
        if matrix.is_adversarial_role(&header) || path.contains("adversarial") {
            if matrix.adversarial_valid(&header) {
                valid_adversarial = true;
            }
            adversarial.push((path.as_str(), finding_ids(text)));
            continue;
        }
        let Some(head) = &header.head else { continue };
        let verdict_ok = header
            .verdict
            .as_deref()
            .is_some_and(|v| APPROVING.contains(&v));
        let problem = matrix.independent_problem(&header);
        let still_current = current(head);
        if verdict_ok && problem.is_none() && still_current {
            approved = true;
        } else {
            notes.push(format!(
                "{path}: reviews {} but verdict={:?}, pairing={}, still current={still_current}",
                &head[..12],
                header.verdict.as_deref().unwrap_or("none"),
                problem.as_deref().unwrap_or("ok"),
            ));
        }
    }
    if !approved {
        findings.push(Finding::general(
            "no approving, role-compatible independent review (different vendor) covers the current head",
        ));
        findings.extend(notes.into_iter().map(Finding::general));
    }
    if !valid_adversarial {
        findings.push(Finding::general(
            "no adversarial report with a valid header (adversarial review / Google) cites a commit of this PR",
        ));
    }
    for (path, ids) in &adversarial {
        let complete = triages.iter().any(|(_, text, header)| {
            let covers = header
                .triaged
                .as_deref()
                .map(|t| t.trim_start_matches("./"))
                == Some(*path);
            let head_ok = header
                .head
                .as_ref()
                .is_some_and(|h| commits.contains(h) && current(h));
            let table = dispositions(text);
            covers
                && head_ok
                && matrix.triager_valid(header)
                && ids.iter().all(|id| {
                    table
                        .get(id)
                        .is_some_and(|d| matrix.dispositions.contains(d))
                })
        });
        if !complete {
            let open: Vec<&String> = ids.iter().collect();
            findings.push(Finding::general(format!(
                "{path}: no current triage record (GPT/coordinator) closes every finding {open:?}"
            )));
        }
    }
    findings
}

/// Whether the tree entry (mode, type, object) of every path in `paths` is
/// the same at `a` and `b`.
fn same_entries(repo: &Path, a: &str, b: &str, path: &str) -> bool {
    let entry = |rev: &str| {
        git(
            repo,
            &[
                "ls-tree",
                "-z",
                "--full-tree",
                rev,
                "--",
                &format!(":(literal){path}"),
            ],
        )
        .ok()
        .map(|out| {
            let text = String::from_utf8_lossy(&out).into_owned();
            text.split('\t').next().unwrap_or_default().to_owned()
        })
    };
    entry(a) == entry(b)
}

fn changed_paths(repo: &Path, from: &str, to: &str) -> Result<BTreeSet<String>, String> {
    Ok(String::from_utf8_lossy(&git(
        repo,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            from,
            to,
        ],
    )?)
    .split('\0')
    .filter(|name| !name.is_empty())
    .map(str::to_owned)
    .collect())
}

/// Whether a review of `reviewed` still covers `head`: every path changed
/// between them either has the same tree entry (blob AND mode) or is no
/// longer changed by the pull request (`main`'s version, after a merge of
/// `main`). No directory is exempt: a file under the reviews directory can be
/// compiled or included like any other, so evidence lands on `main`, never on
/// the pull-request branch (R3-03).
///
/// # Errors
///
/// Fails if Git fails.
pub fn still_current(repo: &Path, main: &str, reviewed: &str, head: &str) -> Result<bool, String> {
    let merge_base = String::from_utf8_lossy(&git(repo, &["merge-base", main, head])?)
        .trim()
        .to_owned();
    for path in changed_paths(repo, reviewed, head)? {
        if !same_entries(repo, reviewed, head, &path)
            && !same_entries(repo, &merge_base, head, &path)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Runs the independent-review gate. `main` is the current tip of `main`
/// (also the evidence tree); `head` is the pull-request head.
///
/// # Errors
///
/// Fails if Git fails, a revision is invalid or the trusted matrix is bad.
pub fn independent_review(ctx: &Ctx, main: &str, head: &str) -> Result<Vec<Finding>, String> {
    let matrix = Matrix::parse(&ctx.policy_text(MATRIX)?)?;
    let repo = ctx.repo.as_path();
    if !is_sha(head) {
        return Err("independent-review: head must be a full SHA".to_owned());
    }
    let main = String::from_utf8_lossy(&git(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            "--end-of-options",
            &format!("{main}^{{commit}}"),
        ],
    )?)
    .trim()
    .to_owned();
    let commits: BTreeSet<String> =
        String::from_utf8_lossy(&git(repo, &["rev-list", &format!("{main}..{head}")])?)
            .split_whitespace()
            .map(str::to_owned)
            .collect();
    let listing = git(
        repo,
        &["ls-tree", "-r", "-z", "--name-only", &main, "--", REVIEWS],
    )?;
    let mut reports = Vec::new();
    for name in String::from_utf8_lossy(&listing).split('\0') {
        if name.ends_with(".md") {
            let text = git(repo, &["show", &format!("{main}:{name}")])?;
            reports.push((name.to_owned(), String::from_utf8_lossy(&text).into_owned()));
        }
    }
    let current = |sha: &str| still_current(repo, &main, sha, head).unwrap_or(false);
    println!(
        "evidence main {}: {} report(s); PR has {} commit(s)",
        &main[..12],
        reports.len(),
        commits.len()
    );
    Ok(evaluate(&reports, &commits, &current, &matrix))
}

/// One pull-request review: (login, state, commit SHA), in submission order.
pub type Review = (String, String, String);

/// Who is involved in an approval decision.
#[derive(Debug, Clone, Copy)]
pub struct People<'a> {
    /// Policy owner login (trusted policy).
    pub owner: &'a str,
    /// Agent machine account login (trusted policy, D-74).
    pub agent: &'a str,
    /// Pull-request author login (event).
    pub author: &'a str,
}

/// Evaluates owner approval for protected-path changes (R1, F2, D-74).
#[must_use]
pub fn evaluate_approval(
    changed: &[String],
    protected: &[String],
    people: People<'_>,
    head: &str,
    reviews: &[Review],
) -> Vec<Finding> {
    let touched: Vec<&String> = changed
        .iter()
        .filter(|path| {
            protected
                .iter()
                .any(|pattern| glob_match(pattern.trim_start_matches('/'), path))
        })
        .collect();
    if touched.is_empty() {
        println!("no protected policy path changed");
        return Vec::new();
    }
    let short = &head[..head.len().min(12)];
    let mut findings = Vec::new();
    if people.author.eq_ignore_ascii_case(people.owner) {
        findings.push(Finding::general(format!(
            "protected policy paths changed in a pull request opened by the policy owner {}: GitHub does not let an author approve their own pull request, so this gate cannot pass. Open policy changes from the agent machine account ({}, D-74) and have {} approve head {short}",
            people.owner, people.agent, people.owner
        )));
    } else {
        let latest = reviews.iter().rfind(|(login, state, _)| {
            login.eq_ignore_ascii_case(people.owner) && state != "COMMENTED" && state != "PENDING"
        });
        if !matches!(latest, Some((_, state, commit)) if state == "APPROVED" && commit == head) {
            findings.push(Finding::general(format!(
                "protected policy paths changed; needs an APPROVED review by {} on head {short}, then a re-run of this check",
                people.owner
            )));
        }
    }
    if !findings.is_empty() {
        findings.extend(
            touched
                .into_iter()
                .map(|path| Finding::at(path.as_str(), None, "protected policy path")),
        );
    }
    findings
}

/// Jobs of pull-request workflows that reuse a trusted gate's job id or name,
/// which could make a spoofed status look like the trusted one (F1).
#[must_use]
pub fn spoofed_gate_names(
    workflows: &[(String, String)],
    trusted_files: &[String],
    trusted_jobs: &[String],
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (path, text) in workflows {
        if trusted_files.contains(path) {
            continue;
        }
        let Ok(documents) = YamlLoader::load_from_str(text) else {
            continue;
        };
        let Some(jobs) = documents
            .first()
            .and_then(|doc| doc.as_hash())
            .and_then(|hash| hash.get(&Yaml::String("jobs".into())))
            .and_then(Yaml::as_hash)
        else {
            continue;
        };
        for (id, job) in jobs {
            let id = id.as_str().unwrap_or_default();
            let name = job
                .as_hash()
                .and_then(|hash| hash.get(&Yaml::String("name".into())));
            // GitHub names the check run after `name:`, or after the job id
            // when `name:` is absent; check names compare case-insensitively
            // (ADV23R7-01). A computed name cannot be judged: fail closed.
            let name = match name {
                None => id,
                Some(value) => value.as_str().unwrap_or("${{"),
            };
            if name.contains("${{") {
                findings.push(Finding::at(
                    path.as_str(),
                    None,
                    format!("job `{id}` has a computed or non-string name; give it a literal name"),
                ));
                continue;
            }
            let same = |text: &str, gate: &str| text.trim().eq_ignore_ascii_case(gate.trim());
            if trusted_jobs
                .iter()
                .any(|gate| same(id, gate) || same(name, gate))
            {
                findings.push(Finding::at(
                    path.as_str(),
                    None,
                    format!("job `{id}` reuses the name of trusted gate; rename it"),
                ));
            }
        }
    }
    findings
}

/// Inputs to the policy-approval gate taken from the event.
#[derive(Debug, Clone)]
pub struct PullRequest {
    /// `owner/name`.
    pub repository: String,
    /// Pull-request number.
    pub number: u64,
    /// Head SHA.
    pub head: String,
    /// Author login.
    pub author: String,
}

impl PullRequest {
    /// Reads and validates the event fields.
    ///
    /// # Errors
    ///
    /// Fails on a missing or malformed field.
    pub fn from_event(event: &serde_json::Value) -> Result<Self, String> {
        let pull = &event["pull_request"];
        let head = pull["head"]["sha"].as_str().unwrap_or_default().to_owned();
        let number = pull["number"].as_u64().ok_or("event: no PR number")?;
        let author = pull["user"]["login"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let repository = event["repository"]["full_name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        let name_ok = |part: &str| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        };
        let valid_repo = repository
            .split_once('/')
            .is_some_and(|(a, b)| name_ok(a) && name_ok(b));
        if !(is_sha(&head) && valid_repo && name_ok(&author)) {
            return Err("policy-approval: invalid event fields".to_owned());
        }
        Ok(Self {
            repository,
            number,
            head,
            author,
        })
    }
}

/// Runs the policy-approval gate against the current tip of `main`.
///
/// # Errors
///
/// Fails on an invalid event, Git failure or GitHub API failure.
pub fn policy_approval(
    ctx: &Ctx,
    main: &str,
    pull: &PullRequest,
    trusted_files: &[String],
    trusted_jobs: &[String],
) -> Result<Vec<Finding>, String> {
    let document = ctx.policy_toml(REPOSITORY)?;
    let text = |key: &str| {
        document
            .get(key)
            .and_then(Item::as_str)
            .map(str::to_owned)
            .ok_or(format!("trusted policy: {key} missing"))
    };
    let owner = text("policy_owner")?;
    let agent = text("agent_account")?;
    let protected = string_array(&document["protected_paths"]);
    let repo = ctx.repo.as_path();
    let head = pull.head.as_str();
    let merge_base = String::from_utf8_lossy(&git(repo, &["merge-base", main, head])?)
        .trim()
        .to_owned();
    let changed: Vec<String> = changed_paths(repo, &merge_base, head)?
        .into_iter()
        .collect();
    let listing = git(
        repo,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            head,
            "--",
            ".github/workflows/",
        ],
    )?;
    let mut workflows = Vec::new();
    for name in String::from_utf8_lossy(&listing).split('\0') {
        if name.ends_with(".yml") || name.ends_with(".yaml") {
            let body = git(repo, &["show", &format!("{head}:{name}")])?;
            workflows.push((name.to_owned(), String::from_utf8_lossy(&body).into_owned()));
        }
    }
    let mut findings = spoofed_gate_names(&workflows, trusted_files, trusted_jobs);
    let reviews = if pull.author.eq_ignore_ascii_case(&owner) {
        Vec::new()
    } else {
        fetch_reviews(pull)?
    };
    findings.extend(evaluate_approval(
        &changed,
        &protected,
        People {
            owner: &owner,
            agent: &agent,
            author: &pull.author,
        },
        head,
        &reviews,
    ));
    Ok(findings)
}

fn fetch_reviews(pull: &PullRequest) -> Result<Vec<Review>, String> {
    let output = std::process::Command::new("gh")
        .args([
            "api",
            "--paginate",
            &format!("repos/{}/pulls/{}/reviews", pull.repository, pull.number),
            "--jq",
            ".[] | [.user.login, .state, .commit_id] | @tsv",
        ])
        .output()
        .map_err(|error| format!("gh: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "gh api failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            Some((
                parts.next()?.to_owned(),
                parts.next()?.to_owned(),
                parts.next()?.to_owned(),
            ))
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sha(c: char) -> String {
        std::iter::repeat_n(c, 40).collect()
    }

    fn matrix() -> Matrix {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("policy/review.toml");
        Matrix::parse(&std::fs::read_to_string(path).expect("matrix")).expect("parse")
    }

    fn report(head: &str, verdict: &str, author: &str, reviewer: &str) -> String {
        format!(
            "# Review\n\n- Author role/provider: {author}.\n- Reviewer role/provider: {reviewer}.\n- Reviewed head: `{head}`.\n- Verdict: **{verdict}**. Notes.\n"
        )
    }

    fn code_review(head: &str, verdict: &str) -> (String, String) {
        (
            "docs/requirements/reviews/gpt-x.md".into(),
            report(
                head,
                verdict,
                "code generation / Anthropic",
                "independent code review / OpenAI",
            ),
        )
    }

    const ADVERSARIAL: &str = "docs/requirements/reviews/gemini-x.md";

    fn adversarial(head: &str) -> (String, String) {
        (
            ADVERSARIAL.into(),
            format!(
                "{}\n### ADV9-01: first\n\n### ADV9-02: second\n",
                report(
                    head,
                    "must-fix",
                    "code generation / Anthropic",
                    "adversarial review / Google"
                )
            ),
        )
    }

    fn triage(head: &str, rows: &str, triager: &str) -> (String, String) {
        (
            "docs/requirements/reviews/triage-x.md".into(),
            format!(
                "# Triage\n\n- Triager role/provider: {triager}\n- Triaged report: `{ADVERSARIAL}`\n- Reviewed head: `{head}`\n\n| ID | Disposition | Reason |\n|---|---|---|\n{rows}"
            ),
        )
    }

    fn complete(head: &str) -> Vec<(String, String)> {
        vec![
            code_review(head, "approve"),
            adversarial(head),
            triage(
                head,
                "| ADV9-01 | **fixed** | test |\n| ADV9-02 | rejected | reason |\n",
                "triage / OpenAI",
            ),
        ]
    }

    // ADV23-10: the gate is not vacuous.
    #[test]
    fn adv23_10_review_gate_requires_evidence() {
        let commits: BTreeSet<String> = [sha('a'), sha('b')].into_iter().collect();
        let yes = |_: &str| true;
        let m = matrix();
        assert_eq!(evaluate(&[], &commits, &yes, &m).len(), 2);
        assert!(evaluate(&complete(&sha('a')), &commits, &yes, &m).is_empty());
        let mut must_fix = complete(&sha('a'));
        must_fix[0] = code_review(&sha('a'), "must-fix");
        assert!(!evaluate(&must_fix, &commits, &yes, &m).is_empty());
        let other_pr = complete(&sha('c'));
        assert!(!evaluate(&other_pr, &commits, &yes, &m).is_empty());
        let stale = |_: &str| false;
        assert!(!evaluate(&complete(&sha('a')), &commits, &stale, &m).is_empty());
    }

    // F6 / ADV23R-07: vendors, not model names, must differ, and roles must
    // be compatible; Gemini never supplies independent review.
    #[test]
    fn f6_adv23r_07_roles_and_vendors_are_checked() {
        let commits: BTreeSet<String> = [sha('a')].into_iter().collect();
        let yes = |_: &str| true;
        let m = matrix();
        for (author, reviewer) in [
            ("Anthropic / Claude-Sonnet", "Anthropic / Claude-Opus"),
            (
                "code generation / Claude Sonnet",
                "independent code review / Claude Opus",
            ),
            ("code generation / Anthropic", "adversarial review / Gemini"),
            (
                "code generation / Anthropic",
                "independent code review / Google",
            ),
            ("code generation / Anthropic", "code generation / OpenAI"),
            (
                "code generation / OpenAI and Anthropic",
                "independent code review / OpenAI",
            ),
            (
                "code generation / Unknown",
                "independent code review / OpenAI",
            ),
        ] {
            let mut reports = complete(&sha('a'));
            reports[0] = (
                "docs/requirements/reviews/r.md".into(),
                report(&sha('a'), "approve", author, reviewer),
            );
            let found = evaluate(&reports, &commits, &yes, &m);
            assert!(!found.is_empty(), "accepted {author} -> {reviewer}");
        }
        let mut reports = complete(&sha('a'));
        reports[0] = (
            "docs/requirements/reviews/r.md".into(),
            report(
                &sha('a'),
                "approve",
                "specification / GPT 6.1 Sol",
                "independent review / Claude Opus",
            ),
        );
        assert!(evaluate(&reports, &commits, &yes, &m).is_empty());
        assert_eq!(m.vendor("Claude-Sonnet").as_deref(), Some("anthropic"));
        assert_eq!(m.vendor("GPT 6.1 Sol").as_deref(), Some("openai"));
        assert_eq!(m.vendor("Gemini Flash 3.8").as_deref(), Some("google"));
    }

    // F6: adversarial findings need a current triage record by GPT or the
    // coordinator that closes every finding ID.
    #[test]
    fn f6_triage_is_required() {
        let commits: BTreeSet<String> = [sha('a')].into_iter().collect();
        let yes = |_: &str| true;
        let m = matrix();
        let head = sha('a');
        let without = complete(&head)[..2].to_vec();
        assert!(!evaluate(&without, &commits, &yes, &m).is_empty());
        for (rows, triager) in [
            ("| ADV9-01 | fixed | x |\n", "triage / OpenAI"),
            (
                "| ADV9-01 | fixed | x |\n| ADV9-02 | accepted | todo |\n",
                "triage / OpenAI",
            ),
            (
                "| ADV9-01 | fixed | x |\n| ADV9-02 | rejected | x |\n",
                "adversarial review / Gemini",
            ),
            (
                "| ADV9-01 | fixed | x |\n| ADV9-02 | rejected | x |\n",
                "triage / Anthropic",
            ),
        ] {
            let mut reports = complete(&head);
            reports[2] = triage(&head, rows, triager);
            assert!(
                !evaluate(&reports, &commits, &yes, &m).is_empty(),
                "accepted {triager}: {rows}"
            );
        }
        // A stale triage record does not count even when the review is current.
        let triage_stale = |sha: &str| sha != head;
        let mut reports = complete(&head);
        reports[0] = code_review(&head, "approve");
        assert!(!evaluate(&reports, &commits, &triage_stale, &m).is_empty());
        assert_eq!(
            finding_ids(
                "## 1. Summary of ADV23-01..18\n### ADV23R-01: x\n### 2. F2 — y\n#### ADV5-3 z\n"
            ),
            ["ADV23R-01".to_owned(), "ADV5-3".to_owned()]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn header_parsing_matches_existing_reports() {
        let text = "- Author role/provider: code generation / Anthropic.\n- Reviewer role/provider: independent code review / OpenAI.\n- Reviewed head: `bebd708a574940805854886130cbf8f15907b29b`.\n- Verdict: **must-fix**. R1–R8 below.\n";
        let header = parse_header(text);
        assert_eq!(
            header.head.as_deref(),
            Some("bebd708a574940805854886130cbf8f15907b29b")
        );
        assert_eq!(header.verdict.as_deref(), Some("must-fix"));
        let author = header.author.expect("author");
        assert_eq!(author.role, "code generation");
        assert_eq!(author.provider, "Anthropic");
        assert_eq!(
            header.reviewer.expect("reviewer").role,
            "independent code review"
        );
    }

    fn people<'a>(author: &'a str) -> People<'a> {
        People {
            owner: "Owner",
            agent: "egs-bot",
            author,
        }
    }

    // R1: protected policy changes need the owner's approval on the head.
    #[test]
    fn r1_protected_changes_need_owner_approval() {
        let protected = vec!["deny.toml".to_owned(), "xtask/".to_owned()];
        let head = sha('h');
        let changed = vec!["deny.toml".to_owned()];
        let review = |login: &str, state: &str, commit: &str| {
            (login.to_owned(), state.to_owned(), commit.to_owned())
        };
        let bot = people("egs-bot");
        assert!(evaluate_approval(&["src/x.rs".into()], &protected, bot, &head, &[]).is_empty());
        assert!(!evaluate_approval(&changed, &protected, bot, &head, &[]).is_empty());
        assert!(
            evaluate_approval(
                &changed,
                &protected,
                bot,
                &head,
                &[review("owner", "APPROVED", &head)]
            )
            .is_empty()
        );
        assert!(
            !evaluate_approval(
                &changed,
                &protected,
                bot,
                &head,
                &[review("owner", "APPROVED", &sha('o'))]
            )
            .is_empty()
        );
        assert!(
            !evaluate_approval(
                &changed,
                &protected,
                bot,
                &head,
                &[review("someone", "APPROVED", &head)]
            )
            .is_empty()
        );
        let dismissed = [
            review("owner", "APPROVED", &head),
            review("owner", "CHANGES_REQUESTED", &head),
            review("owner", "COMMENTED", &head),
        ];
        assert!(!evaluate_approval(&changed, &protected, bot, &head, &dismissed).is_empty());
        assert!(
            !evaluate_approval(&["xtask/src/main.rs".into()], &protected, bot, &head, &[])
                .is_empty()
        );
    }

    // F2 / D-74: a policy change opened by the owner can never pass, and the
    // finding explains why; the agent account's PR passes with owner approval.
    #[test]
    fn f2_owner_authored_policy_change_cannot_pass() {
        let protected = vec!["xtask/".to_owned()];
        let changed = vec!["xtask/src/main.rs".to_owned()];
        let head = sha('h');
        let approved = [("Owner".to_owned(), "APPROVED".to_owned(), head.clone())];
        let found = evaluate_approval(&changed, &protected, people("owner"), &head, &approved);
        assert!(!found.is_empty());
        assert!(found[0].message.contains("cannot pass"), "{found:?}");
        assert!(found[0].message.contains("egs-bot"), "{found:?}");
        assert!(
            evaluate_approval(&changed, &protected, people("egs-bot"), &head, &approved).is_empty()
        );
        // Unprotected changes by the owner need nothing.
        assert!(
            evaluate_approval(
                &["docs/a.md".into()],
                &protected,
                people("Owner"),
                &head,
                &[]
            )
            .is_empty()
        );
    }

    // F1: a pull-request workflow may not reuse a trusted gate's name.
    #[test]
    fn f1_spoofed_gate_names_are_rejected() {
        let trusted_files = vec![".github/workflows/trusted-gates.yml".to_owned()];
        let trusted_jobs = vec![
            "independent-review".to_owned(),
            "policy-approval".to_owned(),
        ];
        let spoof = vec![
            (
                ".github/workflows/ci.yml".to_owned(),
                "jobs:\n  policy-approval:\n    runs-on: ubuntu-24.04\n    steps: [{run: echo ok}]\n".to_owned(),
            ),
            (
                ".github/workflows/x.yml".to_owned(),
                "jobs:\n  fake:\n    name: Independent-Review\n    runs-on: ubuntu-24.04\n".to_owned(),
            ),
            (
                ".github/workflows/trusted-gates.yml".to_owned(),
                "jobs:\n  policy-approval:\n    runs-on: ubuntu-24.04\n".to_owned(),
            ),
        ];
        assert_eq!(
            spoofed_gate_names(&spoof, &trusted_files, &trusted_jobs).len(),
            2
        );
    }

    // ADV23R7-01: a job without `name:` is named by its id, compared
    // case-insensitively; a computed name fails closed.
    #[test]
    fn adv23r7_01_job_ids_are_compared_case_insensitively() {
        let trusted_files = vec![".github/workflows/trusted-gates.yml".to_owned()];
        let trusted_jobs = vec![
            "independent-review".to_owned(),
            "policy-approval".to_owned(),
        ];
        for job in [
            "  Policy-Approval:\n    runs-on: ubuntu-24.04\n",
            "  INDEPENDENT-REVIEW:\n    runs-on: ubuntu-24.04\n",
            "  build:\n    name: ' Policy-Approval '\n    runs-on: ubuntu-24.04\n",
            "  build:\n    name: ${{ 'policy-approval' }}\n    runs-on: ubuntu-24.04\n",
            "  build:\n    name: [policy-approval]\n    runs-on: ubuntu-24.04\n",
        ] {
            let spoof = vec![(
                ".github/workflows/spoof.yml".to_owned(),
                format!("name: spoof\non: [pull_request]\njobs:\n{job}"),
            )];
            assert_eq!(
                spoofed_gate_names(&spoof, &trusted_files, &trusted_jobs).len(),
                1,
                "{job}"
            );
        }
        // Control: other ids and names pass.
        let clean = vec![(
            ".github/workflows/ci.yml".to_owned(),
            "jobs:\n  Policy:\n    runs-on: ubuntu-24.04\n  x:\n    name: Review notes\n    runs-on: ubuntu-24.04\n".to_owned(),
        )];
        assert!(spoofed_gate_names(&clean, &trusted_files, &trusted_jobs).is_empty());
    }
}
