use slc_oxide::v3::atom::AtomVariant;
use slc_oxide::v3::builtin::ActionAtom;
use slc_oxide::v3::{ActionType, Metadata, Replay};
use std::io::Cursor;

#[test]
fn test_v3_run_length_encoding() {
    let metadata = Metadata::new(240.0, 0, 1);
    let mut replay = Replay::new(metadata);

    let mut action_atom = ActionAtom::new();

    for i in 0..10 {
        let base = i * 20;
        action_atom
            .add_player_action(base, ActionType::Jump, true, false)
            .unwrap();
        action_atom
            .add_player_action(base + 2, ActionType::Jump, false, false)
            .unwrap();
        action_atom
            .add_player_action(base + 10, ActionType::Left, true, false)
            .unwrap();
        action_atom
            .add_player_action(base + 12, ActionType::Left, false, false)
            .unwrap();
    }

    replay.add_atom(AtomVariant::Action(action_atom));

    let mut buffer = Vec::new();
    replay.write(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer.clone());
    let loaded_replay = Replay::read(&mut cursor).unwrap();

    if let AtomVariant::Action(atom) = &loaded_replay.atoms.atoms[0] {
        assert_eq!(atom.actions.len(), 40);
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
fn test_v3_edge_cases() {
    let metadata = Metadata::new(360.0, 999999, 42);
    let replay = Replay::new(metadata);

    let mut buffer = Vec::new();
    replay.write(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);
    let loaded_replay = Replay::read(&mut cursor).unwrap();

    assert_eq!(loaded_replay.metadata.tps, metadata.tps);
    assert_eq!(loaded_replay.metadata.seed, metadata.seed);
    assert_eq!(loaded_replay.metadata.build, metadata.build);

    let metadata2 = Metadata::new(240.0, 0, 1);
    let mut replay2 = Replay::new(metadata2);

    let mut action_atom = ActionAtom::new();
    action_atom
        .add_player_action(100, ActionType::Jump, true, false)
        .unwrap();

    replay2.add_atom(AtomVariant::Action(action_atom));

    let mut buffer2 = Vec::new();
    replay2.write(&mut buffer2).unwrap();

    let mut cursor2 = Cursor::new(buffer2.clone());
    let loaded_replay2 = Replay::read(&mut cursor2).unwrap();

    if let AtomVariant::Action(atom) = &loaded_replay2.atoms.atoms[0] {
        assert_eq!(atom.actions.len(), 1);
    } else {
        panic!("Expected ActionAtom");
    }

    let mut buffer3 = Vec::new();
    loaded_replay2.write(&mut buffer3).unwrap();
    assert_eq!(buffer2, buffer3);

    let mut cursor3 = Cursor::new(buffer3);
    let loaded_replay3 = Replay::read(&mut cursor3).unwrap();

    if let (AtomVariant::Action(atom1), AtomVariant::Action(atom2)) = (
        &loaded_replay2.atoms.atoms[0],
        &loaded_replay3.atoms.atoms[0],
    ) {
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
