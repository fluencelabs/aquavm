/*
 * Copyright 2024 Fluence DAO
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

use crate::Beautifier;

#[test]
fn if_else_nested() {
    let script = r#"
         (new my-var (new -if-else-error-
          (new -else-error-
           (new -if-error-
            (xor
             (match "a" "test"
              (ap 0 $result)
             )
             (seq
              (ap :error: -if-error-)
              (xor
               (match :error:.$.error_code 10001
                (ap 1 $result)
               )
               (seq
                (seq
                 (ap :error: -else-error-)
                 (xor
                  (match :error:.$.error_code 10001
                   (ap -if-error- -if-else-error-)
                  )
                  (ap -else-error- -if-else-error-)
                 )
                )
                (fail -if-else-error-)
               )
              )
             )
            )
           )
          )
         ) )
    "#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new my-var:\n",
            r#"    if "a" == "test":"#,
            "\n",
            "        ap 0 $result\n",
            "    else:\n",
            "        ap 1 $result\n",
        ),
    );
}

#[test]
fn if_else_match_on() {
    let script = r#"
         (new -if-else-error-
          (new -else-error-
           (new -if-error-
            (xor
             (match "a" "test"
              (ap 0 $result)
             )
             (seq
              (ap :error: -if-error-)
              (xor
               (match :error:.$.error_code 10001
                (ap 1 $result)
               )
               (seq
                (seq
                 (ap :error: -else-error-)
                 (xor
                  (match :error:.$.error_code 10001
                   (ap -if-error- -if-else-error-)
                  )
                  (ap -else-error- -if-else-error-)
                 )
                )
                (fail -if-else-error-)
               )
              )
             )
            )
           )
          )
         )
    "#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            r#"if "a" == "test":"#,
            "\n",
            "    ap 0 $result\n",
            "else:\n",
            "    ap 1 $result\n",
        ),
    );
}

#[test]
fn if_else_match_off() {
    let script = r#"
         (new -if-else-error-
          (new -else-error-
           (new -if-error-
            (xor
             (match "a" "test"
              (ap 0 $result)
             )
             (seq
              (ap :error: -if-error-)
              (xor
               (match :error:.$.error_code 10001
                (ap 1 $result)
               )
               (seq
                (seq
                 (ap :error: -else-error-)
                 (xor
                  (match :error:.$.error_code 10001
                   (ap -if-error- -if-else-error-)
                  )
                  (ap -else-error- -if-else-error-)
                 )
                )
                (fail -if-else-error-)
               )
              )
             )
            )
           )
          )
         )
    "#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output);
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new -if-else-error-:\n",
            "    new -else-error-:\n",
            "        new -if-error-:\n",
            "            try:\n",
            r#"                match "a" "test":"#,
            "\n",
            "                    ap 0 $result\n",
            "            catch:\n",
            "                ap :error: -if-error-\n",
            "                try:\n",
            "                    match :error:.$.error_code 10001:\n",
            "                        ap 1 $result\n",
            "                catch:\n",
            "                    ap :error: -else-error-\n",
            "                    try:\n",
            "                        match :error:.$.error_code 10001:\n",
            "                            ap -if-error- -if-else-error-\n",
            "                    catch:\n",
            "                        ap -else-error- -if-else-error-\n",
            "                    fail -if-else-error-\n",
        ),
    );
}

#[test]
fn if_else_mismatch_on() {
    let script = r#"
         (new -if-else-error-
          (new -else-error-
           (new -if-error-
            (xor
             (mismatch "a" "test"
              (ap 0 $result)
             )
             (seq
              (ap :error: -if-error-)
              (xor
               (match :error:.$.error_code 10001
                (ap 1 $result)
               )
               (seq
                (seq
                 (ap :error: -else-error-)
                 (xor
                  (match :error:.$.error_code 10001
                   (ap -if-error- -if-else-error-)
                  )
                  (ap -else-error- -if-else-error-)
                 )
                )
                (fail -if-else-error-)
               )
              )
             )
            )
           )
          )
         )
    "#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output).enable_all_patterns();
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            r#"if "a" != "test":"#,
            "\n",
            "    ap 0 $result\n",
            "else:\n",
            "    ap 1 $result\n",
        ),
    );
}

#[test]
fn if_else_mismatch_off() {
    let script = r#"
         (new -if-else-error-
          (new -else-error-
           (new -if-error-
            (xor
             (mismatch "a" "test"
              (ap 0 $result)
             )
             (seq
              (ap :error: -if-error-)
              (xor
               (match :error:.$.error_code 10001
                (ap 1 $result)
               )
               (seq
                (seq
                 (ap :error: -else-error-)
                 (xor
                  (match :error:.$.error_code 10001
                   (ap -if-error- -if-else-error-)
                  )
                  (ap -else-error- -if-else-error-)
                 )
                )
                (fail -if-else-error-)
               )
              )
             )
            )
           )
          )
         )
    "#;

    let mut output = vec![];
    let mut beautifier = Beautifier::new(&mut output);
    beautifier.beautify(script).unwrap();

    assert_eq!(
        String::from_utf8(output).unwrap(),
        concat!(
            "new -if-else-error-:\n",
            "    new -else-error-:\n",
            "        new -if-error-:\n",
            "            try:\n",
            r#"                mismatch "a" "test":"#,
            "\n",
            "                    ap 0 $result\n",
            "            catch:\n",
            "                ap :error: -if-error-\n",
            "                try:\n",
            "                    match :error:.$.error_code 10001:\n",
            "                        ap 1 $result\n",
            "                catch:\n",
            "                    ap :error: -else-error-\n",
            "                    try:\n",
            "                        match :error:.$.error_code 10001:\n",
            "                            ap -if-error- -if-else-error-\n",
            "                    catch:\n",
            "                        ap -else-error- -if-else-error-\n",
            "                    fail -if-else-error-\n",
        ),
    );
}
