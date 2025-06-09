use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Opaque proof struct matching the C header
#[repr(C)]
pub struct StarkProof {
    _dummy: u8,
}

// Generate a deposit proof (stub implementation)
#[no_mangle]
pub extern "C" fn generate_deposit_proof(
    amount: u64,
    term: u32,
    tx_hash: *const u8,
    hash_len: usize,
) -> *mut StarkProof {
    // Stub: ignore inputs and return an empty proof
    let proof = Box::new(StarkProof { _dummy: 0 });
    Box::into_raw(proof)
}

// Verify a deposit proof (stub implementation)
#[no_mangle]
pub extern "C" fn verify_deposit_proof(proof: *const StarkProof) -> bool {
    // Stub: return false if proof pointer is null
    !proof.is_null()
}

#[no_mangle]
pub extern "C" fn winterfell_ffi_hello(name: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = c_str.to_str().unwrap_or("world");
    let result = format!("Hello, {}!", name_str);
    CString::new(result).unwrap().into_raw()
}
