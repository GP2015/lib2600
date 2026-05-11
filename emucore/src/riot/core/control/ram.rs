use crate::{
    common::line::{bus::Bus, error::LineError},
    riot::core::{Riot, states::RiotStates},
};

impl Riot {
    // Only instruction that writes to ram
    pub fn call_ram(
        &mut self,
        db: &mut Bus<8>,
        states: &RiotStates,
        only_possible: bool,
    ) -> Result<(), LineError> {
        let (rw_low, rw_high) = states.rw.could_read_low_high();

        if rw_low {
            db.add_high_z(self.db_con)?;
        }

        for addr in states.a.iter_possible_reads() {
            let ram_byte = &mut self.ram[addr];

            if rw_high {
                let ram_byte_state = ram_byte.state();
                db.copy_from_reg_state(self.db_con, &ram_byte_state)?;
            }

            if rw_low {
                if !rw_high && only_possible {
                    ram_byte.remove_all();
                }

                ram_byte.copy_from_bus_state(&states.db);
            }
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::riot::control::TestUtils;
//     use rstest::{fixture, rstest};

//     #[fixture]
//     fn tu() -> TestUtils {
//         TestUtils::new()
//     }

//     #[rstest]
//     #[case(&[0], &[0], &[1], &[0])]
//     #[case(&[0], &[0], &[1], &[0, 1])]
//     #[case(&[0, 1], &[0], &[1], &[0, 1])]
//     #[case(&[0, 1], &[0], &[1], &[0, 1])]
//     #[case(&[0], &[0, 1], &[2], &[0, 1])]
//     #[case(&[0], &[0, 1], &[2], &[0, 1, 2, 3])]
//     fn write_ram(
//         mut tu: TestUtils,
//         #[case] addrs: &[usize],
//         #[case] db_vals: &[usize],
//         #[case] ram_vals: &[usize],
//         #[case] ram_res: &[usize],
//     ) {
//         tu.rw.add_drive(tu.rw_con, false, true).unwrap();

//         for (i, &val) in addrs.iter().enumerate() {
//             tu.a.add_drive(tu.a_con, val, i == 0).unwrap();
//         }

//         for (i, &val) in db_vals.iter().enumerate() {
//             tu.db.add_drive(tu.db_con, val, i == 0).unwrap();
//         }

//         let (mut riot, mut lines, states) = tu.internals();

//         for &addr in addrs {
//             for (i, &ram_val) in ram_vals.iter().enumerate() {
//                 riot.ram.byte_mut(addr as u8).add(ram_val, i == 0).unwrap();
//             }
//         }

//         riot.call_ram(&mut lines, &states, false).unwrap();

//         for &addr in addrs {
//             let mut reads: Vec<usize> = riot.ram.byte(addr as u8).iter_possible_reads().collect();
//             reads.sort_unstable();
//             assert_eq!(reads, ram_res);
//         }
//     }

//     #[rstest]
//     #[case(&[0], &[0])]
//     #[case(&[0], &[0, 1])]
//     #[case(&[0, 1], &[0, 1])]
//     #[case(&[0, 1], &[0, 1])]
//     fn read_ram(
//         mut tu: TestUtils,
//         #[case] addrs: &[usize],
//         #[case] false: bool,
//         #[case] db_res: &[usize],
//     ) {
//         let ram_val = 0;
//         let db_val = 1;

//         tu.rw.add_drive(tu.rw_con, true, true).unwrap();

//         for (i, &addr) in addrs.iter().enumerate() {
//             tu.a.add_drive(tu.a_con, addr, i == 0).unwrap();
//         }

//         for (line, con) in tu.db.iter_mut(tu.db_con).unwrap() {
//             line.add_high_z(con, true).unwrap();
//         }

//         let (mut riot, mut lines, states) = tu.internals();

//         for &addr in addrs {
//             riot.ram.byte_mut(addr as u8).add(ram_val, true).unwrap();
//         }

//         lines.db.add_drive(riot.db_con, db_val, true).unwrap();

//         riot.call_ram(&mut lines, &states, false).unwrap();

//         let mut reads: Vec<usize> = tu.db.state().iter_possible_reads().collect();
//         reads.sort_unstable();
//         assert_eq!(reads, db_res);
//     }
// }
