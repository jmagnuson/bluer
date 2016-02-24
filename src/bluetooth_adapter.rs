use dbus::{Connection, BusType, Props, Message};
use bluetooth_utils;
use bluetooth_device::BluetoothDevice;

static ADAPTER_INTERFACE: &'static str = "org.bluez.Adapter1";
static SERVICE_NAME: &'static str = "org.bluez";

#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    object_path: String,
}

impl BluetoothAdapter {
    fn new(object_path: String) -> BluetoothAdapter {
        BluetoothAdapter {
            object_path: object_path,
        }
    }

    pub fn init() -> Result<BluetoothAdapter,String> {
        let adapters = bluetooth_utils::get_adapters();

        if adapters.is_empty() {
            return Err(String::from("Bluetooth adapter not found"))
        }

        Ok(BluetoothAdapter::new(adapters[0].clone()))
    }

    pub fn get_first_device(&self) -> Result<BluetoothDevice, String> {
        let devices = bluetooth_utils::list_devices(&self.object_path);

        if devices.is_empty() {
            return Err(String::from("No device found."))
        }
        Ok(BluetoothDevice::create_device(devices[0].clone()))
    }

    pub fn get_devices(&self) -> Vec<String> {
        bluetooth_utils::list_devices(&self.object_path)
    }

    pub fn get_object_path(&self) -> String {
        self.object_path.clone()
    }

    pub fn get_address(&self) -> String {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        String::from(d.get("Address").unwrap().inner::<&str>().unwrap())
    }

    pub fn get_name(&self) -> String {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        String::from(d.get("Name").unwrap().inner::<&str>().unwrap())
    }

    pub fn get_alias(&self) -> String {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        String::from(d.get("Alias").unwrap().inner::<&str>().unwrap())
    }

    pub fn set_alias(&self, value: String) {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.set("Alias", value.into()).unwrap();
    }

    pub fn get_class(&self) -> u32 {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.get("Class").unwrap().inner::<u32>().unwrap()
    }

    pub fn is_powered(&self) -> bool {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.get("Powered").unwrap().inner::<bool>().unwrap()
    }

    pub fn set_powered(&self, value: bool) {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.set("Powered", value.into()).unwrap();
    }

    pub fn get_rssi(&self) -> i32 {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.get("RSSI").unwrap().inner::<i32>().unwrap()        
    }

    pub fn is_discoverable(&self) -> bool {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.get("Discoverable").unwrap().inner::<bool>().unwrap()
    }

    pub fn set_discoverable(&self, value: bool) {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.set("Discoverable", value.into()).unwrap();
    }

    pub fn is_discovering(&self) -> bool {
        let c = Connection::get_private(BusType::System).unwrap();
        let d = Props::new(&c, SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, 10000);
        d.get("Discovering").unwrap().inner::<bool>().unwrap()
    }

    pub fn start_discovery(&self) {
        let c = Connection::get_private(BusType::System).unwrap();
        let m = Message::new_method_call(SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, "StartDiscovery").unwrap();
        c.send(m).unwrap();
    }

    pub fn stop_discovery(&self) {
        let c = Connection::get_private(BusType::System).unwrap();
        let m = Message::new_method_call(SERVICE_NAME, &self.object_path, ADAPTER_INTERFACE, "StopDiscovery").unwrap();
        c.send(m).unwrap();
    }

}
