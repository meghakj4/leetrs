//! Problem picker — fetches questions, writes local files, and drives submission.
//!
//! [`Picker`] is the main orchestrator used by both the CLI commands (`pick`,
//! `submit`, `test`) and the TUI. It wraps [`LeetCodeClient`] and adds local
//! file I/O and a disk cache for the problem list.
use crate::cache::CacheService;
use crate::config::CONFIG;
use crate::error::EngineError;
use crate::format::format_result;
use crate::models::{Identifier, ProblemSummary, UserDetail};
use crate::services::submission::SubmissionService;
use crate::{client::LeetCodeClient, models::Language};
use std::path::Path;
use std::fs;

/// Orchestrates problem fetching, file generation, submission, and caching.
///
/// Constructed once per command invocation and shared across async tasks via
/// clone (the inner [`LeetCodeClient`] is `Clone`).
#[derive(Clone)]
pub struct Picker {
    pub client: LeetCodeClient,
}

impl Picker {
    pub fn new(client: LeetCodeClient) -> Self {
        Picker { client }
    }

    /// Resolves a problem by [`Identifier`], writes the Markdown description
    /// and language-specific code stub to disk, and returns their paths.
    ///
    /// If both files already exist on disk (slug-based match) they are returned
    /// immediately without hitting the network.
    ///
    /// # Returns
    /// `(code_file_path, description_file_path)` on success.
    pub async fn pick(
        &self,
        identifier: &Identifier,
        language: &Option<Language>,
    ) -> crate::error::Result<(String, String)> {
        let mut language = match language {
            Some(lang) => lang.clone(),
            None => {
                let config = CONFIG.get().expect("Config not initialised");
                if let Some(lang) = &config.language {
                    Language::from(lang)
                } else {
                    println!("🔤 No language specified, defaulting to Python.");
                    Language::Python
                }
            }
        };

        //TODO: If language is specified, must open that file
        // else open the file with matching slug.
        if let Identifier::String(ident) = identifier {
            let snake_slug = ident.replace("-", "_");
            let code_filename = format!("{}.{}", snake_slug, language.code_extension());
            let desc_filename = format!("{}.md", snake_slug);
            if Path::new(&code_filename).exists() && Path::new(&desc_filename).exists() {
                return Ok((code_filename, desc_filename));
            }
        }

        let question = match identifier {
            Identifier::Number(num) => {
                println!("🔍 Fetching problem ID: {}...", num);
                self.client.get_question_by_id(*num).await?
            }
            Identifier::String(identifier) => {
                println!("🔍 Fetching problem: {}...", identifier);
                self.client.get_question_by_slug(identifier).await?
            }
        };

        // Convert LeetCode's raw HTML into wrapped terminal text (80 columns wide)
        let formatted_content = html2md::parse_html(&question.content);
        let md_content = format!("# {}\n\n{}", question.title, formatted_content);

        let snippet = question
            .code_snippets
            .iter()
            .find(|s| s.lang_slug == language.to_lang_slug());

        let snippet = match snippet {
            Some(s) => s,
            None => {
                let snippet = question
                    .code_snippets
                    .first()
                    .ok_or_else(|| EngineError::Other("LeetCode problem has no code snippets".to_string()))?;
                language = Language::from(snippet.lang_slug.clone());
                snippet
            }
        };

        //  determine filenames (converting kebab-case to snake_case)
        let snake_slug = question.title_slug.replace("-", "_");
        let code_filename = format!("{}.{}", snake_slug, language.code_extension());
        let desc_filename = format!("{}.md", snake_slug);

        let meta = format!(
            "{} id={} slug={} lang={}",
            language.meta_comment_prefix(),
            question.question_id,
            question.title_slug,
            language.to_lang_slug()
        );

        if let Err(e) = fs::write(&code_filename, format!("{}\n\n{}", meta, snippet.code)) {
            eprintln!("❌ failed to write code file: {}", e);
            return Err(EngineError::System);
        }
        if let Err(e) = fs::write(&desc_filename, md_content) {
            eprintln!("❌ failed to write description file: {}", e);
            return Err(EngineError::System);
        }
        println!("✅ files generated successfully.");

        Ok((code_filename, desc_filename))
    }

    /// Runs the solution file against the problem's built-in example test cases
    /// and prints the result, but **does not** record it as an official submission.
    pub async fn test_submit(&self, file: &String) {
        let service = SubmissionService::new(self.client.clone());
        match service.submit_or_test(file, true).await {
            Ok(result) => format_result(&result),
            Err(e) => eprintln!("❌ {}", e),
        }
    }

    /// Submits the solution file to LeetCode for full judging and prints the
    /// verdict, test-case counts, and performance percentiles.
    pub async fn submit(&self, file: &String) {
        let service = SubmissionService::new(self.client.clone());
        match service.submit_or_test(file, false).await {
            Ok(result) => format_result(&result),
            Err(e) => eprintln!("❌ {}", e),
        }
    }

    /// Returns the cached user profile, refreshing it in the background.
    ///
    /// **Cache-aside strategy:**
    /// 1. Read `user.json` from disk.
    /// 2. If found, return it immediately and spawn a background task that
    ///    fetches the latest data from the API and overwrites the file.
    /// 3. If not found, block on the API fetch, write the file, then return.
    pub async fn get_user_data(&self) -> crate::error::Result<UserDetail> {
        let cache = CacheService::new();
        let user_path = cache.user_path();
        let data = match fs::read_to_string(&user_path) {
            Ok(v) => {
                let client = self.client.clone();
                let user_path_bg = user_path.clone();
                tokio::spawn(async move {
                    let result: Result<(), Box<dyn std::error::Error>> = async {
                        let user_detail = client.get_user_detail().await?;
                        let data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(&user_path_bg, &data);
                        Ok(())
                    }
                    .await;

                    let _ = result;
                });
                v
            }
            Err(_) => {
                let user_detail = self.client.get_user_detail().await?;
                let data = serde_json::to_string(&user_detail)?;
                let _ = fs::write(&user_path, &data);
                data
            }
        };
        let user_detail: UserDetail = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Failed to parse user details: {}", e);
            eprintln!("Try running `leetrs tui` again to refresh the cache.");
            if let Err(err) = fs::remove_file(&user_path) {
                eprintln!("Failed to remove corrupted cache file: {}", err);
            }
            e
        })?;
        Ok(user_detail)
    }

    /// Returns the full problem list, enriched with topic tags.
    ///
    /// **Cache-aside strategy:**
    /// 1. Read `data.json` from disk.
    /// 2. If found, return it immediately and spawn a background task that
    ///    fetches a fresh list (problems + tags) and overwrites the cache.
    /// 3. If not found, block on both API calls, write the cache, then return.
    pub async fn list_problems(&self) -> crate::error::Result<Vec<ProblemSummary>> {
        let cache = CacheService::new();
        let data_path = cache.problems_path();
        let user_path = cache.user_path();
        let data = match fs::read_to_string(&data_path) {
            Ok(v) => {
                // Fetch data in the background and update data.json for next time
                let client_clone = self.client.clone();
                let data_path_bg = data_path.clone();
                let user_path_bg = user_path.clone();
                tokio::spawn(async move {
                    let res: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
                        let user_detail = client_clone.get_user_detail().await?;
                        let data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(&user_path_bg, data);
                        let mut problems = client_clone.get_problem_list().await?;
                        let question_tags = client_clone.get_topics_question_list().await?;
                        for question_tag in question_tags {
                            question_tag.question_ids.iter().for_each(|question_id| {
                                if let Some(problem) =
                                    problems.iter_mut().find(|p| p.id == *question_id)
                                {
                                    problem.topics.push(question_tag.name.clone());
                                }
                            });
                        }
                        let data = serde_json::to_string(&problems)?;
                        let _ = fs::write(&data_path_bg, data);
                        Ok(())
                    }
                    .await;

                    let _ = res;
                });
                v
            }
            Err(_) => {
                let mut problems = self.client.get_problem_list().await?;
                let question_tags = self.client.get_topics_question_list().await?;
                for question_tag in question_tags {
                    question_tag.question_ids.iter().for_each(|question_id| {
                        if let Some(problem) = problems.iter_mut().find(|p| p.id == *question_id) {
                            problem.topics.push(question_tag.name.clone());
                        }
                    });
                }
                let data = serde_json::to_string(&problems)?;
                let _ = fs::write(&data_path, &data);
                data
            }
        };
        let problems: Vec<ProblemSummary> = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Failed to parse problem list: {}", e);
            eprintln!("Try running `leetrs tui` again to refresh the cache.");
            if let Err(err) = fs::remove_file(&data_path) {
                eprintln!("Failed to remove corrupted cache file: {}", err);
            }
            e
        })?;
        Ok(problems)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Language;

    // -----------------------------------------------------------------------
    // Slug / filename derivation (mirrors logic in Picker::pick)
    // -----------------------------------------------------------------------

    /// Converts a kebab-case LeetCode slug to the snake_case form used for filenames.
    fn slug_to_snake(slug: &str) -> String {
        slug.replace('-', "_")
    }

    /// Converts a snake_case filename stem back to the kebab-case API slug.
    fn stem_to_slug(stem: &str) -> String {
        stem.replace('_', "-")
    }

    #[test]
    fn slug_to_snake_single_word() {
        assert_eq!(slug_to_snake("fizzbuzz"), "fizzbuzz");
    }

    #[test]
    fn slug_to_snake_two_words() {
        assert_eq!(slug_to_snake("two-sum"), "two_sum");
    }

    #[test]
    fn slug_to_snake_multiple_hyphens() {
        assert_eq!(
            slug_to_snake("longest-palindromic-substring"),
            "longest_palindromic_substring"
        );
    }

    #[test]
    fn slug_to_snake_empty_string() {
        assert_eq!(slug_to_snake(""), "");
    }

    #[test]
    fn stem_to_slug_roundtrip() {
        let slug = "trapping-rain-water";
        assert_eq!(stem_to_slug(&slug_to_snake(slug)), slug);
    }

    #[test]
    fn stem_to_slug_roundtrip_single_word() {
        let slug = "median";
        assert_eq!(stem_to_slug(&slug_to_snake(slug)), slug);
    }

    // -----------------------------------------------------------------------
    // Code filename derivation per language
    // -----------------------------------------------------------------------

    fn code_filename(snake_slug: &str, lang: &Language) -> String {
        match lang {
            Language::Rust => format!("{}.rs", snake_slug),
            Language::Python | Language::Pandas => format!("{}.py", snake_slug),
            Language::Mysql | Language::Postgres => format!("{}.sql", snake_slug),
        }
    }

    #[test]
    fn code_filename_rust() {
        assert_eq!(code_filename("two_sum", &Language::Rust), "two_sum.rs");
    }

    #[test]
    fn code_filename_python() {
        assert_eq!(code_filename("two_sum", &Language::Python), "two_sum.py");
    }

    #[test]
    fn code_filename_pandas_uses_py_extension() {
        // Pandas is a Python library — the file extension must also be .py.
        assert_eq!(
            code_filename("group_sold_products", &Language::Pandas),
            "group_sold_products.py"
        );
    }

    #[test]
    fn code_filename_mysql() {
        assert_eq!(
            code_filename("combine_two_tables", &Language::Mysql),
            "combine_two_tables.sql"
        );
    }

    #[test]
    fn code_filename_postgres_uses_sql_extension() {
        assert_eq!(
            code_filename("combine_two_tables", &Language::Postgres),
            "combine_two_tables.sql"
        );
    }

    #[test]
    fn desc_filename_always_md() {
        let desc = format!("{}.md", slug_to_snake("longest-palindromic-substring"));
        assert_eq!(desc, "longest_palindromic_substring.md");
    }

    // -----------------------------------------------------------------------
    // Metadata comment prefix per language
    // -----------------------------------------------------------------------

    fn meta_prefix(lang: &Language) -> &'static str {
        match lang {
            Language::Python | Language::Pandas | Language::Mysql => "#",
            Language::Rust => "//",
            Language::Postgres => "--",
        }
    }

    #[test]
    fn meta_prefix_python_is_hash() {
        assert_eq!(meta_prefix(&Language::Python), "#");
    }

    #[test]
    fn meta_prefix_pandas_is_hash() {
        assert_eq!(meta_prefix(&Language::Pandas), "#");
    }

    #[test]
    fn meta_prefix_rust_is_double_slash() {
        assert_eq!(meta_prefix(&Language::Rust), "//");
    }

    #[test]
    fn meta_prefix_mysql_is_hash() {
        assert_eq!(meta_prefix(&Language::Mysql), "#");
    }

    #[test]
    fn meta_prefix_postgres_is_double_dash() {
        assert_eq!(meta_prefix(&Language::Postgres), "--");
    }

    #[test]
    fn metadata_line_format_rust() {
        let lang = Language::Rust;
        let line = format!(
            "{} id={} slug={} lang={}",
            meta_prefix(&lang),
            1,
            "two-sum",
            lang.to_lang_slug()
        );
        assert_eq!(line, "// id=1 slug=two-sum lang=rust");
    }

    #[test]
    fn metadata_line_format_python() {
        let lang = Language::Python;
        let line = format!(
            "{} id={} slug={} lang={}",
            meta_prefix(&lang),
            42,
            "climbing-stairs",
            lang.to_lang_slug()
        );
        assert_eq!(line, "# id=42 slug=climbing-stairs lang=python3");
    }

    #[test]
    fn metadata_line_format_postgres() {
        let lang = Language::Postgres;
        let line = format!(
            "{} id={} slug={} lang={}",
            meta_prefix(&lang),
            175,
            "combine-two-tables",
            lang.to_lang_slug()
        );
        assert_eq!(line, "-- id=175 slug=combine-two-tables lang=postgresql");
    }

    // -----------------------------------------------------------------------
    // Acceptance ratio edge cases (mirrors get_problem_list arithmetic)
    // -----------------------------------------------------------------------

    fn acceptance_ratio(accepted: u64, submitted: u64) -> f64 {
        accepted as f64 / submitted as f64
    }

    #[test]
    fn acceptance_ratio_normal() {
        let r = acceptance_ratio(500, 1000);
        assert!((r - 0.5).abs() < 1e-9);
    }

    #[test]
    fn acceptance_ratio_perfect() {
        let r = acceptance_ratio(1000, 1000);
        assert!((r - 1.0).abs() < 1e-9);
    }

    #[test]
    fn acceptance_ratio_zero_submissions_is_nan() {
        // 0/0 in floating point is NaN. This is what the current production code
        // produces; asserting it documents the known behavior explicitly.
        let r = acceptance_ratio(0, 0);
        assert!(r.is_nan(), "expected NaN for 0/0, got {r}");
    }

    #[test]
    fn acceptance_ratio_zero_accepted() {
        let r = acceptance_ratio(0, 100);
        assert!((r - 0.0).abs() < 1e-9);
    }
}
