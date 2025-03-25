use sysinfo::{System, Components, Disks};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub system: SystemDetails,
    pub memory: MemoryInfo,
    pub disks: Vec<DiskInfo>,
    pub components: Vec<ComponentInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct SystemDetails {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_ram_gb: f64,
    pub total_ram_mb: f64,
    pub used_ram_gb: f64,
    pub used_ram_mb: f64,
    pub ram_percent: f64,
    pub total_swap_gb: f64,
    pub total_swap_mb: f64,
    pub used_swap_gb: f64,
    pub used_swap_mb: f64,
    pub swap_percent: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub available_space_gb: f64,
    pub available_space_mb: f64,
    pub total_space_gb: f64,
    pub total_space_mb: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ComponentInfo {
    pub label: String,
    pub temperature: Option<f32>,
}

pub async fn get_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let system = SystemDetails {
        name: System::name().unwrap_or_default(),
        kernel_version: System::kernel_version().unwrap_or_default(),
        os_version: System::os_version().unwrap_or_default(),
        host_name: System::host_name().unwrap_or_default(),
    };

    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let total_swap = sys.total_swap() as f64;
    let used_swap = sys.used_swap() as f64;

    let memory = MemoryInfo {
        total_ram_gb: total_memory / 1_000_000_000.0,
        total_ram_mb: total_memory / 1_000_000.0,
        used_ram_gb: used_memory / 1_000_000_000.0,
        used_ram_mb: used_memory / 1_000_000.0,
        ram_percent: if total_memory > 0.0 { (used_memory / total_memory) * 100.0 } else { 0.0 },
        total_swap_gb: total_swap / 1_000_000_000.0,
        total_swap_mb: total_swap / 1_000_000.0,
        used_swap_gb: used_swap / 1_000_000_000.0,
        used_swap_mb: used_swap / 1_000_000.0,
        swap_percent: if total_swap > 0.0 { (used_swap / total_swap) * 100.0 } else { 0.0 },
    };

    let disks = Disks::new_with_refreshed_list();
    let disks_info: Vec<DiskInfo> = disks
        .iter()
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            mount_point: disk.mount_point().to_string_lossy().into_owned(),
            available_space_gb: disk.available_space() as f64 / 1_000_000_000.0,
            available_space_mb: disk.available_space() as f64 / 1_000_000.0,
            total_space_gb: disk.total_space() as f64 / 1_000_000_000.0,
            total_space_mb: disk.total_space() as f64 / 1_000_000.0,
        })
        .collect();

    let components = Components::new_with_refreshed_list();
    let components_info: Vec<ComponentInfo> = components
        .iter()
        .map(|component| {
            let label = match component.label().to_lowercase().as_str() {
                "iwlwifi_1 temp1" => "Wi-Fi Module",
                "sensor 1" => "Sensor 1",
                "sensor 2" => "Sensor 2",
                "composite" => "Chipset",
                "edge" => "GPU (Edge)",
                "tctl" => "CPU (Tctl)",
                _ => component.label(),
            };
            ComponentInfo {
                label: label.to_string(),
                temperature: component.temperature(),
            }
        })
        .collect();

    SystemInfo {
        system,
        memory,
        disks: disks_info,
        components: components_info,
    }
}