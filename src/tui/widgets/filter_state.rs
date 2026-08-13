//! Filter state — difficulty, topic, and fuzzy-search filter logic.
use std::collections::HashSet;

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::widgets::ListState;

use crate::models::ProblemSummary;

/// Topic filter overlay state — a sorted list of all known topics and the
/// currently selected subset.
pub struct TopicFilterState {
    pub all_topics: Vec<String>,
    pub selected_topics: HashSet<String>,
    pub list_state: ListState,
}

impl TopicFilterState {
    pub fn new(problems: &[ProblemSummary]) -> Self {
        let mut set = HashSet::new();
        for p in problems.iter() {
            for t in &p.topics {
                set.insert(t.clone());
            }
        }
        let mut all_topics: Vec<String> = set.into_iter().collect();
        all_topics.sort();

        let mut list_state = ListState::default();
        if !all_topics.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            all_topics,
            selected_topics: HashSet::new(),
            list_state,
        }
    }

    pub fn cursor(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn next(&mut self) {
        if self.all_topics.is_empty() {
            return;
        }
        let i = self.cursor();
        let next = if i >= self.all_topics.len() - 1 {
            0
        } else {
            i + 1
        };
        self.list_state.select(Some(next));
    }

    pub fn previous(&mut self) {
        if self.all_topics.is_empty() {
            return;
        }
        let i = self.cursor();
        let prev = if i == 0 {
            self.all_topics.len() - 1
        } else {
            i - 1
        };
        self.list_state.select(Some(prev));
    }

    pub fn toggle_current(&mut self) {
        let cursor = self.cursor();
        if let Some(topic) = self.all_topics.get(cursor).cloned() {
            if self.selected_topics.contains(&topic) {
                self.selected_topics.remove(&topic);
            } else {
                self.selected_topics.insert(topic);
            }
        }
    }

    pub fn clear(&mut self) {
        self.selected_topics.clear();
    }
}

/// Encapsulates difficulty, topic, and search filters.
///
/// Call [`FilterState::apply`] to get the set of problem indices that pass all
/// active filters in one efficient pass.
pub struct FilterState {
    pub difficulty: Option<u8>,
    pub topics: TopicFilterState,
}

impl FilterState {
    pub fn new(problems: &[ProblemSummary]) -> Self {
        Self {
            difficulty: None,
            topics: TopicFilterState::new(problems),
        }
    }

    /// Returns the sorted list of indices into `problems` that survive all
    /// active filters and the given search query.
    pub fn apply(&self, problems: &[ProblemSummary], query: &str) -> Vec<usize> {
        let has_topics = !self.topics.selected_topics.is_empty();

        let candidates = problems.iter().enumerate().filter(|(_, p)| {
            if let Some(diff) = self.difficulty
                && p.difficulty != diff
            {
                return false;
            }
            if has_topics {
                let matches_topic = p
                    .topics
                    .iter()
                    .any(|t| self.topics.selected_topics.contains(t));
                if !matches_topic {
                    return false;
                }
            }
            true
        });

        if query.is_empty() {
            candidates.map(|(idx, _)| idx).collect()
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored: Vec<(i64, usize)> = Vec::with_capacity(problems.len());
            for (idx, p) in candidates {
                if let Some(score) = matcher
                    .fuzzy_match(&p.title, query)
                    .or_else(|| matcher.fuzzy_match(&p.id.to_string(), query))
                {
                    scored.push((score, idx));
                }
            }
            scored.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));
            scored.into_iter().map(|(_, idx)| idx).collect()
        }
    }

    pub fn set_difficulty(&mut self, difficulty: u8) {
        if difficulty > 0 && difficulty < 4 {
            self.difficulty = Some(difficulty);
        } else {
            self.difficulty = None;
        }
    }
}
