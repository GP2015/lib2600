mod edc;
mod interrupt;
mod io;
mod ram;
mod timer;
mod update;

#[cfg(test)]
use crate::{
    Riot, RiotConnectionIds, RiotLines,
    riot::lines::{RiotLineStates, RiotOutputLines},
};
#[cfg(test)]
use emutils::line::{Bus, BusConnectionId, Line, LineConnectionId};

#[cfg(test)]
struct TestUtils {
    pub a: Bus<7>,
    pub db: Bus<8>,
    pub pa: Bus<8>,
    pub pb: Bus<8>,
    pub cs1: Line,
    pub cs2: Line,
    pub rs: Line,
    pub rw: Line,
    pub a_con: BusConnectionId,
    pub db_con: BusConnectionId,
    pub pa_con: BusConnectionId,
    pub pb_con: BusConnectionId,
    pub cs1_con: LineConnectionId,
    pub cs2_con: LineConnectionId,
    pub rs_con: LineConnectionId,
    pub rw_con: LineConnectionId,
}

#[cfg(test)]
impl TestUtils {
    pub fn new() -> Self {
        let mut a = Bus::new("a");
        let mut db = Bus::new("db");
        let mut pa = Bus::new("pa");
        let mut pb = Bus::new("pb");
        let mut cs1 = Line::new("cs1");
        let mut cs2 = Line::new("cs2");
        let mut rs = Line::new("rs");
        let mut rw = Line::new("rw");

        Self {
            a_con: a.create_connection(),
            db_con: db.create_connection(),
            pa_con: pa.create_connection(),
            pb_con: pb.create_connection(),
            cs1_con: cs1.create_connection(),
            cs2_con: cs2.create_connection(),
            rs_con: rs.create_connection(),
            rw_con: rw.create_connection(),
            a,
            db,
            pa,
            pb,
            cs1,
            cs2,
            rs,
            rw,
        }
    }

    pub fn internals(&mut self) -> (Riot, RiotOutputLines<'_>, RiotLineStates) {
        let (riot, lines) = self.externals();
        let states = RiotLineStates::from(&lines);
        (riot, RiotOutputLines::from(lines), states)
    }

    pub fn externals(&mut self) -> (Riot, RiotLines<'_>) {
        (self.riot(), self.lines())
    }

    pub fn riot(&mut self) -> Riot {
        Riot::new(RiotConnectionIds {
            db: self.db.create_connection(),
            pa: self.pa.create_connection(),
            pb: self.pb.create_connection(),
        })
    }

    pub fn lines(&mut self) -> RiotLines<'_> {
        RiotLines {
            a: &self.a,
            db: &mut self.db,
            pa: &mut self.pa,
            pb: &mut self.pb,
            cs1: &self.cs1,
            cs2: &self.cs2,
            rs: &self.rs,
            rw: &self.rw,
        }
    }
}
