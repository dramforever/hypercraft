use crate::{HyperError, HyperResult};

#[derive(Clone, Copy, Debug)]
pub struct HsmFunction {
    pub hart_id: usize,
    pub entry: usize,
    pub opaque: usize,
}


impl HsmFunction {
    pub(crate) fn from_regs(args: &[usize]) -> HyperResult<Self> {
        (args[6] == 0).then_some(()).ok_or(HyperError::NotSupported)?;
        Ok(Self {
            hart_id: args[0],
            entry: args[1],
            opaque: args[2],
        })
    }
}
