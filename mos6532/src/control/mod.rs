mod edc;
mod interrupt;
mod io;
mod ram;
mod reset;
mod timer;

#[cfg(test)]
use crate::{Riot, RiotLineRefs};
#[cfg(test)]
use emutils::line::{Bus, BusConnection, Line, LineConnection};

#[allow(dead_code)]
#[cfg(test)]
struct TestUtils {
    pub a: Bus,
    pub db: Bus,
    pub pa: Bus,
    pub pb: Bus,
    pub res: Line,
    pub cs1: Line,
    pub cs2: Line,
    pub rs: Line,
    pub rw: Line,
    pub irq: Line,
    pub a_con: BusConnection,
    pub db_con: BusConnection,
    pub pa_con: BusConnection,
    pub pb_con: BusConnection,
    pub res_con: LineConnection,
    pub cs1_con: LineConnection,
    pub cs2_con: LineConnection,
    pub rs_con: LineConnection,
    pub rw_con: LineConnection,
    pub irq_con: LineConnection,
}

#[cfg(test)]
impl TestUtils {
    pub fn new() -> Self {
        let mut a = Bus::new("a", 7);
        let mut db = Bus::new("db", 8);
        let mut pa = Bus::new("pa", 8);
        let mut pb = Bus::new("pb", 8);
        let mut res = Line::new("res");
        let mut cs1 = Line::new("cs1");
        let mut cs2 = Line::new("cs2");
        let mut rs = Line::new("rs");
        let mut rw = Line::new("rw");
        let mut irq = Line::new("irq");

        Self {
            a_con: a.create_connection(),
            db_con: db.create_connection(),
            pa_con: pa.create_connection(),
            pb_con: pb.create_connection(),
            res_con: res.create_connection(),
            cs1_con: cs1.create_connection(),
            cs2_con: cs2.create_connection(),
            rs_con: rs.create_connection(),
            rw_con: rw.create_connection(),
            irq_con: irq.create_connection(),
            a,
            db,
            pa,
            pb,
            res,
            cs1,
            cs2,
            rs,
            rw,
            irq,
        }
    }

    pub fn riot_and_lines(&mut self) -> (Riot, RiotLineRefs<'_>) {
        (self.riot(), self.lines())
    }

    pub fn riot(&mut self) -> Riot {
        Riot::new(
            self.db.create_connection(),
            self.pa.create_connection(),
            self.pb.create_connection(),
            self.irq.create_connection(),
        )
    }

    pub fn lines(&mut self) -> RiotLineRefs<'_> {
        RiotLineRefs {
            a: &self.a,
            db: &mut self.db,
            pa: &mut self.pa,
            pb: &mut self.pb,
            res: &self.res,
            cs1: &self.cs1,
            cs2: &self.cs2,
            rs: &self.rs,
            rw: &self.rw,
            irq: &mut self.irq,
        }
    }
}
