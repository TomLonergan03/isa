use rand::{thread_rng, Rng};
use simulator::{self, processor::Processor, types::RunState};

#[test]
fn add() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..65535);
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x0223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], a.wrapping_add(b));
    }
}

#[test]
fn subtract() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..65535);
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x1223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        if a < b {
            assert_eq!(_dump_registers[2], (a as i16).wrapping_sub(b as i16) as u16);
        } else {
            assert_eq!(_dump_registers[2], a.wrapping_sub(b));
        }
    }
}

#[test]
fn and() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..65535);
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x2223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], a & b);
    }
}

#[test]
fn or() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..65535);
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x3223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], a | b);
    }
}

#[test]
fn set_if_less() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..65535);
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x4223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        if (a < b) != (_dump_registers[2] == 1) {
            println!("a: {:016b}, b: {:016b}", a, b);
        }
        assert_eq!(_dump_registers[2], (a < b) as u16);
    }
}

#[test]
fn set_if_equal() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let is_equal = rng.gen_bool(0.5);
        let b = match is_equal {
            true => a,
            false => rng.gen_range(0..65535),
        };
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x5223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        if (a < b) != (_dump_registers[2] == 1) {
            println!("a: {}, b: {}", a, b);
        }
        assert_eq!(_dump_registers[2], (a == b) as u16);
    }
}

#[test]
fn shift_left() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..32); // 32 means 50% shifts are valid, 50% overflow
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x6223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], a.checked_shl(b as u32).unwrap_or(0));
    }
}

#[test]
fn shift_right_logical() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..32); // 32 means 50% shifts are valid, 50% overflow
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x7223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], a.checked_shr(b as u32).unwrap_or(0));
    }
}

#[test]
fn shift_right_arithmetic() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let b = rng.gen_range(0..32); // 32 means 50% shifts are valid, 50% overflow
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x8223;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        let shift_result: u16 = a.checked_shr(b as u32).unwrap_or(0);
        let result: u16 =
            shift_result | ((0b1111111111111111 << shift_result.leading_zeros()) & 0xFFFF) as u16;
        assert_eq!(_dump_registers[2], result);
    }
}

#[test]
fn set_lower() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..256);
        let register_state = [0; 16];
        let mut memory_state = [0; 65536];
        memory_state[0] = 0x9200 | a;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], a);
    }
}

#[test]
fn set_upper() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);
        let mut register_state = [0; 16];
        register_state[2] = a;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0xA200 + b;
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[2], b << 8 | a);
    }
}

#[test]
fn load_word() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let word: u16 = rng.gen_range(0..65535);
        let address: u16 = rng.gen_range(0..65520); // address + offset < 65536 to fit in u16
        let offset: u16 = rng.gen_range(0..16);
        let mut register_state = [0; 16];
        register_state[2] = address;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0xB032 | (offset << 12);
        memory_state[1] = 0xF100;
        memory_state[(address + offset) as usize] = word;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        if _dump_registers[2] != word {
            println!("word: {}, address: {}, offset: {}", word, address, offset);
            println!("expected: {}, actual: {}", word, _dump_registers[2]);
        }
        assert_eq!(_dump_registers[3], word);
    }
}

#[test]
fn save_word() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let word: u16 = rng.gen_range(0..65535);
        let address: u16 = rng.gen_range(0..65520); // address + offset < 65536 to fit in u16
        let offset: u16 = rng.gen_range(0..16);
        let mut register_state = [0; 16];
        register_state[2] = address;
        register_state[3] = word;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0xC032 | (offset << 12);
        memory_state[1] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_memory[(address + offset) as usize], word);
    }
}

#[test]
fn set_pc_if() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(0..65535);
        let is_equal = rng.gen_bool(0.5);
        let b = match is_equal {
            true => a,
            false => rng.gen_range(0..65535),
        };
        let mut register_state = [0; 16];
        register_state[2] = a;
        register_state[3] = b;
        let mut memory_state = [0; 65536];
        memory_state[0] = 0xD223;
        memory_state[1] = 0x9301;
        memory_state[2] = 0xF100;
        let mut processor = Processor::new_from_array(register_state, memory_state, false);
        let mut running = RunState::Continue;
        while running == RunState::Continue {
            running = processor.run();
        }
        let (_dump_registers, _dump_memory) = processor.coredump(false);
        assert_eq!(_dump_registers[3], (a == b) as u16);
    }
}
