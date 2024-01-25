use itertools::Itertools;

pub trait Pairs<T> {
    fn to_pairs(&self) -> Vec<(&T, &T)>;
}

impl<T> Pairs<T> for Vec<T> {
    fn to_pairs(&self) -> Vec<(&T, &T)> {
        self.iter().zip(self[1..].iter()).collect()
    }
}

pub trait GroupLines {
    fn group_lines(&self) -> Vec<Vec<String>>;
}

impl GroupLines for Vec<String> {
    fn group_lines(&self) -> Vec<Vec<String>> {
        let line_group = self.iter().group_by(|s| s.len() == 0);
        line_group
            .into_iter()
            .map(|(_, group)| group.collect::<Vec<&String>>())
            .filter(|group| group.iter().any(|line| line.len() != 0))
            .map(|group| group.iter().map(|s| s.to_string()).collect())
            .collect()
    }
}
