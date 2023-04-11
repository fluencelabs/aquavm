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

mod dashboard;
mod network_explore;

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let dashboard_datas = dashboard::dashboard();
    save_datas("dashboard", &dashboard_datas.0, &dashboard_datas.1)?;

    let explore_datas = network_explore::network_explore();
    save_datas("explore", &explore_datas.0, &explore_datas.1)?;

    Ok(())
}

fn save_datas(prefix: &str, prev_data: &[u8], current_data: &[u8]) -> std::io::Result<()> {
    let mut prev = File::create(format!("{}_prev_data.json", prefix))?;
    prev.write_all(prev_data)?;

    let mut cur = File::create(format!("{}_cur_data.json", prefix))?;
    cur.write_all(current_data)?;
    Ok(())
}
