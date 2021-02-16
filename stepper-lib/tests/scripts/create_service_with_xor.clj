(seq
 (seq
  (seq
   (call "set_variables" ("add_module" "") ["module_bytes"] module_bytes)
   (call "set_variables" ("add_module" "") ["module_config"] module_config)
   )
  (call "set_variables" ("add_module" "") ["blueprint"] blueprint)
  )
 (xor
  (seq
   (call relay ("dist" "add_module") [module_bytes module_config] module)
   (seq
    (call relay ("dist" "add_blueprint") [blueprint] blueprint_id)
    (seq
     (call relay ("srv" "create") [blueprint_id] service_id)
     (call client ("return" "") [service_id] client_result)
     )
    )
   )
  (seq
   (call relay ("op" "identity") ["XOR: create_greeting_service failed"] fail[])
   (call client ("return" "") [fail %last_error%])
   )
  )
 )