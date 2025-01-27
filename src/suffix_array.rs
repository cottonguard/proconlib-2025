pub fn suffix_array(s: &[u8]) -> Vec<usize> {
    if s.len() <= 20 {
        suffix_array_naive(s)
    } else {
        sa_is(s, 255)
    }
}

pub(crate) fn suffix_array_naive<T: Ord>(s: &[T]) -> Vec<usize> {
    let mut sa: Vec<usize> = (0..s.len()).collect();
    sa.sort_by(|&i, &j| s[i..].cmp(&s[j..]));
    sa
}

pub(crate) fn sa_is<T>(s: &[T], max: T) -> Vec<usize>
where
    T: Ord + Copy + Into<usize>,
{
    if s.is_empty() {
        return vec![];
    }
    if s.len() == 1 {
        return vec![0];
    }

    let mut ls = vec![false; s.len() + 1];
    ls[s.len()] = true;
    for i in (0..s.len()).rev() {
        ls[i] = if i + 1 < s.len() && s[i] == s[i + 1] {
            ls[i + 1]
        } else {
            i + 1 < s.len() && s[i] < s[i + 1]
        };
    }

    let mut to_lms = vec![!0; s.len()];
    let mut from_lms = vec![];
    for i in 1..s.len() {
        if !ls[i - 1] && ls[i] {
            to_lms[i] = from_lms.len();
            from_lms.push(i);
        }
    }

    let mut end = vec![0; max.into() + 1];
    for i in (0..s.len()).rev() {
        end[s[i].into()] += 1;
    }
    for i in 0..end.len() - 1 {
        end[i + 1] += end[i];
    }

    let mut sa = vec![!0; s.len()];
    let mut lms_end = end.clone();
    for i in 1..s.len() - 1 {
        if !ls[i - 1] && ls[i] {
            lms_end[s[i].into()] -= 1;
            sa[lms_end[s[i].into()]] = i;
        }
    }

    let induced_sort = |sa: &mut [usize], end: &mut [usize]| {
        let mut start = Vec::with_capacity(end.len());
        start.push(0);
        start.extend_from_slice(&end[..end.len() - 1]);

        // L-type
        // $
        let b = s[s.len() - 1].into();
        sa[start[b]] = s.len() - 1;
        start[b] += 1;
        for i in 0..sa.len() {
            if sa[i] != 0 && sa[i] != !0 {
                let j = sa[i] - 1;
                let b = s[j].into();
                sa[start[b]] = j;
                start[b] += 1;
            }
        }

        // S-type
        for i in (0..sa.len()).rev() {
            if sa[i] != !0 {
                let Some(j) = sa[i].checked_sub(1) else {
                    continue;
                };
                let b = s[j].into();
                end[b] -= 1;
                sa[end[b]] = j;
            }
        }
    };

    induced_sort(&mut sa, &mut end.clone());

    let mut lms_s = vec![0usize; from_lms.len()];
    let mut ch = 0;
    let mut j_prev = !0;
    for i in 0..sa.len() {
        if sa[i] > 0 && !ls[sa[i] - 1] && ls[sa[i]] {
            let j = to_lms[sa[i]];
            if j_prev != !0
                && s[from_lms[j_prev]..=from_lms.get(j_prev + 1).copied().unwrap_or(s.len() - 1)]
                    != s[sa[i]..=from_lms.get(j + 1).copied().unwrap_or(s.len() - 1)]
            {
                ch += 1;
            }
            lms_s[j] = ch;
            j_prev = j;
        }
    }

    let sa_lms = sa_is(&lms_s, ch);

    for sa in &mut sa {
        *sa = !0;
    }
    let mut lms_end = end.clone();
    for &j in sa_lms.iter().rev() {
        let i = from_lms[j];
        lms_end[s[i].into()] -= 1;
        sa[lms_end[s[i].into()]] = i;
    }
    induced_sort(&mut sa, &mut end);

    sa
}

pub fn lcp_array<T: Ord>(s: &[T], sa: &[usize]) -> Vec<usize> {
    assert_eq!(s.len(), sa.len());
    let mut rank: Vec<usize> = vec![0; s.len()];
    for (i, &sa) in sa.iter().enumerate() {
        rank[sa] = i;
    }
    let mut lcp = vec![0; s.len() - 1];
    let mut h = 0usize;
    for (i, &rank_i) in rank.iter().enumerate() {
        h = h.saturating_sub(1);
        if rank_i == 0 {
            continue;
        }
        let j = sa[rank[i] - 1];
        while s.get(i + h) == s.get(j + h) {
            h += 1;
        }
        lcp[rank_i - 1] = h;
    }
    lcp
}
