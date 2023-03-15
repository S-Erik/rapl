use super::*;
impl<T: Clone + Debug + Default + Display, const R: usize> Display for Ndarr<T, R> {
    // Kind of nasty function, it can be imprube a lot, but I think there is no scape from recursion.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //convert to string
        let strs: Vec<String> = self.data.iter().map(|x| x.to_string()).collect();
        // len of each strings
        let binding: Vec<usize> = strs.clone().iter().map(|s| s.len()).collect();
        // max len ( for formatting)
        let max_size = binding.iter().max().unwrap();
        //format each string
        let mut fmt_str: Vec<String> = strs
            .iter()
            .map(|s| helpers::format_vla(s.to_string(), max_size))
            .collect();

        let mut splits = self.shape.clone();
        //splits.reverse();

        fn slip_format<'a>(strings: &'a mut [String], splits: &'a [usize]) -> () {
            if splits.len() == 0 {
                return;
            }
            let l = helpers::multiply_list(splits, 1);
            let n_splits = strings.len() / l;
            for i in 0..n_splits {
                let new_s: &mut [String] = &mut strings[i * l..(i + 1) * l];
                new_s[0].insert_str(0, "[");
                new_s[l - 1].push_str("]");
                slip_format(new_s, &splits[1..]);
            }
            return;
        }
        // TODO: add new lines in the correct places to display it more numpy like
        slip_format(&mut fmt_str[0..], &mut splits[..]);

        let out = fmt_str.clone().join(" ");
        write!(f, "Ndarr({})", out)
    }
}