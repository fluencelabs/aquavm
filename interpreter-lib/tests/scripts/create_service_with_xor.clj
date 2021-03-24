(seq
 (seq
  (seq
   (call "{0}" ("add_module" "") ["module_bytes"] module_bytes)
   (call "{0}" ("add_module" "") ["module_config"] module_config)
   )
  (call "{0}" ("add_module" "") ["blueprint"] blueprint)
  )
 (xor
  (seq
   (call "{0}" ("dist" "add_module") [module_bytes module_config] module)
   (seq
    (call "{0}" ("dist" "add_blueprint") [blueprint] blueprint_id)
    (seq
     (call "{0}" ("srv" "create") [blueprint_id] service_id)
     (call "{1}" ("fallible_call_service" "") [service_id] client_result)
     )
    )
   )
  (seq
   (call "{1}" ("op" "identity") ["XOR: create_greeting_service failed"] $fail)
   (call "{2}" ("return" "") [%last_error%])
   )
  )
 )
