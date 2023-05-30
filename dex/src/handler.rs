use alloc::{string::ToString, sync::Arc, format};
use borsh::BorshDeserialize;
use marketplace::market::order_book::PlaceOrder;
use order_book::order::OrderSide;
use platform::zk::ZkSdk;
use spin::Mutex;
use spot::availability::SpotMarketAvailability;
use spot::currency::Currency;
use spot::spot_zk::Spot;
use types::number::decimal::Decimal;
use types::primitives::{U128, Timestamp};
use zk_sdk::interface::{Action, Handler, Logger, Metadata, Storage};
use zk_sdk::zk::ZkStorage;

pub struct DexHandler;

impl Handler for DexHandler {
    fn handle<B: BorshDeserialize, /*S: Storage,*/ A: Action<B>, L: Logger, M: Metadata>(
        storage: Arc<Mutex<ZkStorage>>,
        _action: &A,
        logger: &mut L,
        _metadata: &M,
    ) {
        ZkSdk::init(storage.clone());

        let owner = "OWNER".to_string();
        let user = "USER".to_string();
        let usdc = "usdc.near".to_string();
        
        let mut spot = Spot::<ZkSdk>::new(owner.clone());
        
        spot.add_near_currency(&owner);
        spot.add_ft_currency(&owner, usdc.clone(), 6, "USDCe".to_string());
        spot.create_market(&owner, Currency::Native, Currency::Ft(usdc.clone()));
        
        spot.set_market_options(owner.as_str(), 1, Some(SpotMarketAvailability {
            allow_place: true,
            allow_cancel: true,
            allow_trigger: true,
        }), None, None);

        spot.deposit(owner.clone(), Currency::Native, U128(1000000000000000000000000), true);
        spot.deposit(user.clone(), Currency::Ft(usdc.clone()), U128(5000000), true);

        spot.place_order(owner.clone(), 1, PlaceOrder {
            price: Decimal::int(3),
            quantity: Decimal::int(1),
            market_order: false,
            side: OrderSide::Ask,
            client_order_id: None,
            time_in_force: None,
            post_only: None,
            trigger_price: None,
        }, Timestamp(0), None);

        spot.place_order(user.clone(), 1, PlaceOrder {
            price: Decimal::int(3),
            quantity: Decimal::int(1),
            market_order: true,
            side: OrderSide::Bid,
            client_order_id: None,
            time_in_force: None,
            post_only: None,
            trigger_price: None,
        }, Timestamp(0), None);

        let result = format!("success: {:?} {:?}", spot.get_deposits(owner.clone()), spot.get_deposits(user.clone())).to_string();
        logger.log(&result);
    }
}
