
#[cfg(test)]
mod test {
    use scripts::opcodes::execute_script;
    use bitcoin_script::bitcoin_script as script;
    use scripts::opcodes::u32_std::u32_push;
    use scripts::opcodes::u32_add::*;
    use scripts::opcodes::pushable;
    
    #[test]
    fn test_u32_add() {
        let u32_value_a = 0xFFEEFFEEu32;
        let u32_value_b = 0xEEFFEEFFu32;

        let script = script! {
            { u32_push(u32_value_a) }
            { u32_push(u32_value_b) }
            { u32_add_drop(1, 0) }
            0xed OP_EQUALVERIFY
            0xee OP_EQUALVERIFY
            0xee OP_EQUALVERIFY
            0xee OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success)
    }
}
