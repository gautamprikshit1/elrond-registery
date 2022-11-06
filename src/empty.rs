#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::contract]
pub trait RegistryContract {
    #[init]
    fn init(&self) {}

    #[endpoint(addEntry)]
    fn add_entry(
        &self,
        title: ManagedBuffer,
        description: ManagedBuffer,
        public_url: ManagedBuffer,
    ) {
        let new_entry = Entry {
            sender: self.blockchain().get_caller(),
            title,
            description,
            public_url,
            id: self.get_all_enteries().len() + 1usize,
            votes: 0,
        };
        self.get_all_enteries().push(&new_entry);
    }

    #[endpoint(upvoteEntry)]
    fn upvote_entry(&self, index: usize) {
        let mut entry = self.get_all_enteries().get(index);
        entry.votes += 1;
        self.get_all_enteries().set(index, &entry);
    }

    #[view(enteries)]
    #[storage_mapper("Enteries")]
    fn get_all_enteries(&self) -> VecMapper<Entry<Self::Api>>;
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Entry<M: ManagedTypeApi> {
    pub sender: ManagedAddress<M>,
    pub title: ManagedBuffer<M>,
    pub description: ManagedBuffer<M>,
    pub public_url: ManagedBuffer<M>,
    pub id: usize,
    pub votes: u64,
}
