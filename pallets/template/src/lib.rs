#![cfg_attr(not(feature = "std"), no_std)]

  pub use pallet::*;
  #[frame_support::pallet]
  pub mod pallet {
      use frame_support::pallet_prelude::*;
      use frame_system::pallet_prelude::*;
      // use scale_info::prelude::vec;
      // use scale_info::prelude::format;

      // The struct on which we build all of our Pallet logic.
      
      #[pallet::pallet]
      // #[pallet::generate_store(pub(super) trait Store)]
      pub struct Pallet<T>(_);

      /* Placeholder for defining custom types. */

      // pub enum Actions {
      //   Increment, 
      //   Decrement,
      //   Idle
      // }

      // TODO: Update the `config` block below
      #[pallet::config]
      pub trait Config: frame_system::Config {
          type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

          // #[pallet::constant]
		      type StringLimit: Get<u32>;
      }

      // TODO: Update the `event` block below
      #[pallet::event]
      #[pallet::generate_deposit(pub(super) fn deposit_event)]
      pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        ActionChanged(BoundedVec<u8, T::StringLimit> , T::AccountId),
        ActionExecuted(BoundedVec<u8, T::StringLimit>, T::AccountId),
        NumberStored(u32, T::AccountId),
      }

      // TODO: Update the `error` block below
      #[pallet::error]
      pub enum Error<T> {
        InvalidAction,
        StorageOverflow,
        NoneValue
      }
      // TODO: add #[pallet::storage] block
      #[pallet::storage]
      pub type NumberStorage<T> = StorageValue<_, u32>;

      // declaring an action storage
      #[pallet::storage]
      pub type ActionStorage<T: Config> = StorageValue<_, BoundedVec<u8, T::StringLimit>>;

      

      // TODO: Update the `call` block below
      #[pallet::call]
      impl<T: Config> Pallet<T> {

        #[pallet::weight(5_000)]
        pub fn store_number(origin: OriginFor<T>, number: u32) -> DispatchResult {
          // Check that the extrinsic was signed and get the signer.
          // This function will return an error if the extrinsic is not signed.
          // https://docs.substrate.io/v3/runtime/origins
          let who = ensure_signed(origin)?;

          // Update storage.
          <NumberStorage<T>>::put(number);

          // Emit an event.
          Self::deposit_event(Event::NumberStored(number, who));
          // Return a successful DispatchResultWithPostInfo
          Ok(())
        }

        #[pallet::weight(1_000)]
        pub fn change_action(origin: OriginFor<T>, action: BoundedVec<u8, T::StringLimit>) -> DispatchResult {
          // Check that the extrinsic was signed and get the signer.
          // This function will return an error if the extrinsic is not signed.
          let who = ensure_signed(origin)?;
          // let valid_actions = vec!["increment", "decrement", "idel"];

          ensure!(!action.is_empty(), Error::<T>::NoneValue);
          // let data = format!("{:?}", action.to_vec());
          // ensure!(valid_actions.contains(&data.as_str()), Error::<T>::InvalidAction);
          
          
          // Update storage.
          <ActionStorage<T>>::put(&action);
  
          // Emit an event.
          Self::deposit_event(Event::ActionChanged(action, who));
          // Return a successful DispatchResultWithPostInfo
          Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn execute_action(origin: OriginFor<T>) -> DispatchResult {
          // Check that the extrinsic was signed and get the signer.
          // This function will return an error if the extrinsic is not signed.
          let who = ensure_signed(origin)?;
          let action = <ActionStorage<T>>::get();
          // println!("ACTION {:?}", &action);
          match action {
            Some(data) => {
              let data1 = data.to_vec();
              if data1 == "increment".as_bytes() {
                if let Some(number) = <NumberStorage<T>>::get() {
                  let new = number.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                  <NumberStorage<T>>::put(new)
                }
              } else if data1 == "decrement".as_bytes() {
                if let Some(number) = <NumberStorage<T>>::get() {
                  if number > 0 {
                    let new = number.checked_sub(1).ok_or(Error::<T>::StorageOverflow)?;
                    <NumberStorage<T>>::put(new)
                  }
                }
              } if data1 == "idel".as_bytes() {
                
              } else {
                Err(Error::<T>::InvalidAction)?
              }
              // Emit an event.
              Self::deposit_event(Event::ActionExecuted(data, who));
            },
            None => {
              Err(Error::<T>::NoneValue)?
            }
          }
          // Return a successful DispatchResultWithPostInfo
          Ok(())
        }
      }
  }