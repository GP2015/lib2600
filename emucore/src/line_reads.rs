// let mut inp1: BitStringState<_> = [BitState::Unknown; _].into();
// for (bit, read) in inp1.iter_mut().enumerate() {
//     let ident = LineIdent::BusLine {
//         bus_name: "inp1",
//         bit,
//     };

//     *read = if bit < 4 {
//         let drives = [ext_drives.inp1[bit], riot.pa_out[bit + 4]].into_iter();
//         LineOutput::contend(drives)
//             .map_short(ident)?
//             .read()
//             .map_imp(ident)?
//     } else {
//         ext_drives.inp1[bit].read().map_imp(ident)?
//     }
// }

// let mut inp2: BitStringState<_> = [BitState::Unknown; _].into();
// for (bit, read) in inp2.iter_mut().enumerate() {
//     let ident = LineIdent::BusLine {
//         bus_name: "inp2",
//         bit,
//     };

//     *read = if bit < 4 {
//         let drives = [ext_drives.inp2[bit], riot.pa_out[bit]].into_iter();
//         LineOutput::contend(drives)
//             .map_short(ident)?
//             .read()
//             .map_imp(ident)?
//     } else {
//         ext_drives.inp2[bit].read().map_imp(ident)?
//     };
// }

// create_buses!(
//     (a, [&ext_drives.a, &cpu.a_out]),
//     (db, [&ext_drives.db, &cpu.db_out, &riot.db_out])
// );
// create_lines!(
//     (rdiff, [ext_drives.rdiff, riot.pb_out[4]]),
//     (ldiff, [ext_drives.ldiff, riot.pb_out[3]]),
//     (col, [ext_drives.col, riot.pb_out[2]]),
//     (sel, [ext_drives.sel, riot.pb_out[1]]),
//     (res, [ext_drives.sel, riot.pb_out[0]])
// );

// let rw = cpu.rw_out.read().map_imp("rw".into())?;
// let rdy = BitState::Unknown;

// *self = Self {
//     a,
//     db,
//     inp1,
//     inp2,
//     rdiff,
//     ldiff,
//     col,
//     sel,
//     res,
//     rw,
//     rdy,
// };

// Ok(())
