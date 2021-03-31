use std::sync::Arc;
use crate::merchant::Merchant;

pub trait IdentityGiver {
    type Identity;
    type Token;

    fn give(token: Self::Token) ->  Self::Identity;
}

pub trait MerchantManager {
    type Identity;
    type Giver: IdentityGiver<Identity = Self::Identity>; 

    fn insert(&mut self, merchant: Arc<dyn Merchant>) -> Option<Self::Identity>;
    fn remove(&mut self, identity: Self::Identity) -> Option<Arc<dyn Merchant>>;
    fn get(&self, identity: Self::Identity) -> Option<Arc<dyn Merchant>>;
}
