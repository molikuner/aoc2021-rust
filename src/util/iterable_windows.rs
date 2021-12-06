pub struct IteratorWindows<I: Iterator<Item=T>, T: Copy, const N: usize> {
    current_window: [Option<T>; N],
    iterator: I,
}

impl<I: Iterator<Item=T>, T: Copy, const N: usize> IteratorWindows<I, T, N> {
    fn new(iterator: I) -> Self {
        IteratorWindows {
            current_window: [None; N],
            iterator,
        }
    }
}

impl<I: Iterator<Item=T>, T: Copy, const N: usize> Iterator for IteratorWindows<I, T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        for i in 1..N {
            self.current_window[i - 1] = self.current_window[i]
        }
        self.current_window[N - 1] = None;
        for window_item in self.current_window.iter_mut() {
            if window_item.is_some() { continue; }

            *window_item = self.iterator.next();

            if window_item.is_none() {
                return None;
            }
        }

        let window = (&self.current_window).map(|x| x.unwrap());

        Some(window)
    }
}

pub trait WindowExt<I: Iterator<Item=T>, T: Copy>: Iterator {
    fn window<const N: usize>(self) -> IteratorWindows<I, T, N>;
}

impl<I: Iterator<Item=T>, T: Copy> WindowExt<I, T> for I {
    fn window<const N: usize>(self) -> IteratorWindows<I, T, N> {
        IteratorWindows::new(self)
    }
}
