//! Owned subprocess peers; no live host authentication or GPU-visibility proof.
use std::os::unix::fs::PermissionsExt;
pub const SECRET: &str = "synthetic-ssh-workload-secret";

pub fn command(
    root: &std::path::Path,
    executable: &std::path::Path,
    test: &str,
    mode: &str,
) -> anyhow::Result<std::process::Command> {
    let bin = root.join("bin");
    std::fs::create_dir(&bin)?;
    std::fs::write(root.join("kept"), "preexisting")?;
    std::fs::create_dir_all(root.join(".ssh"))?;
    std::fs::create_dir_all(root.join("etc/systemd/system"))?;
    assert!(
        std::process::Command::new("ssh-keygen")
            .args(["-q", "-t", "ed25519", "-N", "", "-f"])
            .arg(root.join("host"))
            .status()?
            .success()
    );
    std::fs::write(
        root.join(".ssh/known_hosts"),
        format!(
            "gpu-peer.invalid {}",
            std::fs::read_to_string(root.join("host.pub"))?
        ),
    )?;
    let ssh = r#"#!/usr/bin/env python3
import json,os,pathlib,subprocess,sys
root=pathlib.Path(os.environ['SSH_PROCESS_ROOT'])
args=sys.argv[1:]
with (root/'ssh-argv.jsonl').open('a') as f: f.write(json.dumps(args)+'\n')
trusted='StrictHostKeyChecking=yes' in args and not any('/dev/null' in x for x in args)
trusted=trusted and (root/'.ssh/known_hosts').read_text().startswith('gpu-peer.invalid ssh-ed25519 ')
if os.environ['SSH_PROCESS_MODE']=='untrusted' or not trusted: sys.exit(255)
command=args[-1].replace('/opt/',str(root)+'/opt/').replace('/etc/systemd/',str(root)+'/etc/systemd/')
payload=sys.stdin.buffer.read() if command=='sh -se' or command.startswith("sh -c ") else None
failed=os.environ['SSH_PROCESS_MODE']=='failed-transfer' and command.startswith("sh -c ") and "cat >" in command
if failed: payload=payload[:len(payload)//2]
if command=='sh -se': payload=payload.replace(b'/opt/',(str(root)+'/opt/').encode()).replace(b'/etc/systemd/',(str(root)+'/etc/systemd/').encode())
status=subprocess.run(['/bin/sh','-c',command],input=payload).returncode
sys.exit(255 if failed else status)
"#;
    let docker = r#"#!/usr/bin/env python3
import json,os,pathlib,stat,sys
root=pathlib.Path(os.environ['SSH_PROCESS_ROOT']);args=sys.argv[1:]
if args[0]=='create':
    path=pathlib.Path(args[args.index('--env-file')+1])
    receipt={'arguments':args,'mode':stat.S_IMODE(path.stat().st_mode),'loaded':os.environ['SSH_PROCESS_SECRET'] in path.read_text()}
    (root/'docker.json').write_text(json.dumps(receipt));print('a'*64)
elif args[0]=='inspect': print(json.dumps([{'State':{'Status':'running'},'NetworkSettings':{'Ports':{}}}]))
elif args[0]=='--version': print('Docker process peer')
"#;
    let curl = r#"#!/usr/bin/env python3
import pathlib,sys
pathlib.Path(sys.argv[sys.argv.index('-o')+1]).write_bytes(b'#!/bin/sh\nexit 0\n')
"#;
    let sudo = "#!/bin/sh\n[ \"$1\" != -n ] || shift\ncase \"$1\" in useradd|chown) exit 0;; esac\nexec \"$@\"\n";
    for (name, source) in [
        ("ssh", ssh),
        ("docker", docker),
        ("curl", curl),
        ("sudo", sudo),
        ("systemctl", "#!/bin/sh\nexit 0\n"),
        ("sleep", "#!/bin/sh\nexit 0\n"),
    ] {
        let path = bin.join(name);
        std::fs::write(&path, source)?;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
    }
    let mut command = std::process::Command::new(executable);
    command
        .args(["--exact", test, "--nocapture", "--test-threads=1"])
        .env_clear()
        .env("PATH", format!("{}:/usr/bin:/bin", bin.display()))
        .env("HOME", root)
        .env("SSH_PROCESS_ROOT", root)
        .env("SSH_PROCESS_MODE", mode)
        .env("SSH_PROCESS_SECRET", SECRET);
    Ok(command)
}
