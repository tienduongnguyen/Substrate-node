#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage

	#[pallet::storage]
	pub type Number<T: Config> = StorageMap<_,Blake2_128Concat, 
											T::AccountId, 
											u32, 
											ValueQuery, >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		NumberStored(u32, T::AccountId),
		NumberChanged(u32, T::AccountId),
		NumberFree(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Store the given number to the given account.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_number(origin: OriginFor<T>, number: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Update storage.
			<Number<T>>::insert(who.clone(), number);

			// Emit an event.
			Self::deposit_event(Event::NumberStored(number, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Remove the given account's number.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_number(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Update storage.
			<Number<T>>::remove(who.clone());

			// Emit an event.
			Self::deposit_event(Event::NumberFree(who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Increase the number of the given account.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn increase_number(origin: OriginFor<T>, amount: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Get storage.
			let n = <Number<T>>::get(who.clone());

			// Update storage.
			<Number<T>>::insert(who.clone(), n + amount);

			// Emit an event.
			Self::deposit_event(Event::NumberChanged(amount, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Remove the given account's number.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn decrease_number(origin: OriginFor<T>, amount: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Get storage.
			let n = <Number<T>>::get(who.clone());

			// Update storage.
			<Number<T>>::insert(who.clone(), n - amount);

			// Emit an event.
			Self::deposit_event(Event::NumberChanged(amount, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
