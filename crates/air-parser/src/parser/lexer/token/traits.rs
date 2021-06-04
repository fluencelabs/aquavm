/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;

use std::convert::TryFrom;
use std::fmt;

impl fmt::Display for LastErrorPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LastErrorPath::*;

        match self {
            Instruction => write!(f, ".instruction"),
            Message => write!(f, ".msg"),
            None => write!(f, ""),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Number::*;

        match self {
            Int(number) => write!(f, "{}", number),
            Float(number) => write!(f, "{}", number),
        }
    }
}

impl fmt::Display for Variable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Variable::*;

        match self {
            Scalar(name) => write!(f, "{}", name),
            Stream(name) => write!(f, "&{}", name),
        }
    }
}

impl From<Number> for Token<'_> {
    fn from(value: Number) -> Self {
        Token::Number(value)
    }
}

impl From<Number> for serde_json::Value {
    fn from(number: Number) -> Self {
        (&number).into()
    }
}

impl From<&Number> for serde_json::Value {
    fn from(number: &Number) -> Self {
        match number {
            Number::Int(value) => (*value).into(),
            Number::Float(value) => (*value).into(),
        }
    }
}

impl TryFrom<UnparsedNumber<'_>> for Number {
    type Error = LexerError;

    fn try_from(value: UnparsedNumber<'_>) -> LexerResult<Number> {
        match value {
            UnparsedNumber::Int(raw_value, start_pos) => {
                let number = raw_value.parse::<i64>().map_err(|e| {
                    LexerError::ParseIntError(start_pos, start_pos + raw_value.len(), e)
                })?;

                let number = Self::Int(number);
                Ok(number)
            }

            UnparsedNumber::Float(raw_value, start_pos) => {
                if raw_value.len() > 11 {
                    return Err(LexerError::TooBigFloat(
                        start_pos,
                        start_pos + raw_value.len(),
                    ));
                }

                let number = raw_value.parse::<f64>().map_err(|e| {
                    LexerError::ParseFloatError(start_pos, start_pos + raw_value.len(), e)
                })?;

                let number = Self::Float(number);
                Ok(number)
            }
        }
    }
}
