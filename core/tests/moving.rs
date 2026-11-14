use diplo_core::{Carnivore, Cell, Event, Game, Heading, Herbivore, Lane, Level, Obstacle, Step};

fn meadow(rows: i32) -> Level {
    Level::new(rows, vec![], [])
}

#[test]
fn a_press_above_the_pad_centre_steps_up() {
    let press = (0.4, 1.2);

    let step = Step::pointed(press);

    assert_eq!(step, Some(Step::Up));
}

#[test]
fn a_press_further_aside_than_above_steps_aside() {
    assert_eq!(Step::pointed((-1.4, 0.9)), Some(Step::Left));
    assert_eq!(Step::pointed((1.1, -0.8)), Some(Step::Right));
    assert_eq!(Step::pointed((0.3, -0.7)), Some(Step::Down));
}

#[test]
fn a_press_on_the_pad_centre_asks_for_no_step() {
    assert_eq!(Step::pointed((0.5, -0.5)), None);
    assert_eq!(Step::pointed((-0.2, 0.4)), None);
}

#[test]
fn the_dino_starts_in_the_middle_of_the_bottom_row() {
    let game = Game::on(Herbivore::Diplodocus, meadow(6), 7);

    assert_eq!(game.player(), Cell::new(5, 0));
    assert_eq!(game.lives(), 3);
    assert_eq!(game.score(), 0);
    assert_eq!(game.level_number(), 1);
}

#[test]
fn the_dino_walks_one_cell_at_a_time() {
    let mut game = Game::on(Herbivore::Triceratops, meadow(6), 7);

    let events = game.step(Step::Up);
    game.step(Step::Right);

    assert_eq!(events, vec![Event::Moved(Cell::new(5, 1))]);
    assert_eq!(game.player(), Cell::new(6, 1));
}

#[test]
fn a_tree_blocks_the_dino() {
    let level = Level::new(6, vec![], [(Cell::new(5, 1), Obstacle::Tree)]);
    let mut game = Game::on(Herbivore::Ankylosaurus, level, 7);

    let events = game.step(Step::Up);

    assert_eq!(events, vec![]);
    assert_eq!(game.player(), Cell::new(5, 0));
}

#[test]
fn the_dino_never_leaves_the_meadow() {
    let mut game = Game::on(Herbivore::Diplodocus, meadow(6), 7);

    let down = game.step(Step::Down);
    for _ in 0..8 {
        game.step(Step::Left);
    }

    assert_eq!(down, vec![]);
    assert_eq!(game.player(), Cell::new(0, 0));
}

#[test]
fn a_hunter_leaving_one_side_comes_back_from_the_other() {
    let lane = Lane::new(1, Carnivore::Velociraptor, 2.0).with_hunter(14.5, Heading::Right);
    let mut game = Game::on(Herbivore::Diplodocus, Level::new(4, vec![lane], []), 7);

    game.tick(0.1);
    game.tick(0.1);
    game.tick(0.1);

    let hunter = &game.level().lanes()[0].hunters()[0];
    assert!((hunter.x() - 0.1).abs() < 1e-4, "at {}", hunter.x());
}

#[test]
fn a_hunter_turns_around_at_a_rock() {
    let lane = Lane::new(1, Carnivore::Atrociraptor, 2.0).with_hunter(6.9, Heading::Right);
    let level = Level::new(4, vec![lane], [(Cell::new(9, 1), Obstacle::Rock)]);
    let mut game = Game::on(Herbivore::Diplodocus, level, 7);

    game.tick(0.1);
    game.tick(0.1);

    let hunter = &game.level().lanes()[0].hunters()[0];
    assert_eq!(hunter.heading(), Heading::Left);
    assert!(hunter.x() < 7.0, "at {}", hunter.x());
}

#[test]
fn two_hunters_meeting_head_on_both_turn_around() {
    let lane = Lane::new(1, Carnivore::Velociraptor, 1.0)
        .with_hunter(2.0, Heading::Right)
        .with_hunter(3.05, Heading::Left);
    let mut game = Game::on(Herbivore::Diplodocus, Level::new(4, vec![lane], []), 7);

    game.tick(0.1);

    let hunters = game.level().lanes()[0].hunters();
    assert_eq!(hunters[0].heading(), Heading::Left);
    assert_eq!(hunters[1].heading(), Heading::Right);
}

#[test]
fn a_tall_hunter_turns_at_a_rock_in_any_of_its_rows() {
    let lane = Lane::new(1, Carnivore::Spinosaurus, 1.0).with_hunter(3.0, Heading::Left);
    let level = Level::new(5, vec![lane], [(Cell::new(2, 2), Obstacle::Rock)]);
    let mut game = Game::on(Herbivore::Diplodocus, level, 7);

    game.tick(0.1);

    assert_eq!(
        game.level().lanes()[0].hunters()[0].heading(),
        Heading::Right
    );
}

#[test]
fn the_game_keeps_the_dino_the_player_chose() {
    let game = Game::new(Herbivore::Ankylosaurus, 3);

    assert_eq!(game.dino(), Herbivore::Ankylosaurus);
}
