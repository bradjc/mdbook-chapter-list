use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use semver::{Version, VersionReq};
use std::io;

mod preprocessor;

fn main() {
    let chapter_lister = preprocessor::ChapterList::new();

    let _ = handle_preprocessing(&chapter_lister);
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let version = Version::parse(&ctx.mdbook_version).unwrap();
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION).unwrap();
    if !version_req.matches(&version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
