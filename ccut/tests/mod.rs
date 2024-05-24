#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    fn run_test(command: &str, expected_output_file: &str) {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");

        let expected_output = fs::read_to_string(expected_output_file)
            .expect("failed to read expected output file")
            .replace("\r\n", "\n"); // Normalize line endings

        let actual_output = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
        // Remove the stupid `\u{feff}` from the given file
        let actual_output = actual_output.trim_start_matches('\u{FEFF}');
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_step1() {
        run_test(
            "cargo run -- ccut -f2 tests/data/sample.tsv",
            "tests/expected_outputs/step1.txt",
        );
    }

    #[test]
    fn test_step2_comma() {
        run_test(
            "cargo run -- ccut -f1 -d, tests/data/fourchords.csv | head -n5",
            "tests/expected_outputs/step2_comma.txt",
        );
    }

    #[test]
    fn test_step2_tab() {
        run_test(
            "cargo run -- ccut -f1 tests/data/sample.tsv",
            "tests/expected_outputs/step2_tab.txt",
        );
    }

    #[test]
    fn test_step3() {
        run_test(
            "cargo run -- ccut -f1,2 tests/data/sample.tsv",
            "tests/expected_outputs/step3.txt",
        );
    }

    #[test]
    fn test_step4() {
        run_test(
            "tail -n5 tests/data/fourchords.csv | cargo run -- ccut -d, -f\"1 2\" -",
            "tests/expected_outputs/step4.txt",
        );
    }

    #[test]
    fn test_step5() {
        run_test(
            "cargo run -- ccut -f2 -d, tests/data/fourchords.csv | uniq | wc -l",
            "tests/expected_outputs/step5.txt",
        );
    }
}
