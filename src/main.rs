use std::iter::Iterator;

fn vec_compare(va: &[char], vb: &[char]) -> bool {
    (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| a == b)
}

fn bwt(source: &str) -> (Vec<char>, usize) {
    let input: Vec<char> = source.chars().collect();
    let mut aux = Vec::<Vec<char>>::new();
    let mut result = Vec::<char>::new();
    let mut index = 0usize;
    for i in 0..input.len() {
        let mut head: Vec<char> = input
            .iter()
            .skip(i)
            .take(input.len() - i)
            .cloned()
            .collect();
        let mut tail: Vec<char> = input.iter().take(i).cloned().collect();
        head.append(&mut tail);
        aux.push(head);
    }
    aux.sort();
    for a in &aux {
        if !vec_compare(a, &input) {
            index += 1;
        }
    }
    for s in aux {
        result.push(s[s.len() - 1]);
    }
    (result, index)
}

fn ibwt(source: &str, idx: usize) -> Vec<char> {
    let input: Vec<char> = source.chars().collect();
    let mut aux = Vec::<Vec<char>>::new();
    for i in 0..input.len() {
        aux.push(Vec::<char>::new());
        aux[i].append(&mut input.clone());
        aux.sort();
    }
    return aux[idx].clone();
}

fn main() {
    let input = "the theta, there and there, was her";
    let (target, index) = bwt(input);
    let target: String = target.into_iter().collect();
    assert_eq!(target, "es,de,aet wnrhrhhhhttt taeeeaer    ");
    let source = ibwt(input, index);
    let inverse_target: String = source.into_iter().collect();
    assert_eq!(inverse_target, input);
}
