//! Storage migrations for the peaq-pallet-mor.

use frame_support::{pallet_prelude::*, weights::Weight};

use crate::{
    pallet::*,
    types::{BalanceOf, MorConfig},
};

pub(crate) fn on_runtime_upgrade<T: Config>() -> Weight {
    v2::MigrateToV2x::<T>::on_runtime_upgrade()
}

mod v2 {
    use super::*;

    /// Migration implementation that renames storage HardCap into MaxCurrencySupply
    pub struct MigrateToV2x<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> MigrateToV2x<T> {
        pub fn on_runtime_upgrade() -> Weight {
            let mut weight_reads = 1;
            let mut weight_writes = 0;

            let this_version = Pallet::<T>::current_storage_version();
            let on_chain_version = Pallet::<T>::on_chain_storage_version();

            if on_chain_version < this_version {
                log::info!(
                    "Migrating storage from version {:?} to version {:?}",
                    on_chain_version,
                    this_version
                );
                // Translate the reward_rec
                let reward_rec = RewardsRecordStorage::<T>::get();
                let mor_config = MorConfigStorage::<T>::get();
                // This part, we only need to check whether the data is set before
                // Therefore, once it not setup, we should reset it
                if mor_config.track_n_block_rewards as usize != reward_rec.1.len() {
                    log::info!("Resetting storage");
                    let mor_config = MorConfig::<BalanceOf<T>>::default();
                    Pallet::<T>::init_storages(&mor_config);
                    weight_writes += 3;
                    weight_reads += 2;
                    T::DbWeight::get().reads_writes(2, 3);
                } else {
                    weight_reads += 2;
                    T::DbWeight::get().reads_writes(2, 0);
                }
                STORAGE_VERSION.put::<Pallet<T>>();
            }
            T::DbWeight::get().reads_writes(weight_reads, weight_writes)
        }
    }
}
