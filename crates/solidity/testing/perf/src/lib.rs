use std::path::PathBuf;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::cst::Node;
use slang_solidity::language::Language;
use strum_macros::Display;
use url::Url;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Display)]
pub enum TestContractKind {
    ERC721_v5_0_0,
}

impl TestContractKind {
    fn url(self) -> Result<Url> {
        Ok(Url::parse( match self {
            Self::ERC721_v5_0_0 => "https://raw.githubusercontent.com/OpenZeppelin/openzeppelin-contracts/v5.0.0/contracts/token/ERC721/ERC721.sol",
        })?)
    }

    fn solc_version(self) -> Result<Version> {
        Ok(Version::parse(match self {
            Self::ERC721_v5_0_0 => "0.8.20",
        })?)
    }

    pub fn load(&self) -> Result<TestContract> {
        let cache_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/datasets");

        std::fs::create_dir_all(&cache_dir)?;

        let cache_file = cache_dir.join(format!("{self}.sol"));

        let source = if cache_file.exists() {
            cache_file.read_to_string()?
        } else {
            let source = reqwest::blocking::get(self.url()?)?.text()?;
            cache_file.write_string(&source)?;
            source
        };

        Ok(TestContract {
            kind: *self,
            source,
        })
    }
}

pub struct TestContract {
    kind: TestContractKind,
    source: String,
}

impl TestContract {
    pub fn parse(&self) -> Result<Node> {
        let language = Language::new(self.kind.solc_version()?)?;

        let parse_output = language.parse(Language::ROOT_KIND, &self.source);

        assert!(parse_output.is_valid(), "{0:?}", parse_output.errors());

        Ok(parse_output.tree())
    }
}
