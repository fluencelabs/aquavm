"""Helper functions for performance_metering."""


def get_host_id() -> str:
    """Return a hash of host id."""
    import socket
    import hashlib

    hostname = socket.gethostname().encode('utf-8')
    return hashlib.sha256(hostname).hexdigest()
