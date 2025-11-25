use slc_oxide::v3::atom::AtomVariant;
use slc_oxide::v3::builtin::ActionAtom;
use slc_oxide::v3::{ActionType, Metadata, Replay};
use std::io::Cursor;

#[test]
fn test_v3_basic_features() {
    let metadata = Metadata::new(240.0, 12345, 1);
    let mut replay = Replay::new(metadata);

    let mut action_atom = ActionAtom::new();
    action_atom
        .add_player_action(100, ActionType::Jump, true, false)
        .unwrap();
    action_atom
        .add_player_action(102, ActionType::Jump, false, false)
        .unwrap();
    action_atom
        .add_player_action(200, ActionType::Left, true, true)
        .unwrap();
    action_atom
        .add_player_action(201, ActionType::Left, false, true)
        .unwrap();
    action_atom
        .add_player_action(300, ActionType::Right, true, false)
        .unwrap();
    action_atom
        .add_player_action(301, ActionType::Right, false, false)
        .unwrap();

    replay.add_atom(AtomVariant::Action(action_atom));

    let mut buffer = Vec::new();
    replay.write(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);
    let loaded_replay = Replay::read(&mut cursor).unwrap();

    assert_eq!(loaded_replay.metadata.tps, metadata.tps);
    assert_eq!(loaded_replay.metadata.seed, metadata.seed);
    assert_eq!(loaded_replay.metadata.build, metadata.build);

    if let AtomVariant::Action(atom) = &loaded_replay.atoms.atoms[0] {
        assert_eq!(atom.actions.len(), 6);
        assert!(!atom.actions[0].player2);
        assert!(atom.actions[2].player2);
    } else {
        panic!("Expected ActionAtom");
    }

    let mut buffer2 = Vec::new();
    loaded_replay.write(&mut buffer2).unwrap();

    let mut cursor2 = Cursor::new(buffer2);
    let loaded_replay2 = Replay::read(&mut cursor2).unwrap();

    if let (AtomVariant::Action(atom1), AtomVariant::Action(atom2)) = (
        &loaded_replay.atoms.atoms[0],
        &loaded_replay2.atoms.atoms[0],
    ) {
        assert_eq!(atom1.actions.len(), atom2.actions.len());
        for (i, (action1, action2)) in atom1.actions.iter().zip(&atom2.actions).enumerate() {
            assert_eq!(
                action1.frame, action2.frame,
                "frame mismatch at action {}",
                i
            );
            assert_eq!(
                action1.action_type, action2.action_type,
                "action_type mismatch at action {}",
                i
            );
            assert_eq!(
                action1.holding, action2.holding,
                "holding mismatch at action {}",
                i
            );
            assert_eq!(
                action1.player2, action2.player2,
                "player2 mismatch at action {}",
                i
            );
        }
    }
}

#[test]
fn test_v3_special_actions() {
    let metadata = Metadata::new(240.0, 54321, 2);
    let mut replay = Replay::new(metadata);

    let mut action_atom = ActionAtom::new();
    action_atom
        .add_player_action(50, ActionType::Jump, true, false)
        .unwrap();
    action_atom
        .add_player_action(52, ActionType::Jump, false, false)
        .unwrap();
    action_atom
        .add_death_action(100, ActionType::Restart, 99999)
        .unwrap();
    action_atom.add_tps_action(200, 480.0).unwrap();
    action_atom
        .add_death_action(300, ActionType::RestartFull, 11111)
        .unwrap();
    action_atom
        .add_death_action(400, ActionType::Death, 22222)
        .unwrap();

    replay.add_atom(AtomVariant::Action(action_atom));

    let mut buffer = Vec::new();
    replay.write(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);
    let loaded_replay = Replay::read(&mut cursor).unwrap();

    if let AtomVariant::Action(atom) = &loaded_replay.atoms.atoms[0] {
        assert_eq!(atom.actions[2].action_type, ActionType::Restart);
        assert_eq!(atom.actions[2].seed, 99999);
        assert_eq!(atom.actions[3].action_type, ActionType::TPS);
        assert_eq!(atom.actions[3].tps, 480.0);
        assert_eq!(atom.actions[4].action_type, ActionType::RestartFull);
        assert_eq!(atom.actions[5].action_type, ActionType::Death);
    } else {
        panic!("Expected ActionAtom");
    }

    let mut buffer2 = Vec::new();
    loaded_replay.write(&mut buffer2).unwrap();

    let mut cursor2 = Cursor::new(buffer2);
    let loaded_replay2 = Replay::read(&mut cursor2).unwrap();

    if let (AtomVariant::Action(atom1), AtomVariant::Action(atom2)) = (
        &loaded_replay.atoms.atoms[0],
        &loaded_replay2.atoms.atoms[0],
    ) {
        assert_eq!(atom1.actions.len(), atom2.actions.len());
        for (i, (action1, action2)) in atom1.actions.iter().zip(&atom2.actions).enumerate() {
            assert_eq!(
                action1.frame, action2.frame,
                "frame mismatch at action {}",
                i
            );
            assert_eq!(
                action1.action_type, action2.action_type,
                "action_type mismatch at action {}",
                i
            );
            assert_eq!(
                action1.holding, action2.holding,
                "holding mismatch at action {}",
                i
            );
            assert_eq!(
                action1.player2, action2.player2,
                "player2 mismatch at action {}",
                i
            );
            if matches!(
                action1.action_type,
                ActionType::Restart | ActionType::RestartFull | ActionType::Death
            ) {
                assert_eq!(action1.seed, action2.seed, "seed mismatch at action {}", i);
            }
            if action1.action_type == ActionType::TPS {
                assert_eq!(action1.tps, action2.tps, "tps mismatch at action {}", i);
            }
        }
    }
}

#[test]
fn test_v3_large_replay() {
    let metadata = Metadata::new(240.0, 98765, 3);
    let mut replay = Replay::new(metadata);

    let mut action_atom = ActionAtom::new();

    for i in 0..100 {
        action_atom
            .add_player_action(i * 10, ActionType::Jump, true, false)
            .unwrap();
        action_atom
            .add_player_action(i * 10 + 2, ActionType::Jump, false, false)
            .unwrap();
    }

    action_atom
        .add_death_action(1500, ActionType::Restart, 55555)
        .unwrap();

    for i in 0..50 {
        action_atom
            .add_player_action(2000 + i * 5, ActionType::Right, i % 2 == 0, false)
            .unwrap();
    }

    replay.add_atom(AtomVariant::Action(action_atom));

    let mut buffer = Vec::new();
    replay.write(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer.clone());
    let loaded_replay = Replay::read(&mut cursor).unwrap();

    if let AtomVariant::Action(atom) = &loaded_replay.atoms.atoms[0] {
        assert_eq!(atom.actions.len(), 251);
    } else {
        panic!("Expected ActionAtom");
    }

    let mut buffer2 = Vec::new();
    loaded_replay.write(&mut buffer2).unwrap();
    assert_eq!(buffer, buffer2);

    let mut cursor2 = Cursor::new(buffer2);
    let loaded_replay2 = Replay::read(&mut cursor2).unwrap();

    if let (AtomVariant::Action(atom1), AtomVariant::Action(atom2)) = (
        &loaded_replay.atoms.atoms[0],
        &loaded_replay2.atoms.atoms[0],
    ) {
        assert_eq!(atom1.actions.len(), atom2.actions.len());
        for (i, (action1, action2)) in atom1.actions.iter().zip(&atom2.actions).enumerate() {
            assert_eq!(
                action1.frame, action2.frame,
                "frame mismatch at action {}",
                i
            );
            assert_eq!(
                action1.action_type, action2.action_type,
                "action_type mismatch at action {}",
                i
            );
            assert_eq!(
                action1.holding, action2.holding,
                "holding mismatch at action {}",
                i
            );
            assert_eq!(
                action1.player2, action2.player2,
                "player2 mismatch at action {}",
                i
            );
            if matches!(
                action1.action_type,
                ActionType::Restart | ActionType::RestartFull | ActionType::Death
            ) {
                assert_eq!(action1.seed, action2.seed, "seed mismatch at action {}", i);
            }
            if action1.action_type == ActionType::TPS {
                assert_eq!(action1.tps, action2.tps, "tps mismatch at action {}", i);
            }
        }
    }
}
