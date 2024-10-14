use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::WifiModemPeripheral, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    sys::EspError,
    timer::EspTaskTimerService,
    wifi::{AsyncWifi, ClientConfiguration, Configuration as WifiConfig, EspWifi},
};
use log::info;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

pub async fn try_initialize<'a>(
    modem: impl Peripheral<P = impl WifiModemPeripheral> + 'a,
    event_loop: &EspSystemEventLoop,
    timer_service: &EspTaskTimerService,
    nvs: &EspDefaultNvsPartition,
) -> Result<AsyncWifi<EspWifi<'a>>, EspError> {
    info!("initialize wifi");
    let wifi = EspWifi::new(modem, event_loop.clone(), Some(nvs.clone()))?;
    info!("sync wifi created");
    let mut wifi = AsyncWifi::wrap(wifi, event_loop.clone(), timer_service.clone())?;
    info!("async wifi created");
    wifi.set_configuration(&WifiConfig::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap(),
        password: WIFI_PASSWORD.try_into().unwrap(),
        ..Default::default()
    }))?;
    info!("wifi configured");
    wifi.start().await?;
    info!("wifi started");
    wifi.connect().await?;
    info!("wifi connected");
    wifi.wait_netif_up().await?;
    info!("wifi netif up");
    Ok(wifi)
}
