use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Map(pub Vec<Vec<u8>>);

#[allow(unused)]
#[derive(Debug)]
pub struct ParseDigitError(char);

impl FromStr for Map {
    type Err = ParseDigitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                let result = line
                    .chars()
                    .map(|c| -> Result<u8, ParseDigitError> {
                        c.to_digit(10).map(|d| d as u8).ok_or(ParseDigitError(c))
                    })
                    .collect::<Result<Vec<u8>, ParseDigitError>>();
                result
            })
            .collect::<Result<Vec<Vec<u8>>, ParseDigitError>>()?;

        Ok(Map(grid))
    }
}

impl Map {
    pub fn get(&self, i: usize, j: usize) -> Option<u8> {
        self.0.get(i).and_then(|row| row.get(j)).copied()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let latest_row = self.0.len() - 1;
        for (i, row) in self.0.iter().enumerate() {
            for digit in row.iter() {
                write!(f, "{digit}")?;
            }
            if i != latest_row {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
