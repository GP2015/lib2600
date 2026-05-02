mod edc;
mod interrupt;
mod io;
mod ram;
mod reset;
mod timer;

#[cfg(test)]
use crate::{Riot, RiotLineRefs};
#[cfg(test)]
use emutils::line::{Bus, BusConnectionId, Line, LineConnectionId};

#[cfg(test)]
struct TestUtils {
    pub a: Bus<7>,
    pub db: Bus<8>,
    pub pa: Bus<8>,
    pub pb: Bus<8>,
    pub phi2: Line,
    pub res: Line,
    pub cs1: Line,
    pub cs2: Line,
    pub rs: Line,
    pub rw: Line,
    pub irq: Line,
    pub a_con: BusConnectionId,
    pub db_con: BusConnectionId,
    pub pa_con: BusConnectionId,
    pub pb_con: BusConnectionId,
    pub phi2_con: LineConnectionId,
    pub res_con: LineConnectionId,
    pub cs1_con: LineConnectionId,
    pub cs2_con: LineConnectionId,
    pub rs_con: LineConnectionId,
    pub rw_con: LineConnectionId,
    pub irq_con: LineConnectionId,
}

#[cfg(test)]
impl TestUtils {
    pub fn new() -> Self {
        let mut a = Bus::new("a");
        let mut db = Bus::new("db");
        let mut pa = Bus::new("pa");
        let mut pb = Bus::new("pb");
        let mut phi2 = Line::new("phi2");
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
            phi2_con: phi2.create_connection(),
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
            phi2,
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
            phi2: &self.phi2,
            res: &self.res,
            cs1: &self.cs1,
            cs2: &self.cs2,
            rs: &self.rs,
            rw: &self.rw,
            irq: &mut self.irq,
        }
    }
}
