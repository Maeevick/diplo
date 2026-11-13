use diplo_core::{COLUMNS, Carnivore, Cell, Ground, LANE_LENGTH, Level, Obstacle};

const SEEDS: std::ops::Range<u64> = 0..150;

fn overlap(a: f32, a_len: f32, b: f32, b_len: f32) -> bool {
    let gap = (b - a).rem_euclid(LANE_LENGTH);
    gap < a_len || gap > LANE_LENGTH - b_len
}

#[test]
fn a_level_walled_off_by_trees_cannot_be_crossed() {
    let wall = (0..COLUMNS).map(|column| (Cell::new(column, 2), Obstacle::Tree));

    let level = Level::new(5, vec![], wall);

    assert_eq!(level.path_to_goal(), None);
}

#[test]
fn a_path_winds_around_the_trees() {
    let wall = (0..COLUMNS - 1).map(|column| (Cell::new(column, 2), Obstacle::Tree));

    let level = Level::new(5, vec![], wall);

    let steps = level.path_to_goal().expect("the last column is open");
    assert_eq!(steps.len(), 4 + 5);
}

#[test]
fn every_generated_level_can_be_crossed_from_its_start() {
    for seed in SEEDS {
        for number in 1..=20 {
            let level = Level::generate(number, seed);

            assert!(level.is_open(level.start()), "level {number}, seed {seed}");
            assert!(
                level.path_to_goal().is_some(),
                "level {number}, seed {seed}"
            );
        }
    }
}

#[test]
fn the_start_and_goal_rows_are_grass() {
    for seed in SEEDS {
        let level = Level::generate(9, seed);

        assert_eq!(level.ground(0), Ground::Grass);
        assert_eq!(level.ground(level.goal_row()), Ground::Grass);
    }
}

#[test]
fn a_lane_is_track_on_every_row_its_carnivores_fill() {
    for seed in SEEDS {
        let level = Level::generate(8, seed);

        for lane in level.lanes() {
            for row in lane.rows() {
                assert_eq!(level.ground(row), Ground::Track);
            }
        }
    }
}

#[test]
fn the_same_seed_gives_the_same_level() {
    assert_eq!(Level::generate(5, 42), Level::generate(5, 42));
    assert_ne!(Level::generate(5, 42), Level::generate(5, 43));
}

#[test]
fn the_first_level_has_only_velociraptors_and_spinosauri() {
    for seed in SEEDS {
        let level = Level::generate(1, seed);

        assert!(level.lanes().iter().all(|lane| matches!(
            lane.carnivore(),
            Carnivore::Velociraptor | Carnivore::Spinosaurus
        )));
    }
}

#[test]
fn every_carnivore_shows_up_by_the_fourth_level() {
    for carnivore in Carnivore::ALL {
        let seen = SEEDS.into_iter().any(|seed| {
            Level::generate(4, seed)
                .lanes()
                .iter()
                .any(|lane| lane.carnivore() == carnivore)
        });

        assert!(seen, "{carnivore:?}");
    }
}

#[test]
fn later_levels_are_taller_faster_and_more_crowded() {
    for seed in SEEDS {
        let first = Level::generate(1, seed);
        let tenth = Level::generate(10, seed);

        assert!(tenth.rows() > first.rows(), "seed {seed}");
        assert!(tenth.lanes().len() > first.lanes().len(), "seed {seed}");
    }
    let raptor_speed = |number| {
        SEEDS
            .flat_map(|seed| Level::generate(number, seed).lanes().to_vec())
            .find(|lane| lane.carnivore() == Carnivore::Velociraptor)
            .map(|lane| lane.speed())
            .unwrap()
    };
    assert!(raptor_speed(10) > raptor_speed(1));
}

#[test]
fn speed_and_crowding_stop_growing_so_every_level_stays_playable() {
    for seed in SEEDS {
        let level = Level::generate(60, seed);

        for lane in level.lanes() {
            let filled = lane.hunters().len() as f32 * lane.carnivore().width() as f32;
            assert!(filled <= LANE_LENGTH * 0.5, "seed {seed}");
            assert!(lane.speed() <= 3.5, "seed {seed}");
        }
        assert!(level.rows() <= 40, "seed {seed}");
    }
}

#[test]
fn no_hunter_starts_on_a_rock_or_on_another_hunter() {
    for seed in SEEDS {
        for number in 1..=12 {
            let level = Level::generate(number, seed);

            for lane in level.lanes() {
                let width = lane.carnivore().width() as f32;
                for (i, hunter) in lane.hunters().iter().enumerate() {
                    for other in &lane.hunters()[i + 1..] {
                        assert!(
                            !overlap(hunter.x(), width, other.x(), width),
                            "level {number}, seed {seed}"
                        );
                    }
                    for (cell, _) in level
                        .obstacles()
                        .filter(|(cell, _)| lane.rows().contains(&cell.row))
                    {
                        assert!(
                            !overlap(hunter.x(), width, cell.column as f32, 1.0),
                            "level {number}, seed {seed}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn a_level_whose_start_is_under_a_tree_cannot_be_crossed() {
    let level = Level::new(5, vec![], [(Cell::new(COLUMNS / 2, 0), Obstacle::Tree)]);

    assert_eq!(level.path_to_goal(), None);
}
