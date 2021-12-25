use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut, Range};
use std::str::FromStr;

pub struct Field<T> {
    values: Vec<Vec<T>>,
}

impl<T: FromStr<Err=E>, E: Display> FromStr for Field<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<Vec<T>> = vec![];

        for line in s.lines() {
            let mut line_values: Vec<T> = vec![];
            for c in line.chars() {
                let res = T::from_str(c.to_string().as_str());
                if let Ok(x) = res {
                    line_values.push(x);
                } else if let Err(err) = res {
                    return Err(format!("An error occurred while reading the input: {}", err));
                }
            }
            if values.iter().any(|column| column.len() != line_values.len()) {
                return Err(format!("Unequal y-lengths: column {} is {} long", values.len(), line_values.len()));
            }
            values.push(line_values);
        }

        Ok(Field { values })
    }
}

impl<T: Debug> Debug for Field<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter_field().enumerate().fold(Ok(()), |res, (i, column)| {
            res
                .and_then(|_| write!(f, "{}", if i == 0 { "" } else { "\n" }))
                .and_then(|_| column.enumerate().fold(Ok(()), |res, (i, value)|
                    res.and_then(|_| write!(f, "{}{:?}", if i == 0 { "" } else { " " }, value))
                ))
        })
    }
}

impl<T> Index<(usize, usize)> for Field<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.values.index(x).index(y)
    }
}

impl<T> IndexMut<(usize, usize)> for Field<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.values.index_mut(x).index_mut(y)
    }
}

impl<T> Field<T> {
    pub fn x_len(&self) -> usize {
        self.values.len()
    }

    pub fn y_len(&self) -> usize {
        self.values.get(0).map(|column| column.len()).unwrap_or(0)
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter().flat_map(|column| column.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.values.iter_mut().flat_map(|column| column.iter_mut())
    }

    pub fn iter_field(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.values.iter().map(|column| column.iter())
    }

    // pub fn iter_field_mut(&mut self) -> impl Iterator<Item = impl Iterator<Item = &mut T>> {
    //     self.values.iter_mut().map(|column| column.iter_mut())
    // }

    pub fn iter_coordinates(&self) -> impl Iterator<Item=(usize, usize)> {
        let x_len = self.x_len();
        let y_len = self.y_len();

        (0..x_len).flat_map(move |x| (0..y_len).map(move |y| (x, y)))
    }

    pub fn iter_adjacent_coordinates(&self, x: usize, y: usize, include_diagonal: bool) -> impl Iterator<Item=(usize, usize)> {
        fn adj_range(start: usize, max: usize) -> Range<usize> {
            start.saturating_sub(1)..start.saturating_add(2).min(max)
        }

        let x_len = self.x_len();
        let y_len = self.y_len();

        adj_range(x, x_len).flat_map(move |nx| {
            adj_range(y, y_len).filter_map(move |ny|
                if (include_diagonal || x == nx || y == ny) && (x != nx || y != ny) {
                    Some((nx, ny))
                } else {
                    None
                }
            )
        })
    }
}
