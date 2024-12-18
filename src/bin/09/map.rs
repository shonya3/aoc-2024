#[derive(Debug)]
pub struct ParseDigitError(pub char);

#[derive(Debug, Clone)]
pub struct Map(pub Vec<Element>);

impl Map {
    pub fn compress(&mut self) -> &mut Self {
        while self.compress_step().is_ok() {}

        self
    }

    pub fn from_input(s: &str) -> Result<Map, ParseDigitError> {
        let mut current_file_id: u128 = 0;
        let mut map = Map(Vec::new());

        for (i, ch) in s.char_indices() {
            let is_file = i % 2 == 0;
            let n = ch.to_digit(10).ok_or(ParseDigitError(ch))?;

            for _ in 0..n {
                let el = match is_file {
                    true => Element::File(FileId(current_file_id)),
                    false => Element::Empty,
                };

                map.0.push(el);
            }

            if is_file {
                current_file_id += 1;
            }
        }

        Ok(map)
    }

    fn first_empty(&self) -> Option<usize> {
        self.0.iter().position(|el| *el == Element::Empty)
    }

    fn compress_step(&mut self) -> Result<(), CompressionError> {
        let Some(leftmost_empty_index) = self.first_empty() else {
            return Err(CompressionError::AlreadyCompressed);
        };

        let Some(rightmost_unsorted_file_index) = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, el)| -> Option<usize> {
                let Element::File(_) = el else {
                    return None;
                };
                Some(i)
            })
            .last()
        else {
            return Err(CompressionError::NoFiles);
        };

        if leftmost_empty_index > rightmost_unsorted_file_index {
            return Err(CompressionError::AlreadyCompressed);
        }

        self.0
            .swap(leftmost_empty_index, rightmost_unsorted_file_index);
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompressionError {
    AlreadyCompressed,
    NoFiles,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for el in self.0.iter() {
            write!(f, "{el}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    File(FileId),
    Empty,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::File(file_id) => f.write_str(&file_id.0.to_string()),
            Element::Empty => f.write_str("."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FileId(pub u128);

#[cfg(test)]
mod tests {
    use crate::map::{Element, FileId};

    use super::Map;

    const SIMPLE_EXAMPLE: &str = "12345";
    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn compress() {
        assert_eq!(
            "022111222......",
            Map::from_input(SIMPLE_EXAMPLE)
                .unwrap()
                .compress()
                .to_string()
        );

        assert_eq!(
            "0099811188827773336446555566..............",
            Map::from_input(EXAMPLE).unwrap().compress().to_string()
        )
    }

    #[test]
    fn from_input() {
        let f = |n| Element::File(FileId(n));
        let e = || Element::Empty;

        assert_eq!(
            vec![
                f(0),
                e(),
                e(),
                f(1),
                f(1),
                f(1),
                e(),
                e(),
                e(),
                e(),
                f(2),
                f(2),
                f(2),
                f(2),
                f(2)
            ],
            Map::from_input(SIMPLE_EXAMPLE).unwrap().0,
        );
    }

    #[test]
    fn print_map() {
        assert_eq!(
            &Map::from_input(SIMPLE_EXAMPLE).unwrap().to_string(),
            "0..111....22222"
        );

        assert_eq!(
            &Map::from_input(EXAMPLE).unwrap().to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        )
    }
}
