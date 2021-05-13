use inflector::string::pluralize::to_plural;

#[derive(Default, Debug)]
/// Statistics about a run of DirectoryPatcher
pub struct Stats {
    matching_files: usize,
    matching_lines: usize,
    total_replacements: usize,
}

impl Stats {
    pub(crate) fn update(&mut self, lines: usize, replacements: usize) {
        self.matching_files += 1;
        self.matching_lines += lines;
        self.total_replacements += replacements;
    }

    /// Number of matching files
    pub fn matching_files(&self) -> usize {
        self.matching_files
    }

    /// Total number of lines that were replaced
    pub fn matching_lines(&self) -> usize {
        self.matching_lines
    }

    /// Total number of lines that were replaced
    pub fn total_replacements(&self) -> usize {
        self.total_replacements
    }
}

fn pluralize(input: &str, num: usize) -> String {
    if num > 1 {
        to_plural(input)
    } else {
        input.to_string()
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let file_string = pluralize("file", self.matching_files);
        let replacements_string = pluralize("replacement", self.total_replacements);
        write!(
            f,
            "{} {} on {} matching {}",
            self.total_replacements, replacements_string, self.matching_files, file_string
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_to_string() {
        let stats = Stats {
            matching_files: 2,
            total_replacements: 4,
            matching_lines: 1,
        };
        let actual = stats.to_string();
        assert_eq!(actual, "4 replacements on 2 matching files");

        let stats = Stats {
            matching_files: 1,
            total_replacements: 2,
            matching_lines: 1,
        };
        let actual = stats.to_string();
        assert_eq!(actual, "2 replacements on 1 matching file");
    }
}
