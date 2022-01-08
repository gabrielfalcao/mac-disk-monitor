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
        let re =
            Regex::new(r"^[*]{3}(\w+)\s*\(('([^']+)')?,\s*DAVolumePath\s*=\s*('([^']+)')?\s*,\s*DAVolumeKind\s*=\s*('([^']+)')?\s*,\s*DAVolumeName\s*=\s*('([^']+)')?\s*.*\)")
                .unwrap();
        for cap in re.captures_iter(line) {
            event.set_name(&cap[1]);
            event.set_bsd_name(&cap[3]);
            event.set_path(&cap[5]);
            event.set_kind(&cap[7]);
            event.set_volume_name(&cap[9]);
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
        self.volume_path = Some(String::from(path));
    }
    pub fn path(&self) -> Option<String> {
        self.volume_path.clone()
    }
    pub fn set_kind(&mut self, kind: &str) {
        self.volume_kind = Some(String::from(kind));
    }
    pub fn kind(&self) -> Option<String> {
        self.volume_kind.clone()
    }
    pub fn set_volume_name(&mut self, name: &str) {
        self.volume_name = Some(String::from(name));
    }
    pub fn volume_name(&self) -> Option<String> {
        self.volume_name.clone()
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
    }
}

fn main() {}
