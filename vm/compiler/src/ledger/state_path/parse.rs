// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

static STATE_PATH_PREFIX: &str = "path";

impl<N: Network> Parser for StatePath<N> {
    /// Parses a string into the state path.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Prepare a parser for the Aleo state path.
        let parse_state_path = recognize(pair(
            pair(tag(STATE_PATH_PREFIX), tag("1")),
            many1(terminated(one_of("qpzry9x8gf2tvdw0s3jn54khce6mua7l"), many0(char('_')))),
        ));

        // Parse the state path from the string.
        map_res(parse_state_path, |state_path: &str| -> Result<_, Error> {
            Self::from_str(&state_path.replace('_', ""))
        })(string)
    }
}

impl<N: Network> FromStr for StatePath<N> {
    type Err = Error;

    /// Reads in the state path string.
    fn from_str(state_path: &str) -> Result<Self, Self::Err> {
        // Decode the state path string from bech32m.
        let (hrp, data, variant) = bech32::decode(state_path)?;
        if hrp != STATE_PATH_PREFIX {
            bail!("Failed to decode state path: '{hrp}' is an invalid prefix")
        } else if data.is_empty() {
            bail!("Failed to decode state path: data field is empty")
        } else if variant != bech32::Variant::Bech32m {
            bail!("Found an state path that is not bech32m encoded: {state_path}");
        }
        // Decode the state path data from u5 to u8, and into the state path.
        Ok(Self::read_le(&Vec::from_base32(&data)?[..])?)
    }
}

impl<N: Network> Debug for StatePath<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network> Display for StatePath<N> {
    /// Writes the state path as a bech32m string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Convert the state path to bytes.
        let bytes = self.to_bytes_le().map_err(|_| fmt::Error)?;
        // Encode the bytes into bech32m.
        let string =
            bech32::encode(STATE_PATH_PREFIX, bytes.to_base32(), bech32::Variant::Bech32m).map_err(|_| fmt::Error)?;
        // Output the string.
        Display::fmt(&string, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_parse() -> Result<()> {
        let mut rng = TestRng::default();

        // Ensure type and empty value fails.
        assert!(StatePath::<CurrentNetwork>::parse(&format!("{STATE_PATH_PREFIX}1")).is_err());
        assert!(StatePath::<CurrentNetwork>::parse("").is_err());

        // Sample a ledger.
        let ledger = crate::ledger::test_helpers::sample_genesis_ledger(&mut rng);

        // Retrieve the genesis block.
        let genesis = ledger.get_block(0).unwrap();
        // Ensure there is at least 1 commitment.
        assert!(genesis.transactions().commitments().count() > 0);

        // Check each commitment.
        for commitment in genesis.transactions().commitments() {
            // Compute the state path.
            let expected = ledger.to_state_path(commitment).unwrap();

            let expected = format!("{expected}");
            let (remainder, candidate) = StatePath::<CurrentNetwork>::parse(&expected).unwrap();
            assert_eq!(format!("{expected}"), candidate.to_string());
            assert_eq!(STATE_PATH_PREFIX, candidate.to_string().split('1').next().unwrap());
            assert_eq!("", remainder);
        }
        Ok(())
    }

    #[test]
    fn test_string() -> Result<()> {
        let mut rng = TestRng::default();

        // Sample a ledger.
        let ledger = crate::ledger::test_helpers::sample_genesis_ledger(&mut rng);

        // Retrieve the genesis block.
        let genesis = ledger.get_block(0).unwrap();
        // Ensure there is at least 1 commitment.
        assert!(genesis.transactions().commitments().count() > 0);

        // Check each commitment.
        for commitment in genesis.transactions().commitments() {
            // Compute the state path.
            let expected = ledger.to_state_path(commitment).unwrap();

            // Check the string representation.
            let candidate = format!("{expected}");
            assert_eq!(expected, StatePath::from_str(&candidate)?);
            assert_eq!(STATE_PATH_PREFIX, candidate.to_string().split('1').next().unwrap());
        }
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let mut rng = TestRng::default();

        // Sample a ledger.
        let ledger = crate::ledger::test_helpers::sample_genesis_ledger(&mut rng);

        // Retrieve the genesis block.
        let genesis = ledger.get_block(0).unwrap();
        // Ensure there is at least 1 commitment.
        assert!(genesis.transactions().commitments().count() > 0);

        // Check each commitment.
        for commitment in genesis.transactions().commitments() {
            // Compute the state path.
            let expected = ledger.to_state_path(commitment).unwrap();

            let candidate = expected.to_string();
            assert_eq!(format!("{expected}"), candidate);
            assert_eq!(STATE_PATH_PREFIX, candidate.split('1').next().unwrap());

            let candidate_recovered = StatePath::<CurrentNetwork>::from_str(&candidate.to_string())?;
            assert_eq!(expected, candidate_recovered);
        }
        Ok(())
    }
}
