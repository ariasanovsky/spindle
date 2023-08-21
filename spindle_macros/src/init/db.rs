use spindle_db::map::AsDbMap;

use crate::map::in_out::InOut;

use super::InputInitFn;

impl AsDbMap for InputInitFn {
    type InOut = InOut;

    fn db_ident(&self) -> String {
        let Self {
            item_fn,
            input_type,
            output_type,
        } = self;
        todo!()
    }

    fn db_content(&self) -> String {
        todo!()
    }

    fn db_inouts(&self) -> Vec<Self::InOut> {
        todo!()
    }

    fn range_type(&self) -> Option<String> {
        todo!()
    }
}