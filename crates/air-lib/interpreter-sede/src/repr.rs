/*
 * Copyright 2023 Fluence Labs Limited
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

// TODO
// TODO All the traits can be replaced with a trait that has a Format and some default implementations...
// TODO


pub trait ToRepresentation<Value> {
    type Error;

    fn to_representation(&self, value: &Value) -> Result<Vec<u8>, Self::Error>;
    // TODO to value
}

pub trait FromRepresentation<Value> {
    type Error;

    fn from_representation(&self, repr: &[u8]) -> Result<Value, Self::Error>;
    // TODO from value
}

pub trait ToWrite<Value> {
    type Error;

    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        writer: &mut W,
    ) -> Result<(), Self::Error>;
}
