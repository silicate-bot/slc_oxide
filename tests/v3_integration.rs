use slc_oxide::{Meta, Replay};
use std::fs;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
struct TestMeta([u8; 64]);

impl Meta for TestMeta {
    fn size() -> u64 {
        64
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut data = [0u8; 64];
        data[..bytes.len().min(64)].copy_from_slice(&bytes[..bytes.len().min(64)]);
        TestMeta(data)
    }

    fn to_bytes(&self) -> Box<[u8]> {
        Box::from(self.0.as_slice())
    }
}

#[test]
fn test_macro_files_roundtrip() {
    let macro_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("macros");

    let entries = fs::read_dir(&macro_dir).expect("Failed to read macros directory");

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("slc") {
            let file_data = fs::read(&path).expect("Failed to read file");

            let mut reader = BufReader::new(Cursor::new(&file_data));
            let replay = Replay::<TestMeta>::read(&mut reader).expect("Failed to parse replay");

            let mut v2_buffer = Vec::new();
            replay
                .write(&mut v2_buffer)
                .expect("Failed to write v2 replay");

            let mut reader2 = BufReader::new(Cursor::new(&v2_buffer));
            let replay2 =
                Replay::<TestMeta>::read(&mut reader2).expect("Failed to re-parse v2 replay");

            assert_eq!(replay.tps, replay2.tps);
            assert_eq!(replay.inputs.len(), replay2.inputs.len());
            for (i, (input1, input2)) in replay.inputs.iter().zip(&replay2.inputs).enumerate() {
                assert_eq!(
                    input1.frame, input2.frame,
                    "V2 roundtrip: frame mismatch at action {}",
                    i
                );
                assert_eq!(
                    input1.delta, input2.delta,
                    "V2 roundtrip: delta mismatch at action {}",
                    i
                );
                assert_eq!(
                    input1.data, input2.data,
                    "V2 roundtrip: data mismatch at action {}",
                    i
                );
            }

            let mut v3_buffer = Vec::new();
            replay
                .write_v3(&mut v3_buffer)
                .expect("Failed to write v3 replay");

            let mut reader3 = BufReader::new(Cursor::new(&v3_buffer));
            let replay3 =
                Replay::<TestMeta>::read(&mut reader3).expect("Failed to parse v3 replay");

            assert_eq!(replay.tps, replay3.tps);
            assert_eq!(replay.inputs.len(), replay3.inputs.len());
            for (i, (input1, input3)) in replay.inputs.iter().zip(&replay3.inputs).enumerate() {
                assert_eq!(
                    input1.frame, input3.frame,
                    "V3 roundtrip: frame mismatch at action {}",
                    i
                );
                assert_eq!(
                    input1.delta, input3.delta,
                    "V3 roundtrip: delta mismatch at action {}",
                    i
                );
                assert_eq!(
                    input1.data, input3.data,
                    "V3 roundtrip: data mismatch at action {}",
                    i
                );
            }
        }
    }
}
