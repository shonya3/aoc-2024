#[allow(unused)]
#[derive(Debug)]
pub struct ParseDigitError(pub char);

#[derive(Debug, Clone)]
pub struct Map(pub Vec<Element>);

impl Map {
    pub fn compress(&mut self) -> &mut Self {
        let mut cursor = None;
        while self.compress_step(&mut cursor).is_ok() {}

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

    pub fn find_empty_block(&self, size: u32) -> Option<(usize, usize)> {
        let mut iter = self.0.iter().enumerate();

        loop {
            let (i, el) = iter.next()?;

            if *el == Element::Empty {
                let mut size_remaining = size - 1;
                let start = i;
                let mut end = i;

                loop {
                    if size_remaining == 0 {
                        return Some((start, end));
                    }

                    let (i, el) = iter.next()?;
                    if *el != Element::Empty {
                        break;
                    }

                    size_remaining -= 1;
                    end = i;
                }

                continue;
            }
        }
    }

    pub fn compress_step(
        &mut self,
        file_cursor: &mut Option<usize>,
    ) -> Result<(), CompressionError> {
        let find_next_file =
            |file_cursor: &mut Option<usize>| -> Result<(usize, usize), CompressionError> {
                let mut iter = self
                    .0
                    .iter()
                    .enumerate()
                    .take(file_cursor.unwrap_or(usize::MAX - 1) + 1)
                    .rev();

                loop {
                    let Some((i, el)) = iter.next() else {
                        return Err(CompressionError::NoFiles);
                    };
                    *file_cursor = Some(i);

                    if let Element::File(file_id) = el {
                        let end = i;
                        let mut start = i;

                        loop {
                            match iter.next() {
                                Some((i, el)) => {
                                    *file_cursor = Some(i);
                                    match el {
                                        Element::File(id) => match file_id == id {
                                            true => start = i,
                                            false => return Ok((start, end)),
                                        },
                                        Element::Empty => return Ok((start, end)),
                                    }
                                }
                                None => return Err(CompressionError::AlreadyCompressed),
                            }
                        }
                    }
                }
            };

        let (file_start, file_end) = find_next_file(file_cursor)?;
        let size = file_end as u32 - file_start as u32 + 1;
        let Some((empty_start, _)) = self.find_empty_block(size) else {
            return Ok(());
        };

        if empty_start > file_start {
            return Ok(());
        }

        for i in 0..size {
            self.0
                .swap(empty_start + i as usize, file_start + i as usize);
        }

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
    use super::{Element, FileId};

    use super::Map;

    const SIMPLE_EXAMPLE: &str = "12345";
    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn compress() {
        let mut map = Map::from_input(EXAMPLE).unwrap();

        assert_eq!(
            "00992111777.44.333....5555.6666.....8888..",
            map.compress().to_string()
        );
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
    fn find_empty_block() {
        let mut map = Map::from_input(EXAMPLE).unwrap();
        assert_eq!(Some((2, 4)), map.find_empty_block(3));
        map.0[2] = Element::File(FileId(2));
        assert_eq!(Some((8, 10)), map.find_empty_block(3));
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
