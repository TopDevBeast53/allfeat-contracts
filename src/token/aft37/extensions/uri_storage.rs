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
    aft37,
    aft37::extensions::uri_storage,
    traits::aft37::{extensions::uri_storage::*, *},
};
pub use aft37::Internal as _;
use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, Storage};
use openbrush::{storage::Mapping, traits::Balance};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub base_uri: Option<URI>,
    pub token_uris: Mapping<Id, URI>,
}

pub trait AFT37URIStorageImpl: aft37::Internal + Storage<Data> {
    fn base_uri(&self) -> Option<URI> {
        self.data().base_uri.clone()
    }

    fn token_uri(&self, token_id: Id) -> Result<Option<URI>, AFT37Error> {
        match self.data().token_uris.get(&token_id) {
            None => Err(AFT37Error::TokenNotExists)?,
            Some(uri) => {
                let base_uri = self.data().base_uri.clone();

                match base_uri {
                    // If both are set, concatenate the baseURI and tokenURI.
                    Some(base) => Ok(Some(base + &uri)),
                    // If there is no base URI, return the token URI.
                    None => Ok(Some(uri)),
                }
            }
        }
    }
}

pub trait Internal: aft37::Internal {
    /// Event is emitted when an URI is set for a token.
    fn _emit_attribute_set_event(&self, token_id: Id, token_uri: URI);
    /// Event is emitted when the base URI is updated.
    fn _emit_attribute_set_base_event(&self, base_uri: Option<URI>);

    /// Sets `token_uri` as the tokenURI of `token_id`.
    ///
    /// `token_id` must exist.
    fn _set_token_uri(&mut self, token_id: Id, token_uri: URI) -> Result<(), AFT37Error>;

    /// Sets `base_uri`.
    fn _set_base_uri(&mut self, base_uri: Option<URI>);

    /// This override additionally checks to see if
    /// token-specific URI was set for the token, and if so, it deletes the token URI from
    ///  the storage mapping.
    fn _burn_from(
        &mut self,
        from: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
    ) -> Result<(), AFT37Error>;
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_attribute_set_event(&self, _token_id: Id, _token_uri: URI) {}
    fn _emit_attribute_set_base_event(&self, _base_uri: Option<URI>) {}

    fn _set_token_uri(&mut self, token_id: Id, token_uri: URI) -> Result<(), AFT37Error> {
        self.data().token_uris.insert(&token_id, &token_uri);
        Internal::_emit_attribute_set_event(self, token_id, token_uri);
        Ok(())
    }

    fn _set_base_uri(&mut self, base_uri: Option<URI>) {
        self.data().base_uri = base_uri.clone();
        Internal::_emit_attribute_set_base_event(self, base_uri)
    }

    fn _burn_from(
        &mut self,
        from: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
    ) -> Result<(), AFT37Error> {
        aft37::Internal::_burn_from(self, from, ids_amounts)
    }
}
