// Copyright 2017 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rand::{Rng, XorShiftRng, SeedableRng};

use exonum::messages::{Message, RawTransaction};
use exonum::encoding::Error as MessageError;
use exonum::crypto::{PublicKey, SecretKey, Hash, gen_keypair};
use exonum::storage::{Fork, Snapshot};
use exonum::blockchain::{Service, Transaction, TransactionSet, ExecutionResult};

pub const TIMESTAMPING_SERVICE: u16 = 129;
pub const TIMESTAMPING_TRANSACTION_MESSAGE_ID: u16 = 128;

transactions! {
    TimestampingTransactions {
        const SERVICE_ID = TIMESTAMPING_SERVICE;

        struct TimestampTx {
            pub_key: &PublicKey,
            data: &[u8],
        }
    }
}

impl Transaction for TimestampTx {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, _: &mut Fork) {}
}


#[derive(Default)]
pub struct TimestampingService {}

pub struct TimestampingTxGenerator {
    rand: XorShiftRng,
    data_size: usize,
    public_key: PublicKey,
    secret_key: SecretKey,
}

impl TimestampingTxGenerator {
    pub fn new(data_size: usize) -> TimestampingTxGenerator {
        let keypair = gen_keypair();
        TimestampingTxGenerator::with_keypair(data_size, keypair)
    }

    pub fn with_keypair(
        data_size: usize,
        keypair: (PublicKey, SecretKey),
    ) -> TimestampingTxGenerator {
        let rand = XorShiftRng::from_seed([192, 168, 56, 1]);

        TimestampingTxGenerator {
            rand: rand,
            data_size: data_size,
            public_key: keypair.0,
            secret_key: keypair.1,
        }
    }
}

impl Iterator for TimestampingTxGenerator {
    type Item = TimestampTx;

    fn next(&mut self) -> Option<TimestampTx> {
        let mut data = vec![0; self.data_size];
        self.rand.fill_bytes(&mut data);
        Some(TimestampTx::new(&self.public_key, &data, &self.secret_key))
    }
}

impl TimestampingService {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Service for TimestampingService {
    fn service_name(&self) -> &'static str {
        "sandbox_timestamping"
    }

    fn service_id(&self) -> u16 {
        TIMESTAMPING_SERVICE
    }

    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![Hash::new([127; 32]), Hash::new([128; 32])]
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, MessageError> {
        let tx = TimestampingTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }
}
