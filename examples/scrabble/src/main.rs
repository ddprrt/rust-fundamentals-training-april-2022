fn score(word: &str) -> u64 {
    let mut score = 0;
    for ch in word.to_lowercase().chars() {
        score += match ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0
        };
    }
    score
}

fn main() {
    println!("{}", score("Hello"));
}

#[test]
fn a_is_worth_one_point() {
    assert_eq!(score("a"), 1);
}
#[test]
fn scoring_is_case_insensitive() {
    assert_eq!(score("A"), 1);
}
#[test]
fn f_is_worth_four() {
    assert_eq!(score("f"), 4);
}
#[test]
fn two_one_point_letters_make_a_two_point_word() {
    assert_eq!(score("at"), 2);
}
#[test]
fn three_letter_word() {
    assert_eq!(score("zoo"), 12);
}
#[test]
fn medium_word() {
    assert_eq!(score("street"), 6);
}
#[test]
fn longer_words_with_valuable_letters() {
    assert_eq!(score("quirky"), 22);
}
#[test]
fn long_mixed_case_word() {
    assert_eq!(score("OxyphenButazone"), 41);
}
#[test]
fn non_english_scrabble_letters_do_not_score() {
    assert_eq!(score("pinata"), 8, "'n' should score 1");
    assert_eq!(score("piñata"), 7, "'ñ' should score 0");
}
#[test]
fn empty_words_are_worth_zero() {
    assert_eq!(score(""), 0);
}
#[test]
fn all_letters_work() {
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}
#[test]
fn german_letters_do_not_score() {
    assert_eq!(score("STRASSE"), 7, "\"SS\" should score 2");
    assert_eq!(score("STRAßE"), 5, "'ß' should score 0");
}
