use semver::Version;

pub const SOLC_VERSION: Version = Version::new(0, 8, 20);

pub const SOURCES: [&str; 3] = [
    include_str!("./ERC20.sol"),
    include_str!("./ERC721.sol"),
    include_str!("./Governor.sol"),
];
