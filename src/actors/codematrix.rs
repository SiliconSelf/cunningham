use actix::prelude::*;

use crate::toolbox::algorithms::levenshtein_distance;

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

/// Errors that can occur when working with a code matrix
pub(crate) enum CodeMatrixError {
    /// The Levenshtein Difference between the input and any possible match
    /// was the length of any possible match
    TotalDifference,
}

impl TryFrom<&str> for CodeMatrix {
    type Error = CodeMatrixError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
        if least_distance == &2 {
            return Err(CodeMatrixError::TotalDifference);
        }
        match index {
            0 => Ok(Self::_1C),
            1 => Ok(Self::_55),
            2 => Ok(Self::_7A),
            3 => Ok(Self::_BD),
            4 => Ok(Self::_E9),
            5 => Ok(Self::_FF),
            _ => unreachable!(),
        }
    }
}

/// Actor that handles Code Matrix functions
pub(crate) struct CodeMatrixActor;

impl Actor for CodeMatrixActor {
    type Context = Context<Self>;
}
