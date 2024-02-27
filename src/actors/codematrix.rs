use actix::prelude::*;

use crate::toolbox::levenshtein_distance;

/// Symbols that can appear in the code matrix
#[derive(Debug)]
pub(crate) enum CodeMatrix {
    /// 1C
    _1C,
    /// 55
    _55,
    /// 7A
    _7A,
    /// BD
    _BD,
    /// E9
    _E9,
    /// FF
    _FF,
}

impl From<&str> for CodeMatrix {
    fn from(value: &str) -> Self {
        let distances: [usize; 6] = [
            levenshtein_distance("1C", value),
            levenshtein_distance("55", value),
            levenshtein_distance("7A", value),
            levenshtein_distance("BD", value),
            levenshtein_distance("E9", value),
            levenshtein_distance("FF", value),
        ];
        let mut least_distance: &usize = &distances[0];
        let mut index = 0;
        for (i, x) in distances.iter().enumerate() {
            if x < least_distance {
                least_distance = x;
                index = i;
            }
        }
        match index {
            0 => Self::_1C,
            1 => Self::_55,
            2 => Self::_7A,
            3 => Self::_BD,
            4 => Self::_E9,
            5 => Self::_FF,
            _ => unreachable!(),
        }
    }
}

/// Actor that handles Code Matrix functions
pub(crate) struct CodeMatrixActor;

impl Actor for CodeMatrixActor {
    type Context = Context<Self>;
}
