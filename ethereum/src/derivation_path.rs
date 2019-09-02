use wagyu_model::derivation_path::{ChildIndex, DerivationPath, DerivationPathError};

use std::convert::TryFrom;
use std::{fmt, str::FromStr};

/// Represents a Ethereum derivation path
#[derive(Clone, PartialEq, Eq)]
pub struct EthereumDerivationPath(Vec<ChildIndex>);

impl DerivationPath for EthereumDerivationPath {
    /// Returns a child index vector given the derivation path.
    fn to_vec(&self) -> Result<Vec<ChildIndex>, DerivationPathError> {
        Ok(self.0.clone())
    }

    /// Returns a derivation path given the child index vector.
    fn from_vec(path: &Vec<ChildIndex>) -> Result<Self, DerivationPathError> {
        Ok(Self(path.clone()))
    }
}

impl FromStr for EthereumDerivationPath {
    type Err = DerivationPathError;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let mut parts = path.split("/");

        if parts.next().unwrap() != "m" {
            return Err(DerivationPathError::InvalidDerivationPath(path.to_string()));
        }

        let path: Result<Vec<ChildIndex>, Self::Err> = parts.map(str::parse).collect();
        Ok(Self(path?))
    }
}

impl TryFrom<Vec<ChildIndex>> for EthereumDerivationPath {
    type Error = DerivationPathError;

    fn try_from(path: Vec<ChildIndex>) -> Result<Self, Self::Error> {
        Self::from_vec(&path)
    }
}

impl<'a> TryFrom<&'a [ChildIndex]> for EthereumDerivationPath {
    type Error = DerivationPathError;

    fn try_from(path: &'a [ChildIndex]) -> Result<Self, Self::Error> {
        Self::try_from(path.to_vec())
    }
}

impl fmt::Debug for EthereumDerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for EthereumDerivationPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.to_vec() {
            Ok(path) => {
                f.write_str("m")?;
                for index in path.iter() {
                    f.write_str("/")?;
                    fmt::Display::fmt(index, f)?;
                }
                Ok(())
            }
            Err(_) => Err(fmt::Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wagyu_model::derivation_path::{ChildIndex, DerivationPathError};

    use std::convert::TryInto;
    use std::str::FromStr;

    #[test]
    fn valid_path() {
        assert_eq!(EthereumDerivationPath::from_str("m"), Ok(vec![].try_into().unwrap()));
        assert_eq!(
            EthereumDerivationPath::from_str("m/0"),
            Ok(vec![ChildIndex::from_normal(0).unwrap()].try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0/1"),
            Ok(vec![ChildIndex::from_normal(0).unwrap(), ChildIndex::from_normal(1).unwrap()].try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0/1/2"),
            Ok(vec![
                ChildIndex::from_normal(0).unwrap(),
                ChildIndex::from_normal(1).unwrap(),
                ChildIndex::from_normal(2).unwrap()
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0/1/2/3"),
            Ok(vec![
                ChildIndex::from_normal(0).unwrap(),
                ChildIndex::from_normal(1).unwrap(),
                ChildIndex::from_normal(2).unwrap(),
                ChildIndex::from_normal(3).unwrap()
            ]
                .try_into().unwrap())
        );

        assert_eq!(EthereumDerivationPath::from_str("m"), Ok(vec![].try_into().unwrap()));
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'"),
            Ok(vec![ChildIndex::from_hardened(0).unwrap()].try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'/1"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_normal(1).unwrap()
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'/1/2'"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_normal(1).unwrap(),
                ChildIndex::from_hardened(2).unwrap(),
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'/1/2'/3"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_normal(1).unwrap(),
                ChildIndex::from_hardened(2).unwrap(),
                ChildIndex::from_normal(3).unwrap(),
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'/1/2'/3/4'"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_normal(1).unwrap(),
                ChildIndex::from_hardened(2).unwrap(),
                ChildIndex::from_normal(3).unwrap(),
                ChildIndex::from_hardened(4).unwrap(),
            ]
                .try_into().unwrap())
        );

        assert_eq!(EthereumDerivationPath::from_str("m"), Ok(vec![].try_into().unwrap()));
        assert_eq!(
            EthereumDerivationPath::from_str("m/0h"),
            Ok(vec![ChildIndex::from_hardened(0).unwrap()].try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0h/1'"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_hardened(1).unwrap()
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'/1h/2'"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_hardened(1).unwrap(),
                ChildIndex::from_hardened(2).unwrap(),
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0h/1'/2h/3'"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_hardened(1).unwrap(),
                ChildIndex::from_hardened(2).unwrap(),
                ChildIndex::from_hardened(3).unwrap(),
            ]
                .try_into().unwrap())
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0'/1h/2'/3h/4'"),
            Ok(vec![
                ChildIndex::from_hardened(0).unwrap(),
                ChildIndex::from_hardened(1).unwrap(),
                ChildIndex::from_hardened(2).unwrap(),
                ChildIndex::from_hardened(3).unwrap(),
                ChildIndex::from_hardened(4).unwrap(),
            ]
                .try_into().unwrap())
        );
    }

    #[test]
    fn invalid_path() {
        assert_eq!(
            EthereumDerivationPath::from_str("n"),
            Err(DerivationPathError::InvalidDerivationPath("n".try_into().unwrap()))
        );
        assert_eq!(
            EthereumDerivationPath::from_str("n/0"),
            Err(DerivationPathError::InvalidDerivationPath("n/0".try_into().unwrap()))
        );
        assert_eq!(
            EthereumDerivationPath::from_str("n/0/0"),
            Err(DerivationPathError::InvalidDerivationPath("n/0/0".try_into().unwrap()))
        );

        assert_eq!(
            EthereumDerivationPath::from_str("1"),
            Err(DerivationPathError::InvalidDerivationPath("1".try_into().unwrap()))
        );
        assert_eq!(
            EthereumDerivationPath::from_str("1/0"),
            Err(DerivationPathError::InvalidDerivationPath("1/0".try_into().unwrap()))
        );
        assert_eq!(
            EthereumDerivationPath::from_str("1/0/0"),
            Err(DerivationPathError::InvalidDerivationPath("1/0/0".try_into().unwrap()))
        );

        assert_eq!(
            EthereumDerivationPath::from_str("m/0x"),
            Err(DerivationPathError::InvalidChildNumberFormat)
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0x0"),
            Err(DerivationPathError::InvalidChildNumberFormat)
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/0x00"),
            Err(DerivationPathError::InvalidChildNumberFormat)
        );

        assert_eq!(
            EthereumDerivationPath::from_str("0/m"),
            Err(DerivationPathError::InvalidDerivationPath("0/m".try_into().unwrap()))
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m//0"),
            Err(DerivationPathError::InvalidChildNumberFormat)
        );
        assert_eq!(
            EthereumDerivationPath::from_str("m/2147483648"),
            Err(DerivationPathError::InvalidChildNumber(2147483648))
        );
    }
}
