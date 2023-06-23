use simulator::{self, types::RunState};

#[test]
fn add() {
    let mut register_state = [0; 16];
    register_state[2] = 17;
    register_state[3] = 132;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x0223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 149);
}

#[test]
fn subtract() {
    let mut register_state = [0; 16];
    register_state[2] = 75;
    register_state[3] = 66;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x1223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 9);
}

#[test]
fn and() {
    let mut register_state = [0; 16];
    register_state[2] = 0b10101010;
    register_state[3] = 0b11110000;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x2223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0b10100000);
}

#[test]
fn or() {
    let mut register_state = [0; 16];
    register_state[2] = 0b10101010;
    register_state[3] = 0b11110000;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x3223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0b11111010);
}

#[test]
fn set_if_less_false() {
    let mut register_state = [0; 16];
    register_state[2] = 55;
    register_state[3] = 27;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x4223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0);
}

#[test]
fn set_if_less_true() {
    let mut register_state = [0; 16];
    register_state[2] = 27;
    register_state[3] = 55;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x4223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 1);
}

#[test]
fn set_if_equal_false() {
    let mut register_state = [0; 16];
    register_state[2] = 27;
    register_state[3] = 55;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x5223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0);
}

#[test]
fn set_if_equal_true() {
    let mut register_state = [0; 16];
    register_state[2] = 27;
    register_state[3] = 27;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x5223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 1);
}

#[test]
fn shift_left() {
    let mut register_state = [0; 16];
    register_state[2] = 0b10;
    register_state[3] = 3;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x6223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    println!("{:b}", _dump_registers[2]);
    println!("{:b}", 0b10000);
    assert_eq!(_dump_registers[2], 0b10000);
}

#[test]
fn shift_right_logical() {
    let mut register_state = [0; 16];
    register_state[2] = 0b1010101000000000;
    register_state[3] = 2;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x7223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0b0010101010000000);
}

#[test]
fn shift_right_arithmetic() {
    let mut register_state = [0; 16];
    register_state[2] = 0b1010101000000000;
    register_state[3] = 2;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x8223;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0b1110101010000000);
}

#[test]
fn set_lower() {
    let register_state = [0; 16];
    let mut memory_state = [0; 65536];
    memory_state[0] = 0x921F;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0x1F);
}

#[test]
fn set_upper() {
    let mut register_state = [0; 16];
    register_state[2] = 0x1F;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0xA21F;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0x1F1F);
}

#[test]
fn load_word() {
    let mut register_state = [0; 16];
    register_state[2] = 0x1F;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0xB122;
    memory_state[1] = 0xF100;
    memory_state[0x20] = 0x1234;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[2], 0x1234);
}

#[test]
fn save_word() {
    let mut register_state = [0; 16];
    register_state[2] = 0xAB24;
    register_state[3] = 5;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0xC123;
    memory_state[1] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    println!("{:X?}", _dump_memory.split_at(100).0);
    assert_eq!(_dump_memory[6], 0xAB24);
}

#[test]
fn set_pc_if_true() {
    let mut register_state = [0; 16];
    register_state[2] = 2;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0xD200;
    memory_state[1] = 0x9301;
    memory_state[2] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[3], 0);
}

#[test]
fn set_pc_if_false() {
    let mut register_state = [0; 16];
    register_state[2] = 2;
    register_state[4] = 1;
    let mut memory_state = [0; 65536];
    memory_state[0] = 0xD240;
    memory_state[1] = 0x9301;
    memory_state[2] = 0xF100;
    let mut processor =
        simulator::processor::Processor::new_from_array(register_state, memory_state);
    let mut running: RunState = RunState::Continue;
    while running == RunState::Continue {
        running = processor.run();
    }
    let (_dump_registers, _dump_memory) = processor.coredump(false);
    assert_eq!(_dump_registers[3], 1);
}