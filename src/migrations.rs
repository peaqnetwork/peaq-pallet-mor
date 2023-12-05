//! Storage migrations for the peaq-pallet-mor.

use frame_support::{pallet_prelude::*, storage_alias, weights::Weight};

use crate::{
    pallet::*,
    types::{BalanceOf, DiscAvg},
};

pub(crate) fn on_runtime_upgrade<T: Config>() -> Weight {
    v2::MigrateToV2x::<T>::on_runtime_upgrade()
}

mod v2 {
    use super::*;

    // Old storage aliases, to be removed in this migration.
    #[storage_alias]
    type RewardsRecordStorage<T: Config> =
        StorageValue<Pallet<T>, (u8, Vec<BalanceOf<T>>), ValueQuery>;
    #[storage_alias]
    type PeriodRewardStorage<T: Config> = StorageValue<Pallet<T>, BalanceOf<T>, ValueQuery>;

    /// Migration implementation that renames storage HardCap into MaxCurrencySupply
    pub struct MigrateToV2x<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> MigrateToV2x<T> {
        pub fn on_runtime_upgrade() -> Weight {
            let mut weight_reads = 1;
            let mut weight_writes = 0;

            let this_version = Pallet::<T>::current_storage_version();
            let on_chain_version = Pallet::<T>::on_chain_storage_version();

            if this_version < on_chain_version {
                // Retrieve old "sum"-value as reference for new distributions
                let old_sum = PeriodRewardStorage::<T>::get();

                // Fill new storage with DiscAvg initial values
                let disc_avg = DiscAvg::<T>::new(old_sum, 7200);
                AverageReferenceBalance::<T>::put(disc_avg);

                // Kill old storages, which are no more needed anymore
                RewardsRecordStorage::<T>::kill();
                PeriodRewardStorage::<T>::kill();

                weight_reads += 1;
                weight_writes += 3;
            }

            T::DbWeight::get().reads_writes(weight_reads, weight_writes)
        }
    }
}
