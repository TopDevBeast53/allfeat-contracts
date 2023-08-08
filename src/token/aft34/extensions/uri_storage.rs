// Copyright (c) 2022-2023 Allfeat labs
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    aft34,
    traits::aft34::{extensions::uri_storage::*, *},
};
pub use aft34::{
    AFT34Impl, BalancesManager as _, Internal as _, InternalImpl as _, Operator, Owner,
};
use openbrush::storage::Mapping;
use openbrush::traits::Storage;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub base_uri: Option<URI>,
    pub token_uris: Mapping<Id, URI>,
}

pub trait AFT34URIImpl: Storage<Data> {
    fn token_uri(&self, token_id: Id) -> Result<URI, AFT34Error> {
        let token_uri = self
            .data()
            .token_uris
            .get(&token_id)
            .ok_or(AFT34Error::TokenNotExists)?;
        let base_uri = self.data().base_uri.clone();

        match base_uri {
            // If both are set, concatenate the baseURI and tokenURI..
            Some(base) => Ok(base + &token_uri),
            // If there is no base URI, return the token URI.
            None => Ok(token_uri),
        }
    }
}

pub trait Internal: aft34::Internal {
    /// Event is emitted when an URI is set for a token.
    fn _emit_attribute_set_event(&self, token_id: Id, token_uri: URI);

    /// Sets `token_uri` as the tokenURI of `token_id`.
    ///
    /// `token_id` must exist.
    fn _set_token_uri(&mut self, token_id: Id, token_uri: URI) -> Result<(), AFT34Error>;
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_attribute_set_event(&self, _token_id: Id, _token_uri: URI) {}

    fn _set_token_uri(&mut self, token_id: Id, token_uri: URI) -> Result<(), AFT34Error> {
        aft34::Internal::_owner_of(self, &token_id).ok_or(AFT34Error::TokenNotExists)?;
        self.data().token_uris.insert(&token_id, &token_uri);
        Ok(())
    }
}
