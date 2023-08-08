use std::fmt::Display;

use prettytable::{row, Table};

use crate::{primitive::DbPrimitive, union::DbUnion};

use super::DbCrate;

/* for a spindle crate of
        [ U_1, ...,  U_m]
    f:  [ X_1, ...,  X_m] -> [ Y_1, ...,  Y_n],
    f': [X_1', ..., X_m'] -> [Y_1', ..., Y_n'], etc
display the grid
    | X_1  @ x_1 | Y_1  @ y_1 | U_1 = A | B | ...
    | ...        | ...        | ...
    | X_m  @ x_m | Y_m  @ y_m | U_m = C | D | ...
    | X_1' @ x_1'| Y_1' @ y_1'|
    | ...        | ...        |
    | X_m' @ x_m'| Y_m' @ y_m'|
    etc
*/

// todo! prettier printing

impl Display for DbCrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // make sure we can print unions
        self.unions.iter().try_for_each(|u| writeln!(f, "{}", u))?;

        let mut table = Table::new();
        table.add_row(row!["input", "output", "union"]);
        // for the first lift, we will display the union
        // table.add_row(row!["X @ x", "Y @ y", "U = A | B | ..."]);
        // for now let's just display the lifts
        let mut lifts = self.lifts.iter();
        // todo! ?some more natural iter idiom
        match lifts.next() {
            Some(lift) => {
                // if there is a lift, show the union on those rows
                let positions = lift.positions.iter();
                let map = &lift.map;
                let in_outs = map.in_outs.iter();
                positions.zip(in_outs).zip(self.unions.iter()).for_each(
                    |((positions, in_out), union)| {
                        let input = PositionedField(positions.0, in_out.input.as_ref());
                        let output = PositionedField(positions.1, in_out.output.as_ref());
                        table.add_row(row![input, output, union]);
                    },
                );
            }
            None => {
                // if there are no lifts, we just display the unions
                return self.unions.iter().try_for_each(|u| writeln!(f, "{u}"));
            }
        }
        lifts.for_each(|lift| {
            let positions = lift.positions.iter();
            let map = &lift.map;
            let in_outs = map.in_outs.iter();
            positions.zip(in_outs).for_each(|(positions, in_out)| {
                let input = PositionedField(positions.0, in_out.input.as_ref());
                let output = PositionedField(positions.1, in_out.output.as_ref());
                table.add_row(row![input, output, ""]);
            });
            // table.add_row(row!["X @ x", "Y @ y", "U = A | B | ..."]);
        });
        let table = table.to_string();
        write!(f, "{table}")
    }
}

struct PositionedField<'a>(Option<usize>, Option<&'a DbPrimitive>);

impl Display for PositionedField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // e.g., "f32 @ 1" or "_ @ _"
        write!(
            f,
            "{} @ {}",
            self.1.map(|p| p.ident.as_str()).unwrap_or("_"),
            self.0.map(|p| p.to_string()).unwrap_or("_".to_string())
        )
    }
}

impl Display for DbUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // e.g., "U = A | B | ..."
        write!(
            f,
            "{} = {}",
            self.ident.as_str(),
            self.fields
                .iter()
                .map(|v| v.ident.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}
