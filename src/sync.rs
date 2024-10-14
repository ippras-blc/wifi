use crate::r#async;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    sys::EspError,
    timer::EspTaskTimerService,
    wifi::{AsyncWifi, EspWifi},
};
use futures_lite::future::block_on;
use log::error;
use std::time::Duration;

const RETRY: Duration = Duration::from_secs(1);

pub macro initialize($modem:expr, $event_loop:expr, $timer_service:expr, $nvs:expr) {
    loop {
        match try_initialize($modem, $event_loop, $timer_service, $nvs) {
            Ok(wifi) => break wifi,
            Err(error) => {
                std::thread::sleep(RETRY);
                error!("{error}");
            }
        }
    }
}

// pub(crate) fn initialize<'a>(
//     modem: &'a mut Modem,
//     event_loop: &EspSystemEventLoop,
//     timer_service: &EspTaskTimerService,
//     nvs: &EspDefaultNvsPartition,
// ) -> AsyncWifi<EspWifi<'a>> {
//     loop {
//         match initialize(&mut *modem, event_loop, timer_service, nvs) {
//             Ok(wifi) => break wifi,
//             Err(error) => {
//                 thread::sleep(DEFAULT_TIMEOUT);
//                 error!("{error}");
//             }
//         }
//     }
// }

pub fn try_initialize<'a>(
    modem: impl Peripheral<P = impl WifiModemPeripheral> + 'a,
    event_loop: &EspSystemEventLoop,
    timer_service: &EspTaskTimerService,
    nvs: &EspDefaultNvsPartition,
) -> Result<AsyncWifi<EspWifi<'a>>, EspError> {
    block_on(r#async::try_initialize(
        modem,
        event_loop,
        timer_service,
        nvs,
    ))
}
