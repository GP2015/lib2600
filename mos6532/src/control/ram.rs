use crate::{Riot, RiotError, RiotLineRefs};

impl Riot {
    pub(crate) fn handle_ram(
        &mut self,
        lines: &mut RiotLineRefs,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        let rw_could_read_high = lines.rw.could_read_high();
        let rw_could_read_low = lines.rw.could_read_low();

        if rw_could_read_low {
            self.handle_write_ram(lines, only_possible && !rw_could_read_high);
        }

        if rw_could_read_high {
            self.handle_read_ram(lines, only_possible && !rw_could_read_low)?;
        }

        Ok(())
    }

    fn handle_write_ram(&mut self, lines: &mut RiotLineRefs, mut only_possible: bool) {
        only_possible &= lines.a.iter_possible_reads().count() == 1;

        for address in lines.a.iter_possible_reads() {
            self.ram
                .byte_mut(address as u8)
                .copy_from_bus(lines.db, only_possible)
                .expect("already checked");
        }
    }

    fn handle_read_ram(
        &mut self,
        lines: &mut RiotLineRefs,
        mut only_possible: bool,
    ) -> Result<(), RiotError> {
        only_possible &= lines.a.iter_possible_reads().count() == 1;

        for address in lines.a.iter_possible_reads() {
            lines
                .db
                .copy_from_reg(&self.db_con, self.ram.byte(address as u8), only_possible)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::control::TestUtils;
    use rstest::{fixture, rstest};

    #[fixture]
    fn tu() -> TestUtils {
        TestUtils::new()
    }

    #[rstest]
    #[case(&[0], &[0], &[1], true, &[0])]
    #[case(&[0], &[0], &[1], false, &[0, 1])]
    #[case(&[0, 1], &[0], &[1], true, &[0, 1])]
    #[case(&[0, 1], &[0], &[1], false, &[0, 1])]
    #[case(&[0], &[0, 1], &[2], true, &[0, 1])]
    #[case(&[0], &[0, 1], &[2], false, &[0, 1, 2, 3])]
    fn write_ram(
        mut tu: TestUtils,
        #[case] addrs: &[usize],
        #[case] db_vals: &[usize],
        #[case] ram_vals: &[usize],
        #[case] only_possible: bool,
        #[case] ram_res: &[usize],
    ) {
        tu.rw.add_drive(&tu.rw_con, false, true).unwrap();

        for (bus, con, vals) in [
            (&mut tu.a, &tu.a_con, addrs),
            (&mut tu.db, &tu.db_con, db_vals),
        ] {
            for (i, &val) in vals.iter().enumerate() {
                bus.add_drive(con, val, i == 0).unwrap();
            }
        }

        let (mut riot, mut lines) = tu.riot_and_lines();

        for &addr in addrs {
            for (i, &ram_val) in ram_vals.iter().enumerate() {
                riot.ram.byte_mut(addr as u8).add(ram_val, i == 0).unwrap();
            }
        }

        riot.handle_ram(&mut lines, only_possible).unwrap();

        for &addr in addrs {
            let mut reads: Vec<usize> = riot.ram.byte(addr as u8).iter_possible_reads().collect();
            reads.sort_unstable();
            assert_eq!(reads, ram_res);
        }
    }

    #[rstest]
    #[case(&[0], true, &[0])]
    #[case(&[0], false, &[0, 1])]
    #[case(&[0, 1], true, &[0, 1])]
    #[case(&[0, 1], false, &[0, 1])]
    fn read_ram(
        mut tu: TestUtils,
        #[case] addrs: &[usize],
        #[case] only_possible: bool,
        #[case] db_res: &[usize],
    ) {
        let ram_val = 0;
        let db_val = 1;

        tu.rw.add_drive(&tu.rw_con, true, true).unwrap();

        for (i, &addr) in addrs.iter().enumerate() {
            tu.a.add_drive(&tu.a_con, addr, i == 0).unwrap();
        }

        tu.db
            .iter_mut(&tu.db_con)
            .for_each(|(line, con)| line.add_high_z(con, true));

        let (mut riot, mut lines) = tu.riot_and_lines();

        for &addr in addrs {
            riot.ram.byte_mut(addr as u8).add(ram_val, true).unwrap();
        }

        lines.db.add_drive(&riot.db_con, db_val, true).unwrap();

        riot.handle_ram(&mut lines, only_possible).unwrap();

        let mut reads: Vec<usize> = tu.db.iter_possible_reads().collect();
        reads.sort_unstable();
        assert_eq!(reads, db_res);
    }
}
