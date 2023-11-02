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

pub trait ToSerialized<Value> {
    type Error;

    fn serialize(&self, value: &Value) -> Result<Vec<u8>, Self::Error>;
}

pub trait FromSerialized<Value> {
    type Error;

    fn deserialize(&self, repr: &[u8]) -> Result<Value, Self::Error>;
}

pub trait FromSerialiedBorrow<'data, Value: 'data> {
    type Error;

    fn deserialize_borrow(&self, repr: &'data [u8]) -> Result<Value, Self::Error>;
}

pub trait ToWriter<Value> {
    type Error;

    fn to_writer<W: std::io::Write>(
        &self,
        value: &Value,
        writer: &mut W,
    ) -> Result<(), Self::Error>;
}
