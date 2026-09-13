//! Resolve fixture PIDs in the collector's namespace before observing or signalling them.
use serde_json::{Value, json};
#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "linux")]
use std::path::Path;

pub enum Resolution {
    Live { pid: i32, identity: String },
    Gone,
    Unknown,
}

#[cfg(target_os = "linux")]
fn stat_fields(text: &str) -> Option<(i32, u64)> {
    // comm can contain spaces and parentheses; fields following its last ')' are fixed.
    let fields: Vec<_> = text.rsplit_once(") ")?.1.split_whitespace().collect();
    Some((fields.get(2)?.parse().ok()?, fields.get(19)?.parse().ok()?))
}

pub fn fixture_scope() -> Value {
    #[cfg(target_os = "linux")]
    {
        let namespace = fs::read_link("/proc/self/ns/pid").ok();
        let stat = fs::read_to_string("/proc/self/stat")
            .ok()
            .and_then(|text| stat_fields(&text));
        json!({"pid_namespace":namespace,"process_start_ticks":stat.map(|(_, ticks)|ticks)})
    }
    #[cfg(not(target_os = "linux"))]
    {
        json!({"pid_namespace":null,"process_start_ticks":null})
    }
}

pub fn identity(pid: i32) -> Option<String> {
    if pid <= 1 {
        return None;
    }
    #[cfg(target_os = "linux")]
    {
        let (group, start) = stat_fields(&fs::read_to_string(format!("/proc/{pid}/stat")).ok()?)?;
        Some(format!("{start}:{group}"))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let result = std::process::Command::new("ps")
            .args([
                "-p",
                &pid.to_string(),
                "-o",
                "lstart=",
                "-o",
                "stat=",
                "-o",
                "pgid=",
            ])
            .output()
            .ok()?;
        let text = String::from_utf8(result.stdout).ok()?;
        let fields: Vec<_> = text.split_whitespace().collect();
        if fields.len() < 7 {
            return None;
        }
        Some(fields[..5].join(" ") + " " + fields[6])
    }
}

pub fn resolve(receipt: &Value) -> Resolution {
    let Some(pid) = receipt["pid"]
        .as_i64()
        .and_then(|pid| i32::try_from(pid).ok())
    else {
        return Resolution::Unknown;
    };
    #[cfg(not(target_os = "linux"))]
    {
        match identity(pid) {
            Some(identity) => Resolution::Live { pid, identity },
            None => Resolution::Gone,
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::MetadataExt;
        let Some(namespace) = receipt["pid_namespace"].as_str() else {
            return Resolution::Unknown;
        };
        let Some(start) = receipt["process_start_ticks"].as_u64() else {
            return Resolution::Unknown;
        };
        let Ok(entries) = fs::read_dir("/proc") else {
            return Resolution::Unknown;
        };
        let mut complete = true;
        for entry in entries.flatten() {
            let Ok(host_pid) = entry.file_name().to_string_lossy().parse::<i32>() else {
                continue;
            };
            let root = entry.path();
            let Ok(text) = fs::read_to_string(root.join("stat")) else {
                if root
                    .metadata()
                    .is_ok_and(|m| m.uid() == unsafe { libc::geteuid() })
                {
                    complete = false;
                }
                continue;
            };
            let Some((group, candidate_start)) = stat_fields(&text) else {
                continue;
            };
            if candidate_start != start {
                continue;
            }
            let Ok(candidate_namespace) = fs::read_link(root.join("ns/pid")) else {
                // Ignore unrelated users; an unreadable candidate owned by us is ambiguous.
                if root
                    .metadata()
                    .is_ok_and(|m| m.uid() == unsafe { libc::geteuid() })
                {
                    complete = false;
                }
                continue;
            };
            if candidate_namespace != Path::new(namespace) {
                continue;
            }
            let inner_pid = fs::read_to_string(root.join("status"))
                .ok()
                .and_then(|status| {
                    status
                        .lines()
                        .find(|line| line.starts_with("NSpid:"))?
                        .split_whitespace()
                        .last()?
                        .parse::<i32>()
                        .ok()
                });
            if inner_pid != Some(pid) {
                continue;
            }
            if let Some(identity) =
                identity(host_pid).filter(|id| id == &format!("{start}:{group}"))
            {
                return Resolution::Live {
                    pid: host_pid,
                    identity,
                };
            }
        }
        if complete {
            Resolution::Gone
        } else {
            Resolution::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_fixture_identity_resolves_without_guessing_a_host_pid() {
        let mut receipt = fixture_scope();
        receipt["pid"] = json!(std::process::id());
        assert!(
            matches!(resolve(&receipt), Resolution::Live {pid, ..} if pid == std::process::id() as i32)
        );
        #[cfg(target_os = "linux")]
        {
            receipt["pid_namespace"] = json!("pid:[invalid]");
            assert!(!matches!(resolve(&receipt), Resolution::Live { .. }));
            receipt["pid_namespace"] = Value::Null;
            assert!(matches!(resolve(&receipt), Resolution::Unknown));
        }
    }
}
