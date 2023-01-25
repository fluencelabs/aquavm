#
#  Copyright 2023 Fluence Labs Limited
#
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
#
"""Helper functions for performance_metering."""


def get_host_id() -> str:
    """Return a hash of host id."""
    import socket
    import hashlib

    hostname = socket.gethostname().encode('utf-8')
    return hashlib.sha256(hostname).hexdigest()


def get_aquavm_version(path: str) -> str:
    """Get `version` field from a TOML file."""
    import toml
    with open(path, 'r') as inp:
        data = toml.load(inp)
    return data['package']['version']
