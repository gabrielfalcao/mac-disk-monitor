#![allow(unused_variables)]
#![allow(dead_code)]
use regex::Regex;

pub struct Event {
    name: Option<String>,
    time: String,
    bsd_name: Option<String>,
    volume_path: Option<String>,
    volume_kind: Option<String>,
    volume_name: Option<String>,
}

impl Event {
    pub fn empty() -> Event {
        Event {
            name: None,
            time: String::new(),
            bsd_name: None,
            volume_path: None,
            volume_kind: None,
            volume_name: None,
        }
    }
    pub fn from_line(line: &str) -> Event {
        let mut event = Event::empty();
        //\s*\(('(?P<bsd_name>[^']+)')?, DAVolumePath\s*=\s*(?P<path>'[^']+')\)
        match extract_base_metadata(line) {
            Some((name, bsd_name, time)) => {
                event.set_name(name.as_str());
                match bsd_name {
                    Some(bsd_name) => {
                        event.set_bsd_name(bsd_name.as_str());
                    }
                    None => {}
                }
                event.set_time_string(time.as_str());
            }
            None => {}
        }
        match extract_volume_path(line) {
            Some(path) => {
                event.set_path(path.as_str());
            }
            None => {}
        }
        match extract_volume_kind(line) {
            Some(kind) => {
                event.set_kind(kind.as_str());
            }
            None => {}
        }
        match extract_volume_name(line) {
            Some(name) => {
                event.set_volume_name(name.as_str());
            }
            None => {}
        }

        event
    }
    pub fn name(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => String::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(String::from(name));
    }
    pub fn set_bsd_name(&mut self, bsd_name: &str) {
        self.bsd_name = Some(String::from(bsd_name));
    }
    pub fn bsd_name(&self) -> Option<String> {
        self.bsd_name.clone()
    }
    pub fn set_path(&mut self, path: &str) {
        self.volume_path = if !path.eq("<null>") {
            Some(String::from(path))
        } else {
            None
        };
    }
    pub fn path(&self) -> Option<String> {
        self.volume_path.clone()
    }
    pub fn set_kind(&mut self, kind: &str) {
        self.volume_kind = if !kind.eq("<null>") {
            Some(String::from(kind))
        } else {
            None
        };
    }
    pub fn kind(&self) -> Option<String> {
        self.volume_kind.clone()
    }
    pub fn set_volume_name(&mut self, name: &str) {
        self.volume_name = if !name.eq("<null>") {
            Some(String::from(name))
        } else {
            None
        };
    }
    pub fn volume_name(&self) -> Option<String> {
        self.volume_name.clone()
    }
    pub fn set_time_string(&mut self, time: &str) {
        self.time = String::from(time);
    }
    pub fn time_string(&self) -> String {
        self.time.clone()
    }
}

pub fn extract_base_metadata(line: &str) -> Option<(String, Option<String>, String)> {
    let re = Regex::new(r"^[*]{3}(\w+)\s*\('?([^,']+)'?.*?\)\s*Time=(\S+)").unwrap();
    match re.captures(line) {
        Some(caps) => {
            let name = caps.get(1).unwrap().as_str().to_string();
            let bsd_name = Some(caps.get(2).unwrap().as_str().to_string());
            let time = caps.get(3).unwrap().as_str().to_string();
            Some((name, bsd_name, time))
        }
        None => None,
    }
}
pub fn extract_volume_path(line: &str) -> Option<String> {
    let re = Regex::new(r"DAVolumePath\s*=\s*('([^']+)')").unwrap();

    match re.captures(line) {
        Some(caps) => {
            let name = caps.get(2).unwrap().as_str().to_string();
            Some(name)
        }
        None => None,
    }
}
pub fn extract_volume_kind(line: &str) -> Option<String> {
    let re = Regex::new(r"DAVolumeKind\s*=\s*('([^']+)')").unwrap();

    match re.captures(line) {
        Some(caps) => {
            let name = caps.get(2).unwrap().as_str().to_string();
            Some(name)
        }
        None => None,
    }
}
pub fn extract_volume_name(line: &str) -> Option<String> {
    let re = Regex::new(r"DAVolumeName\s*=\s*('([^']+)')").unwrap();

    match re.captures(line) {
        Some(caps) => {
            let name = caps.get(2).unwrap().as_str().to_string();
            Some(name)
        }
        None => None,
    }
}
#[cfg(test)]
mod tests {
    use super::Event;
    use k9::assert_equal;

    #[test]
    fn test_parse_disk_appeared_with_volume_path() {
        let line = String::from("***DiskAppeared ('disk4', DAVolumePath = 'file:///Volumes/my%20backups/', DAVolumeKind = 'hfs', DAVolumeName = 'Time Machine Backups') Time=20220108-20:22:05.1438");
        let disk_appeared = Event::from_line(line.as_str());

        assert_equal!(disk_appeared.name().as_str(), "DiskAppeared");
        assert_equal!(disk_appeared.bsd_name(), Some(String::from("disk4")));
        assert_equal!(
            disk_appeared.path(),
            Some(String::from("file:///Volumes/my%20backups/"))
        );
        assert_equal!(disk_appeared.kind(), Some(String::from("hfs")));
        assert_equal!(
            disk_appeared.volume_name(),
            Some(String::from("Time Machine Backups"))
        );
        assert_equal!(
            disk_appeared.time_string().as_str(),
            "20220108-20:22:05.1438"
        );
    }
    #[test]
    fn test_parse_disk_appeared_without_volume_path_kind_and_name() {
        let line = String::from("***DiskAppeared ('disk3s2', DAVolumePath = '<null>', DAVolumeKind = '<null>', DAVolumeName = '<null>') Time=20220108-20:22:05.1453");
        let disk_appeared = Event::from_line(line.as_str());

        assert_equal!(disk_appeared.name().as_str(), "DiskAppeared");
        assert_equal!(disk_appeared.bsd_name(), Some(String::from("disk3s2")));
        assert_equal!(disk_appeared.path(), None);
        assert_equal!(disk_appeared.kind(), None);
        assert_equal!(disk_appeared.volume_name(), None);
        assert_equal!(
            disk_appeared.time_string().as_str(),
            "20220108-20:22:05.1453"
        );
    }
    #[test]
    fn test_parse_disk_appeared_without_bsd_name() {
        let line = String::from("***DiskAppeared ((no BSD name), DAVolumePath = 'file:///System/Volumes/Data/home/', DAVolumeKind = 'autofs', DAVolumeName = '<null>') Time=20220108-20:22:05.1457");
        let disk_appeared = Event::from_line(line.as_str());

        assert_equal!(disk_appeared.name().as_str(), "DiskAppeared");
        assert_equal!(disk_appeared.bsd_name(), None);
        assert_equal!(
            disk_appeared.path(),
            Some(String::from("file:///System/Volumes/Data/home/"))
        );
        assert_equal!(disk_appeared.kind(), Some(String::from("autofs")));
        assert_equal!(disk_appeared.volume_name(), None);
        assert_equal!(
            disk_appeared.time_string().as_str(),
            "20220108-20:22:05.1457"
        );
    }
}

fn main() {}
