use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::Deserialize;
use std::borrow::Cow;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
pub struct GameInfo {
    pub amount: u64,
    pub timestamp_nanos: u64,
    pub result: String,
    pub random_hex: String,
    pub hash: String,
}

impl Storable for GameInfo {
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 200,
        is_fixed_size: false,
    };
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static MAP: RefCell<StableBTreeMap<Principal, GameInfo, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

pub fn get(key: Principal) -> Option<GameInfo> {
    MAP.with(|p| p.borrow().get(&key))
}

pub fn insert(key: Principal, value: GameInfo) {
    MAP.with(|p| p.borrow_mut().insert(key, value));
}

pub fn delete(key: Principal) -> Option<GameInfo> {
    MAP.with(|p| p.borrow_mut().remove(&key))
}
