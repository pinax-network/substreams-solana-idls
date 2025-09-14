#[cfg(test)]
mod tests {
    use dflow::v4;
    use substreams::hex;

    #[test]
    fn decode_swap_instruction() {
        let bytes = hex!("f8c69e91e17587c8010000000480c3c9010000000001004fa747030000000032000000");
        match v4::instructions::unpack(&bytes).expect("decode instruction") {
            v4::instructions::DflowV4Instruction::Swap(ix) => {
                assert_eq!(ix.params.actions.len(), 1);
                match &ix.params.actions[0] {
                    v4::instructions::Action::MeteoraDlmmSwap(opts) => {
                        assert_eq!(opts.amount, 30_000_000);
                        assert_eq!(opts.num_bin_arrays, 1);
                        assert_eq!(opts.orchestrator_flags.flags, 0);
                    }
                    other => panic!("unexpected action: {other:?}"),
                }
                assert_eq!(ix.params.quoted_out_amount, 55_027_535);
                assert_eq!(ix.params.slippage_bps, 50);
                assert_eq!(ix.params.platform_fee_bps, 0);
            }
            _ => panic!("expected swap instruction"),
        }
    }

    #[test]
    fn decode_swap_event() {
        let bytes = hex!("e445a52e51cb9a1d40c6cde8260871e204e9e12fbc84e826c932cce9e2640cce15590c1c6273b0925708ba3b8520b0bcc6fa7af3bedbad3a3d65f36aabc97431b1bbe4c2d2f6e0e47ca60203452f5d6180c3c901000000000479d9c7cc1035de7211f99eb48c09d70b2bdf5bdf9e2e56b8a1fbb5a2ea332712f2460300000000");
        let event_bytes = &bytes[8..];
        match v4::events::unpack(event_bytes).expect("decode event") {
            v4::events::DflowV4Event::Swap(ev) => {
                assert_eq!(ev.amm.to_string(), "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");
                assert_eq!(ev.input_mint.to_string(), "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
                assert_eq!(ev.input_amount, 30_000_000);
                assert_eq!(ev.output_mint.to_string(), "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN");
                assert_eq!(ev.output_amount, 54_981_138);
            }
            _ => panic!("expected swap event"),
        }
    }
}
