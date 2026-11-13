use diplo_core::{Initials, Leaderboard};

fn initials(text: &str) -> Initials {
    Initials::parse(text).expect("three capital letters")
}

#[test]
fn a_fresh_board_has_ten_places_all_at_one() {
    let board = Leaderboard::fresh();

    assert_eq!(board.entries().len(), 10);
    assert!(board.entries().iter().all(|entry| entry.score == 1));
}

#[test]
fn a_game_scoring_nothing_never_makes_the_board() {
    assert!(!Leaderboard::fresh().admits(0));
    assert!(Leaderboard::fresh().admits(1));
}

#[test]
fn a_new_score_takes_its_place_below_the_equal_older_ones() {
    let (board, _) = Leaderboard::fresh().record(initials("MAX"), 6);
    let (board, _) = board.record(initials("LEO"), 10);

    let (board, place) = board.record(initials("ZOE"), 6);

    assert_eq!(place, Some(2));
    let top: Vec<_> = board.entries()[..3]
        .iter()
        .map(|e| (e.initials.to_string(), e.score))
        .collect();
    assert_eq!(
        top,
        vec![("LEO".into(), 10), ("MAX".into(), 6), ("ZOE".into(), 6)]
    );
    assert_eq!(board.entries().len(), 10);
}

#[test]
fn a_score_below_the_tenth_place_is_not_recorded() {
    let mut board = Leaderboard::fresh();
    for _ in 0..10 {
        board = board.record(initials("REX"), 3).0;
    }

    let (after, place) = board.record(initials("LEO"), 2);

    assert_eq!(place, None);
    assert_eq!(after, board);
}

#[test]
fn letters_cycle_round_the_alphabet() {
    let start = initials("AZM");

    let changed = start.cycled(0, -1).cycled(1, 1).cycled(2, 2);

    assert_eq!(changed.to_string(), "ZAO");
    assert_eq!(Initials::default().to_string(), "AAA");
}

#[test]
fn initials_are_three_capital_letters() {
    assert!(Initials::parse("LEO").is_some());
    assert!(Initials::parse("leo").is_none());
    assert!(Initials::parse("LE").is_none());
    assert!(Initials::parse("L3O").is_none());
}

#[test]
fn a_board_read_back_from_its_text_is_the_same_board() {
    let (board, _) = Leaderboard::fresh().record(initials("LEO"), 21);

    assert_eq!(Leaderboard::from_text(&board.to_text()), board);
}

#[test]
fn an_unreadable_text_gives_a_fresh_board() {
    assert_eq!(
        Leaderboard::from_text("LEO 4\nnonsense"),
        Leaderboard::fresh()
    );
    assert_eq!(Leaderboard::from_text(""), Leaderboard::fresh());
    let mut lines = Leaderboard::fresh().to_text();
    lines = lines.replacen("SPI 1", "spi 1", 1);
    assert_eq!(Leaderboard::from_text(&lines), Leaderboard::fresh());
    let (board, _) = Leaderboard::fresh().record(initials("LEO"), 4);
    let broken = board.to_text().replacen("LEO 4", "LEO four", 1);
    assert_eq!(Leaderboard::from_text(&broken), Leaderboard::fresh());
}

#[test]
fn a_score_equal_to_the_tenth_place_takes_it_so_a_first_level_makes_a_fresh_board() {
    let (board, place) = Leaderboard::fresh().record(initials("LEO"), 1);

    assert_eq!(place, Some(9));
    assert_eq!(board.entries()[9].initials.to_string(), "LEO");
    assert_eq!(board.entries().len(), 10);
}
