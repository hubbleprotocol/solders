use pyo3::{prelude::*, pyclass::CompareOp, types::PyBytes};
use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    signature::Signature as SignatureOriginal,
    signer::{
        keypair::{
            keypair_from_seed, keypair_from_seed_phrase_and_passphrase, Keypair as KeypairOriginal,
        },
        Signer as SignerTrait, SignerError,
    },
};

use crate::{
    handle_py_value_err, pubkey::Pubkey, signature::Signature, RichcmpEqOnlyPrecalculated, Signer,
};

#[pyclass(module = "solders", subclass)]
#[derive(PartialEq, Debug)]
/// A vanilla Ed25519 key pair.
///
/// Calling ``Keypair()`` creates a new, random ``Keypair``.
///
/// Example::
///     from solders.keypair import Keypair
///     
///     assert Keypair() != Keypair()
///
pub struct Keypair(pub KeypairOriginal);

#[pymethods]
impl Keypair {
    #[classattr]
    /// The length of a keypair in bytes.
    const LENGTH: usize = 64;
    #[new]
    /// Constructs a new, random ``Keypair`` using ``OsRng``
    pub fn new() -> Self {
        KeypairOriginal::new().into()
    }

    /// Recovers a ``Keypair`` from bytes.
    ///
    /// Args:
    ///     raw_bytes (bytes): a 64-byte keypair.
    ///
    /// Returns:
    ///     Keypair: a keypair object.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     kp = Keypair()
    ///     assert kp == Keypair.from_bytes(bytes(kp))
    ///
    #[staticmethod]
    pub fn from_bytes(raw_bytes: [u8; Self::LENGTH]) -> PyResult<Self> {
        handle_py_value_err(KeypairOriginal::from_bytes(&raw_bytes))
    }

    /// Returns this ``Keypair`` as a byte array.
    ///
    /// Returns:
    ///     list[int]: the keypair as a list of 64 u8 ints.
    ///
    /// Example::
    ///      from solders.keypair import Keypair
    ///      raw_bytes = bytes([1] * 64)
    ///      assert Keypair.from_bytes(raw_bytes).to_bytes_array() == list(raw_bytes)
    ///
    pub fn to_bytes_array(&self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }

    pub fn __bytes__<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.to_bytes_array().as_slice())
    }

    #[staticmethod]
    /// Recovers a ``Keypair`` from a base58-encoded string.
    ///
    /// Args:
    ///     s (str): The base58-encoded string.
    ///
    /// Returns:
    ///     Keypair: a keypair oject.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     
    ///     raw_bytes = bytes([0] * 64)
    ///     base58_str = "1" * 64
    ///     kp = Keypair.from_base58_string(base58_str)
    ///     assert kp == Keypair.from_bytes(raw_bytes)
    ///     assert str(kp) == base58_str
    ///     
    pub fn from_base58_string(s: &str) -> Self {
        KeypairOriginal::from_base58_string(s).into()
    }
    /// Gets this ``Keypair``'s secret key.
    ///
    /// Returns:
    ///     bytes: The secret key in 32 bytes.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     
    ///     kp = Keypair.from_bytes(bytes([1] * 64))
    ///     assert kp.secret() == bytes([1] * 32)
    ///
    pub fn secret(&self) -> &[u8] {
        self.0.secret().as_ref()
    }

    pub fn __str__(&self) -> String {
        self.0.to_base58_string()
    }

    #[pyo3(name = "pubkey")]
    /// Get this keypair's :class:`Pubkey`.
    ///
    /// Returns:
    ///     Pubkey: the pubkey of this keypair.
    ///
    /// Example::
    ///     from solders.keypair import Keypair
    ///     from solders.pubkey import Pubkey
    ///     
    ///     seed_bytes = bytes([0] * 32)
    ///     pubkey_bytes = bytes([1] * 32)
    ///     kp = Keypair.from_bytes(seed_bytes + pubkey_bytes)
    ///     assert kp.pubkey() == Pubkey(pubkey_bytes)
    ///
    pub fn py_pubkey(&self) -> Pubkey {
        self.pubkey().into()
    }

    #[pyo3(name = "sign_message")]
    /// Sign a mesage with this keypair, producing an Ed25519 signature over the provided message bytes.
    ///
    /// Args:
    ///     message (bytes): The message to sign.
    ///
    /// Returns:
    ///     Signature: The Ed25519 signature.
    ///
    /// Example:
    ///     >>> from solders.keypair import Keypair
    ///     >>> seed = bytes([1] * 32)
    ///     >>> keypair = Keypair.from_seed(seed)
    ///     >>> msg = b"hello"
    ///     >>> sig = keypair.sign_message(msg)
    ///     >>> bytes(sig).hex()
    ///     'e1430c6ebd0d53573b5c803452174f8991ef5955e0906a09e8fdc7310459e9c82a402526748c3431fe7f0e5faafbf7e703234789734063ee42be17af16438d08'
    ///
    pub fn py_sign_message(&self, message: &[u8]) -> Signature {
        self.sign_message(message).into()
    }

    #[staticmethod]
    /// Generate a keypair from a 32 byte seed.
    ///
    /// Args:
    ///     seed: 32-byte seed.
    /// Returns:
    ///     Keypair: The generated keypair.
    ///
    /// Example::
    ///
    ///     from solders.keypair import Keypair
    ///     from solders.pubkey import Pubkey
    ///     
    ///     seed_bytes = bytes([0] * 32)
    ///     from_seed = Keypair.from_seed(seed_bytes)
    ///     from_bytes = Keypair.from_bytes(seed_bytes + bytes(from_seed.pubkey()))
    ///     assert from_seed == from_bytes
    ///
    pub fn from_seed(seed: [u8; 32]) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed(&seed))
    }

    #[staticmethod]
    /// Generate a keypair from a seed phrase and passphrase.
    ///
    /// Args:
    ///     seed_phrase (string): Secret seed phrase.
    ///     passphrase (string): Passphrase.
    ///
    /// Example::
    ///     from pybip39 import Mnemonic, Seed
    ///     from solders.keypair import Keypair
    ///    
    ///     mnemonic = Mnemonic()
    ///     passphrase = "42"
    ///     seed = Seed(mnemonic, passphrase)
    ///     expected_keypair = Keypair.from_seed(bytes(seed)[:32])
    ///     keypair = Keypair.from_seed_phrase_and_passphrase(mnemonic.phrase, passphrase)
    ///     assert keypair.pubkey() == expected_keypair.pubkey()
    ///
    pub fn from_seed_phrase_and_passphrase(seed_phrase: &str, passphrase: &str) -> PyResult<Self> {
        handle_py_value_err(keypair_from_seed_phrase_and_passphrase(
            seed_phrase,
            passphrase,
        ))
    }

    pub fn __hash__(&self) -> PyResult<isize> {
        // call `hash((class_name, bytes(obj)))`
        Python::with_gil(|py| {
            let builtins = PyModule::import(py, "builtins")?;
            let arg1 = "Keypair";
            let arg2 = self.__bytes__(py);
            builtins.getattr("hash")?.call1(((arg1, arg2),))?.extract()
        })
    }

    fn __richcmp__(&self, other: Signer, op: CompareOp) -> PyResult<bool> {
        let other_eq = match other {
            Signer::KeypairWrapper(kp) => kp.0 == self.0,
            Signer::PresignerWrapper(ps) => ps.0 == self.0,
        };
        self.richcmp(other_eq, op)
    }

    #[pyo3(name = "is_interactive")]
    /// Whether the impelmentation requires user interaction to sign.
    ///
    /// Returns:
    ///     bool: True if user interaction is required.
    ///
    pub fn py_is_interactive(&self) -> bool {
        self.is_interactive()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}

impl RichcmpEqOnlyPrecalculated for Keypair {}

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
    }
}

impl From<KeypairOriginal> for Keypair {
    fn from(keypair: KeypairOriginal) -> Self {
        Self(keypair)
    }
}

impl From<Keypair> for KeypairOriginal {
    fn from(k: Keypair) -> Self {
        k.0
    }
}

impl AsRef<KeypairOriginal> for Keypair {
    fn as_ref(&self) -> &KeypairOriginal {
        &self.0
    }
}

impl Clone for Keypair {
    fn clone(&self) -> Self {
        Self::from_bytes(self.to_bytes_array()).unwrap()
    }
}

impl SignerTrait for Keypair {
    fn pubkey(&self) -> PubkeyOriginal {
        self.0.pubkey()
    }
    fn try_pubkey(&self) -> Result<PubkeyOriginal, SignerError> {
        self.0.try_pubkey()
    }
    fn sign_message(&self, message: &[u8]) -> SignatureOriginal {
        self.0.sign_message(message)
    }
    fn try_sign_message(&self, message: &[u8]) -> Result<SignatureOriginal, SignerError> {
        self.0.try_sign_message(message)
    }
    fn is_interactive(&self) -> bool {
        self.0.is_interactive()
    }
}
