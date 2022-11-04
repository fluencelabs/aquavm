(seq
 (seq
  (seq
   (call "set_variables" ("add_module" "") ["module_bytes"] module_bytes)
   (call "set_variables" ("add_module" "") ["module_config"] module_config)
   )
  (call "set_variables" ("add_module" "") ["blueprint"] blueprint)
  )
 (seq
  (call "A" ("add_module" "") [module_bytes module_config] module)
  (seq
   (call "A" ("add_blueprint" "") [blueprint] blueprint_id)
   (seq
    (call "A" ("create" "") [blueprint_id] service_id)
    (call "remote_peer_id" ("" "") [service_id] client_result)
    )
   )
  )
 )
