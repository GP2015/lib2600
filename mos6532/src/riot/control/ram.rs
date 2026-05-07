use crate::{
    Riot,
    riot::lines::{RiotLineStates, RiotOutputLines},
};
use emutils::line::LineError;

impl Riot {
    pub(crate) fn call_ram(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
    ) -> Result<(), LineError> {
        let (rw_low, rw_high) = states.rw.could_read_low_high();

        if rw_low {
            self.write_ram(states);
        }

        if rw_high {
            self.read_ram(lines, states)?;
        }

        Ok(())
    }

    fn write_ram(&mut self, states: &RiotLineStates) {
        for address in states.a.iter_possible_reads() {
            self.ram
                .byte_mut(address as u8)
                .copy_from_bus_state(&states.db, false);
        }
    }

    fn read_ram(
        &self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
    ) -> Result<(), LineError> {
        for address in states.a.iter_possible_reads() {
            lines
                .db
                .copy_from_reg(self.con.db, self.ram.byte(address as u8))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::riot::control::TestUtils;
    use rstest::{fixture, rstest};

    #[fixture]
    fn tu() -> TestUtils {
        TestUtils::new()
    }

    #[rstest]
    #[case(&[0], &[0], &[1], &[0])]
    #[case(&[0], &[0], &[1], &[0, 1])]
    #[case(&[0, 1], &[0], &[1], &[0, 1])]
    #[case(&[0, 1], &[0], &[1], &[0, 1])]
    #[case(&[0], &[0, 1], &[2], &[0, 1])]
    #[case(&[0], &[0, 1], &[2], &[0, 1, 2, 3])]
    fn write_ram(
        mut tu: TestUtils,
        #[case] addrs: &[usize],
        #[case] db_vals: &[usize],
        #[case] ram_vals: &[usize],
        #[case] ram_res: &[usize],
    ) {
        tu.rw.add_drive(tu.rw_con, false, true).unwrap();

        for (i, &val) in addrs.iter().enumerate() {
            tu.a.add_drive(tu.a_con, val, i == 0).unwrap();
        }

        for (i, &val) in db_vals.iter().enumerate() {
            tu.db.add_drive(tu.db_con, val, i == 0).unwrap();
        }

        let (mut riot, mut lines, states) = tu.internals();

        for &addr in addrs {
            for (i, &ram_val) in ram_vals.iter().enumerate() {
                riot.ram.byte_mut(addr as u8).add(ram_val, i == 0).unwrap();
            }
        }

        riot.call_ram(&mut lines, &states, false).unwrap();

        for &addr in addrs {
            let mut reads: Vec<usize> = riot.ram.byte(addr as u8).iter_possible_reads().collect();
            reads.sort_unstable();
            assert_eq!(reads, ram_res);
        }
    }

    #[rstest]
    #[case(&[0], &[0])]
    #[case(&[0], &[0, 1])]
    #[case(&[0, 1], &[0, 1])]
    #[case(&[0, 1], &[0, 1])]
    fn read_ram(
        mut tu: TestUtils,
        #[case] addrs: &[usize],
        #[case] false: bool,
        #[case] db_res: &[usize],
    ) {
        let ram_val = 0;
        let db_val = 1;

        tu.rw.add_drive(tu.rw_con, true, true).unwrap();

        for (i, &addr) in addrs.iter().enumerate() {
            tu.a.add_drive(tu.a_con, addr, i == 0).unwrap();
        }

        for (line, con) in tu.db.iter_mut(tu.db_con).unwrap() {
            line.add_high_z(con, true).unwrap();
        }

        let (mut riot, mut lines, states) = tu.internals();

        for &addr in addrs {
            riot.ram.byte_mut(addr as u8).add(ram_val, true).unwrap();
        }

        lines.db.add_drive(riot.con.db, db_val, true).unwrap();

        riot.call_ram(&mut lines, &states, false).unwrap();

        let mut reads: Vec<usize> = tu.db.state().iter_possible_reads().collect();
        reads.sort_unstable();
        assert_eq!(reads, db_res);
    }
}
