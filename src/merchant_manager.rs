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

    fn append_merchant(&mut self, merchant: Arc<dyn Merchant>) -> Self::Identity;
    fn remove_merchant(&mut self, merchant: Arc<dyn Merchant>) -> Self::Identity;
    fn get(&self, identity: Self::Identity) -> Arc<dyn Merchant>;
}
