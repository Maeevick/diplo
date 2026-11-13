use diplo_core::{Carnivore, Cell, Event, Game, Heading, Herbivore, Lane, Level, Step};

fn raptor_lane_at(x: f32) -> Level {
    let lane = Lane::new(1, Carnivore::Velociraptor, 1.0).with_hunter(x, Heading::Left);
    Level::new(4, vec![lane], [])
}

#[test]
fn stepping_onto_a_hunter_costs_a_life_and_sends_the_dino_back_to_the_start() {
    let mut game = Game::on(Herbivore::Diplodocus, raptor_lane_at(5.0), 7);

    let events = game.step(Step::Up);

    assert_eq!(
        events,
        vec![Event::Moved(Cell::new(5, 1)), Event::Caught { lives: 2 }]
    );
    assert_eq!(game.player(), Cell::new(5, 0));
    assert_eq!(game.lives(), 2);
}

#[test]
fn a_hunter_running_into_the_dino_catches_it() {
    let mut game = Game::on(Herbivore::Diplodocus, raptor_lane_at(7.0), 7);
    game.step(Step::Up);

    let mut events = vec![];
    for _ in 0..20 {
        events.extend(game.tick(0.1));
    }

    assert_eq!(events, vec![Event::Caught { lives: 2 }]);
}

#[test]
fn a_hunter_brushing_the_dino_by_a_hair_does_not_catch_it() {
    let mut game = Game::on(Herbivore::Diplodocus, raptor_lane_at(6.0), 7);

    game.step(Step::Up);
    let events = game.tick(0.1);

    assert_eq!(events, vec![]);
}

#[test]
fn the_third_catch_ends_the_game_and_the_dino_stops_moving() {
    let mut game = Game::on(Herbivore::Triceratops, raptor_lane_at(5.0), 7);
    game.step(Step::Up);
    game.step(Step::Up);

    let events = game.step(Step::Up);
    let after = game.step(Step::Right);

    assert_eq!(
        events,
        vec![Event::Moved(Cell::new(5, 1)), Event::GameOver { score: 0 }]
    );
    assert!(game.is_over());
    assert_eq!(after, vec![]);
    assert_eq!(game.tick(0.1), vec![]);
}

#[test]
fn reaching_the_top_row_scores_the_level_number_and_opens_the_next_level() {
    let mut game = Game::on(Herbivore::Ankylosaurus, Level::new(3, vec![], []), 7);

    game.step(Step::Up);
    let events = game.step(Step::Up);

    assert_eq!(
        events,
        vec![
            Event::Moved(Cell::new(5, 2)),
            Event::LevelCompleted { level: 1, score: 1 }
        ]
    );
    assert_eq!(game.level_number(), 2);
    assert_eq!(game.player(), game.level().start());
    assert_eq!(game.level(), &Level::generate(2, 7));
}

#[test]
fn each_level_adds_its_own_number_to_the_score() {
    let mut game = Game::new(Herbivore::Diplodocus, 11);
    let mut completed = vec![];

    while completed.len() < 3 {
        let path = game
            .level()
            .path_to_goal()
            .expect("every level can be crossed");
        for step in path {
            for event in game.step(step) {
                if let Event::LevelCompleted { score, .. } = event {
                    completed.push(score);
                }
            }
        }
        assert_eq!(game.lives(), 3, "the hunters never move without a tick");
    }

    assert_eq!(completed, vec![1, 3, 6]);
}
