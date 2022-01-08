from typing import List

def is_on_curve(_bytes: bytes) -> bool: ...

class PublicKey:
    LENGTH: int
    def __init__(self, pubkey_bytes: bytes) -> None: ...
    @staticmethod
    def new_unique() -> "PublicKey": ...
    @staticmethod
    def default() -> "PublicKey": ...
    @staticmethod
    def from_str(s: str) -> "PublicKey": ...
    @staticmethod
    def create_with_seed(
        from_public_key: "PublicKey", seed: str, program_id: "PublicKey"
    ) -> "PublicKey": ...
    @staticmethod
    def create_program_address(seeds: List[bytes]) -> "PublicKey": ...
    @staticmethod
    def find_program_address(
        seeds: List[bytes], program_id: "PublicKey"
    ) -> "PublicKey": ...
    def is_on_curve(self) -> bool: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __richcmp__(self, other: "PublicKey", op: int) -> bool: ...

class Keypair:
    def __init__(self) -> None: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "Keypair": ...
