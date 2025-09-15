#[cfg(test)]
mod tests {
    use base64::Engine;
    use heaven;
    use substreams::hex;

    #[test]
    fn unpack_sell_instruction() {
        // https://solscan.io/tx/32yEf5etU6dMSe2MMe9davZ4zHNi94HEYVJYL6ew9PzCcA2PK4KBhSD9DGbDAeds5CsvFyFRPF4QHhpgXbtURi2y
        let bytes = hex!("33e685a4017f83ad948526a45c050000000000000000000000000000");

        match heaven::instructions::unpack(&bytes).expect("decode instruction") {
            heaven::instructions::HeavenInstruction::Sell(ix) => {
                assert_eq!(ix.amount_in, 5_895_449_118_100);
                assert_eq!(ix.minimum_amount_out, 0);
                assert_eq!(ix.encoded_user_defined_event_data, "");
            }
            _ => panic!("expected Sell instruction"),
        }
    }

    #[test]
    fn unpack_sell_log() {
        // https://solscan.io/tx/32yEf5etU6dMSe2MMe9davZ4zHNi94HEYVJYL6ew9PzCcA2PK4KBhSD9DGbDAeds5CsvFyFRPF4QHhpgXbtURi2y
        let base64 = "vdt/007mYe5nk11q92ZpANtuuawSAQAAEuZMwgMAAAChqjUAAAAAAF/utLFpKG5AlIUmpFwFAAAAAAAAAAAAAAAAAAAAAAAA1MT5DQAAAAA=";
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64).expect("decode base64");

        match heaven::logs::unpack(&bytes).expect("decode log") {
            heaven::logs::HeavenLog::Sell(log) => {
                assert_eq!(log.user.to_string(), "7yKKk4eCqVvSrMpiBMB8HzmeQQeivkxj1W2yJLq7to9Z");
                assert_eq!(log.mint.to_string(), "7TUqwzuPcoWmiWLkEdTP4z1ndGMom2myLW3zuJ2w4sKm");
                assert_eq!(log.amount, 234_472_660);
            }
            _ => panic!("expected Sell log"),
        }
    }
}
