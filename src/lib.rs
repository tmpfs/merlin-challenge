#[macro_export]
macro_rules! challenge_labels {
    () => {
        const STAT_PARAM: usize = 16;
        seq_macro::seq!(N in 0..16 {
            const CHALLENGE_LABELS: [&'static [u8]; STAT_PARAM] = [
                #(
                    &[
                        0x50,
                        0x69,
                        0x4d,
                        0x6f,
                        0x64,
                        0x43,
                        0x68,
                        0x61,
                        0x6c,
                        0x6c,
                        0x65,
                        0x6e,
                        0x67,
                        0x65,
                        (N as usize).to_le_bytes()[0],
                        (N as usize).to_le_bytes()[1],
                        (N as usize).to_le_bytes()[2],
                        (N as usize).to_le_bytes()[3],
                        (N as usize).to_le_bytes()[4],
                        (N as usize).to_le_bytes()[5],
                        (N as usize).to_le_bytes()[6],
                        (N as usize).to_le_bytes()[7],
                    ],
                )*
            ];
        });
    }
}
