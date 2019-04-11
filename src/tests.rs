#[test]
fn test_compute_ps() {
	use super::kmp::compute_lps;
	let pat = "AAAA";
	let lps = [0, 1, 2, 3];
	assert_eq!(compute_lps(pat), lps);


	let pat = "ABCDE";
	let lps = [0; 5];
	assert_eq!(compute_lps(pat), lps);


	let pat = "AABAACAABAA";
	let lps = [0, 1, 0, 1, 2, 0, 1, 2, 3, 4, 5];
	assert_eq!(compute_lps(pat), lps);


	let pat = "AAACAAAAAC";
	let lps = [0, 1, 2, 0, 1, 2, 3, 3, 3, 4];
	assert_eq!(compute_lps(pat), lps);


	let pat = "AAABAAA";
	let lps = [0, 1, 2, 0, 1, 2, 3];
	assert_eq!(compute_lps(pat), lps);
}

#[test]
fn test_kmp_search() {
	use super::kmp::search;
	use super::kmp::compute_lps;

	let txt = "ABABDABACDABABCABAB";
	let pat = "ABABCABAB";
	let sol = [10];
	assert_eq!(search(pat, txt, true), sol);


	let txt = "AAAAABAAABA";
	let pat = "AAAA";
	let sol = [0, 1];
	assert_eq!(search(pat, txt, true), sol);
}
