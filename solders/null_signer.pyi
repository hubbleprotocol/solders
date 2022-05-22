from typing import Union, Sequence
from solders.pubkey import Pubkey
from solders.signature import Signature
from solders.keypair import Keypair

class NullSigner:
    def __init__(self, pubkey: Pubkey, signature: Signature) -> None: ...
    def pubkey(self) -> Pubkey: ...
    def sign_message(self, message: Union[bytes, Sequence[int]]) -> Signature: ...
    def __richcmp__(self, other: Union["NullSigner", Keypair], op: int) -> bool: ...
    @staticmethod
    def default() -> "NullSigner": ...
    def __repr__(self) -> str: ...
