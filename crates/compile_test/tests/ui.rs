use ui_test::{
    default_file_filter, default_per_file_config, run_tests_generic, status_emitter::Text, Args,
    Config, OutputConflictHandling,
};

fn main() -> ui_test::Result<()> {
    let mut config = Config {
        dependencies_crate_manifest_path: Some("Cargo.toml".into()),
        output_conflict_handling: OutputConflictHandling::Ignore,
        ..Config::rustc("tests/ui")
    };

    let args = Args::test()?;

    config.with_args(&args);

    run_tests_generic(
        [config].into(),
        default_file_filter,
        default_per_file_config,
        Text::verbose(),
    )
}
