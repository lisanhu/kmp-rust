/**
\brief compute the max possible length when the proper prefix is the suffix for pat[0..pos]

The empty string is always the proper prefix and is also the suffix, so we will return at least 0.

todo: This is not the optimal solution, need to implement the best solution in the future at https://www.geeksforgeeks.org/kmp-algorithm-for-pattern-searching/ Illustration of preprocessing (or construction of lps[]) part

\param pat pattern to be computed
\param pos the boundary to compute the longest proper prefix is the suffix
\return l the length of the longest proper prefix is the suffix
*/
fn compute_ps(pat: &str, pos: usize) -> usize {
	let pat = &pat[..pos + 1];
	let len = pat.len();
	let mut l = 0;

	for i in 0..=pos {
		if &pat[len - i..] == &pat[..i] {
			l = i;
		}
	}
	l
}


pub fn compute_lps(pat: &str) -> std::vec::Vec<usize> {
	let mut lps = vec![0; pat.len()];
	for i in 0..lps.len() {
		lps[i] = compute_ps(&pat, i);
	}
	return lps;
}


pub fn search(pat: &str, txt: &str, all: bool) -> std::vec::Vec<usize> {
	let tlen = txt.len();
	let plen = pat.len();
	let mut i = 0;
	let mut j = 0;
	let mut sol = vec![];

	let lps = compute_lps(pat);

	loop {
		if i >= tlen || j >= plen {
			break;
		}

		if &txt.as_bytes()[i] == &pat.as_bytes()[j] {
			i += 1;
			j += 1;

			if j == plen {
				if !all {
					return vec![i - j];
				} else {
					sol.push(i - j);
				};
				j = lps[j - 1];
			}
		} else {
			if j != 0 {
				j = lps[j - 1];
			} else {
				i += 1;
			}
		}
	}

	sol
}
