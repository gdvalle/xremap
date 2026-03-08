use crate::config::dbus_action::{BusType, DbusMethodCall};
use log::error;
use std::sync::mpsc;

pub struct DbusWorker {
    sender: mpsc::Sender<DbusMethodCall>,
}

impl DbusWorker {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<DbusMethodCall>();
        std::thread::spawn(move || {
            let mut session: Option<zbus::blocking::Connection> = None;
            let mut system: Option<zbus::blocking::Connection> = None;
            for call in rx {
                Self::execute(&mut session, &mut system, &call);
            }
        });
        DbusWorker { sender: tx }
    }

    pub fn send(&self, call: DbusMethodCall) {
        if let Err(e) = self.sender.send(call) {
            error!("D-Bus worker channel closed: {e}");
        }
    }

    fn execute(
        session: &mut Option<zbus::blocking::Connection>,
        system: &mut Option<zbus::blocking::Connection>,
        call: &DbusMethodCall,
    ) {
        let slot = match call.bus {
            BusType::Session => &mut *session,
            BusType::System => &mut *system,
        };
        let conn = match Self::get_or_connect(slot, &call.bus) {
            Ok(c) => c,
            Err(e) => {
                error!("D-Bus connect failed: {e}");
                return;
            }
        };
        let result = conn.call_method(Some(&call.destination), &call.path, call.interface.as_ref(), &call.method, &());
        if let Err(e) = result {
            error!("D-Bus {}.{} failed: {e}", call.destination, call.method);
            if matches!(e, zbus::Error::InputOutput(_)) {
                *slot = None;
            }
        }
    }

    fn get_or_connect<'a>(
        slot: &'a mut Option<zbus::blocking::Connection>,
        bus: &BusType,
    ) -> Result<&'a zbus::blocking::Connection, zbus::Error> {
        if slot.is_none() {
            *slot = Some(match bus {
                BusType::Session => zbus::blocking::Connection::session()?,
                BusType::System => zbus::blocking::Connection::system()?,
            });
        }
        Ok(slot.as_ref().unwrap())
    }
}
