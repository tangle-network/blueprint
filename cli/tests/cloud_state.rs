//! Cloud commands report what the tracker and the provider say, through the shipped process.
//!
//! `CARGO_TANGLE_TEST_BIN` points the same proof at an extracted release binary, because the
//! archives are built with a different feature resolution from this test's own build.
#![cfg(feature = "remote-providers")]
use blueprint_remote_providers::{
    CloudProvider, ResourceSpec,
    deployment::{DeploymentRecord, DeploymentType},
};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};

const REFUSAL: &str = "<Response><Errors><Error><Code>UnauthorizedOperation</Code>\
    <Message>refused</Message></Error></Errors><RequestID>r</RequestID></Response>";

/// An operator's machine: an empty HOME, optionally with tracked AWS deployments.
struct Machine(tempfile::TempDir);

impl Machine {
    fn tracking(instances: &[&str]) -> Self {
        let machine = Self(tempfile::tempdir().unwrap());
        let mut state = HashMap::new();
        for instance in instances {
            let mut record = DeploymentRecord::new(
                format!("blueprint-{instance}"),
                DeploymentType::AwsEc2,
                ResourceSpec::recommended(),
                None,
            );
            record.id = (*instance).to_owned();
            record.set_cloud_info(CloudProvider::AWS, "us-east-1".to_owned());
            record.add_resource("instance_id".to_owned(), (*instance).to_owned());
            record.add_resource("public_ip".to_owned(), "203.0.113.9".to_owned());
            state.insert(record.blueprint_id.clone(), record);
        }
        machine.write_state(&serde_json::to_string(&state).unwrap());
        machine
    }

    fn state_file(&self) -> PathBuf {
        self.0
            .path()
            .join(".tangle/remote_deployments/deployment_state.json")
    }

    fn write_state(&self, contents: &str) {
        let file = self.state_file();
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, contents).unwrap();
    }

    /// The instances still tracked, read from the file the manager also reads.
    fn tracked(&self) -> Vec<String> {
        let state: HashMap<String, DeploymentRecord> =
            serde_json::from_str(&std::fs::read_to_string(self.state_file()).unwrap()).unwrap();
        let mut instances: Vec<_> = state.into_values().map(|record| record.id).collect();
        instances.sort();
        instances
    }

    fn cli(&self, args: &[&str], aws: Option<&FakeAws>) -> Output {
        let binary = std::env::var_os("CARGO_TANGLE_TEST_BIN")
            .unwrap_or_else(|| env!("CARGO_BIN_EXE_cargo-tangle").into());
        let mut command = Command::new(binary);
        command
            .arg("cloud")
            .args(args)
            .env_clear()
            .env("HOME", self.0.path())
            .env("PATH", "/usr/bin:/bin")
            .current_dir(self.0.path())
            .stdin(Stdio::null());
        if let Some(aws) = aws {
            command
                .env("AWS_ENDPOINT_URL", &aws.endpoint)
                .env("AWS_ACCESS_KEY_ID", "test")
                .env("AWS_SECRET_ACCESS_KEY", "test")
                .env("AWS_REGION", "us-east-1")
                .env("AWS_EC2_METADATA_DISABLED", "true");
        }
        command.output().unwrap()
    }
}

/// AWS on a local port: EC2 terminates every instance but the refused one; Pricing denies access.
struct FakeAws {
    endpoint: String,
    terminated: Arc<Mutex<Vec<String>>>,
}

impl FakeAws {
    fn refusing(refused: &'static str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let terminated = Arc::new(Mutex::new(Vec::new()));
        let recorded = terminated.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                answer(stream.unwrap(), refused, &recorded);
            }
        });
        Self {
            endpoint,
            terminated,
        }
    }

    fn terminated(&self) -> Vec<String> {
        let mut instances = self.terminated.lock().unwrap().clone();
        instances.sort();
        instances
    }
}

fn answer(mut stream: TcpStream, refused: &str, terminated: &Mutex<Vec<String>>) {
    let request = read_request(&mut stream);
    let instance = request
        .split('&')
        .find_map(|pair| pair.strip_prefix("InstanceId.1="))
        .map(|value| value.trim().to_owned())
        .unwrap_or_default();
    let (status, body) = if request.to_lowercase().contains("x-amz-target") {
        (
            "403 Forbidden",
            r#"{"__type":"AccessDeniedException"}"#.to_owned(),
        )
    } else if request.contains("Action=TerminateInstances") {
        if instance == refused {
            ("403 Forbidden", REFUSAL.to_owned())
        } else {
            terminated.lock().unwrap().push(instance.clone());
            (
                "200 OK",
                format!(
                    "<TerminateInstancesResponse><requestId>r</requestId><instancesSet><item><instanceId>{instance}</instanceId></item></instancesSet></TerminateInstancesResponse>"
                ),
            )
        }
    } else {
        let state = if terminated.lock().unwrap().contains(&instance) {
            "terminated"
        } else {
            "running"
        };
        (
            "200 OK",
            format!(
                "<DescribeInstancesResponse><requestId>r</requestId><reservationSet><item><instancesSet><item><instanceId>{instance}</instanceId><instanceState><name>{state}</name></instanceState></item></instancesSet></item></reservationSet></DescribeInstancesResponse>"
            ),
        )
    };
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: text/xml\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .unwrap();
}

fn read_request(stream: &mut TcpStream) -> String {
    let mut received = Vec::new();
    let mut chunk = [0; 4096];
    loop {
        let read = stream.read(&mut chunk).unwrap();
        received.extend_from_slice(&chunk[..read]);
        let text = String::from_utf8_lossy(&received);
        let Some((head, body)) = text.split_once("\r\n\r\n") else {
            continue;
        };
        let length = head
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);
        if read == 0 || body.len() >= length {
            return text.into_owned();
        }
    }
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn an_untracked_deployment_is_an_error_without_invented_state() {
    let machine = Machine::tracking(&[]);
    for args in [
        &["status", "does-not-exist"][..],
        &["terminate", "does-not-exist", "--yes"],
    ] {
        let output = machine.cli(args, None);
        assert!(!output.status.success(), "{args:?}: {}", stdout(&output));
        assert!(stderr(&output).contains("does-not-exist"), "{args:?}");
        assert!(!stdout(&output).contains("54.123.45.67"), "{args:?}");
    }
}

#[test]
fn a_corrupt_tracker_fails_every_state_command() {
    let machine = Machine::tracking(&[]);
    machine.write_state("{ not json");
    for args in [
        &["status"][..],
        &["status", "dep-abc123"],
        &["terminate", "--all", "--yes"],
    ] {
        let output = machine.cli(args, None);
        assert!(!output.status.success(), "{args:?}: {}", stdout(&output));
        assert!(!stdout(&output).contains("dep-abc123"), "{args:?}");
    }
}

#[test]
fn status_reports_the_tracker_and_the_provider() {
    let empty = Machine::tracking(&[]);
    let output = empty.cli(&["status"], None);
    assert!(output.status.success());
    assert!(stdout(&output).contains("No tracked deployments"));

    let machine = Machine::tracking(&["i-one"]);
    let output = machine.cli(&["status"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    let listed = stdout(&output);
    for expected in [
        "i-one",
        "AWS",
        "us-east-1",
        "active",
        "unknown",
        "203.0.113.9",
    ] {
        assert!(listed.contains(expected), "{expected}: {listed}");
    }

    let aws = FakeAws::refusing("");
    let output = machine.cli(&["status", "i-one"], Some(&aws));
    assert!(output.status.success(), "{}", stderr(&output));
    let details = stdout(&output);
    for expected in ["blueprint-i-one", "AwsEc2", "Provider state:  running"] {
        assert!(details.contains(expected), "{expected}: {details}");
    }
}

#[test]
fn terminate_all_stops_every_tracked_instance_through_the_provider() {
    let machine = Machine::tracking(&["i-one", "i-two"]);
    let aws = FakeAws::refusing("");
    let output = machine.cli(&["terminate", "--all", "--yes"], Some(&aws));
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(aws.terminated(), ["i-one", "i-two"]);
    assert!(machine.tracked().is_empty());
}

#[test]
fn a_refused_termination_fails_and_keeps_that_deployment_tracked() {
    let machine = Machine::tracking(&["i-one", "i-two"]);
    let aws = FakeAws::refusing("i-two");
    let output = machine.cli(&["terminate", "--all", "--yes"], Some(&aws));
    assert!(!output.status.success(), "{}", stdout(&output));
    assert!(stderr(&output).contains("i-two"), "{}", stderr(&output));
    assert_eq!(aws.terminated(), ["i-one"]);
    assert_eq!(machine.tracked(), ["i-two"]);
}

#[test]
fn terminate_without_provider_credentials_changes_nothing() {
    let machine = Machine::tracking(&["i-one"]);
    let output = machine.cli(&["terminate", "i-one", "--yes"], None);
    assert!(!output.status.success(), "{}", stdout(&output));
    assert_eq!(machine.tracked(), ["i-one"]);
}

#[test]
fn terminate_requires_a_target() {
    let machine = Machine::tracking(&["i-one"]);
    let output = machine.cli(&["terminate", "--yes"], None);
    assert_eq!(output.status.code(), Some(2), "{}", stdout(&output));
    assert_eq!(machine.tracked(), ["i-one"]);
}

#[test]
fn configure_saves_the_region_and_stores_no_secret() {
    let machine = Machine::tracking(&[]);
    let output = machine.cli(
        &["configure", "vast", "--region", "any", "--set-default"],
        None,
    );
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("VAST_AI_API_KEY"),
        "{}",
        stdout(&output)
    );
    assert!(!machine.0.path().join(".env").exists());
    let listed = stdout(&machine.cli(&["list"], None));
    assert!(
        listed.contains("Vast.ai") && listed.contains("any"),
        "{listed}"
    );
}

#[test]
fn aws_pricing_reaches_the_provider() {
    let machine = Machine::tracking(&[]);
    let aws = FakeAws::refusing("");
    let configured = machine.cli(&["configure", "aws", "--region", "us-east-1"], Some(&aws));
    assert!(configured.status.success(), "{}", stderr(&configured));
    let output = machine.cli(
        &["estimate", "--provider", "aws", "--duration", "1h"],
        Some(&aws),
    );
    assert!(!output.status.success(), "{}", stdout(&output));
    assert!(
        stderr(&output).contains("Provider rejected its pricing credential"),
        "{}",
        stderr(&output)
    );
}
