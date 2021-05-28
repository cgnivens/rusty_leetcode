// Attempt to generate all permutations of a Vec<T>
// which I'll use for problem 30
//


struct Permutator<T>{
    permutation: Vec<T>,
    k: usize,
    i: usize
}


impl Permutator<T>{
    fn new(items: Vec<T>) -> Permutator {
        Words { items: items, k: items.len(), i: 0 }
    }
}


impl Iterator for Words {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        
    }
}

pub fn permute<T>(v: &Vec<T>, k: Option<usize>) -> Vec<Vec<T>> {
   if let k = Some(k) {
        k 
   } else {
        let k = v.len();
   }

   if k = 1 {
        v
   } else {
        ();
   }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unit_test() {
        let test = vec!["foo", "bar"];
        let permutations = permute(test);
        assert_eq!(permutations, vec![vec!["foo", "bar"], vec!["bar", "foo"]]);
    }
}
