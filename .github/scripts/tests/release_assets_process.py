#!/usr/bin/env python3
"""Integration proof through the verifier CLI and external Git/GitHub child processes."""

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

SCRIPT = Path(__file__).resolve().parents[1] / "verify_release_tags.py"
TAG = "cargo-tangle-v0.5.0-alpha.28"
SHA = "f88c5c42fa678dbcdcd221d32810256218f93f4c"


def proof():
    with tempfile.TemporaryDirectory(prefix="release-assets-process-") as folder:
        root = Path(folder)
        (root / "cli").mkdir()
        (root / "cli/Cargo.toml").write_text('[package]\nname="cargo-tangle"\nversion="0.5.0-alpha.28"\n')
        (root / "target/distrib").mkdir(parents=True)
        plan = {"announcement_tag": TAG, "artifacts": {"cli.tar.xz": {"checksum": "cli.tar.xz.sha256"}, "cli.tar.xz.sha256": {}}}
        (root / "plan.json").write_text(json.dumps(plan))
        (root / "release.json").write_text(json.dumps([{"package_name": "cargo-tangle", "version": "0.5.0-alpha.28", "tag": TAG}]))
        content = {"cli.tar.xz": "artifact fixture input", "cli.tar.xz.sha256": ""}
        content["cli.tar.xz.sha256"] = hashlib.sha256(content["cli.tar.xz"].encode()).hexdigest() + " *cli.tar.xz\n"
        record = {"schema_version": 1, "tag": TAG, "source_sha": SHA,
                  "sha256": {name: hashlib.sha256(value.encode()).hexdigest() for name, value in content.items()}}
        fixture = {"record": record, "content": content, "complete": False, "partial": False, "fail": False, "head": SHA, "tagged": SHA}
        state = root / "fixture.json"
        binary = root / "bin"
        binary.mkdir()
        (binary / "git").write_text('#!/usr/bin/env python3\nimport json,os,sys\nf=json.load(open(os.environ["RELEASE_FIXTURE"]))\nprint(f["head"] if sys.argv[-1]=="HEAD" else f["tagged"])\n')
        (binary / "gh").write_text('''#!/usr/bin/env python3
import hashlib,json,os,sys
from pathlib import Path
p=Path(os.environ["RELEASE_FIXTURE"]);f=json.loads(p.read_text());a=sys.argv[1:]
with (p.parent/"argv.jsonl").open("a") as out: out.write(json.dumps(a)+"\\n")
if f["fail"]: print("credential-reflection-canary",file=sys.stderr);sys.exit(1)
if a[:2]==["release","upload"]:
 for value in a[3:]:
  path=Path(value)
  if path.is_file():
   if path.name=="release-source.json": f["record_bytes"]=path.read_text();f["record"]=json.loads(f["record_bytes"])
   else: f["content"][path.name]=path.read_text()
   if path.name not in f["published"]: f["published"].append(path.name)
 p.write_text(json.dumps(f));sys.exit(0)
if a[:2]==["release","edit"]: sys.exit(0)
endpoint=next(x for x in a if x.startswith("repos/"))
if endpoint.endswith("/dispatches"):
 print(json.dumps({"workflow_run_id":99,"run_url":"https://api.github.com/repos/tangle-network/blueprint/actions/runs/99","html_url":"https://github.com/tangle-network/blueprint/actions/runs/99"}));sys.exit(0)
if "/releases/tags/" in endpoint:
 allnames=list(f["content"])+["release-source.json"]
 names=f["published"] if "published" in f else (allnames[1:] if f["partial"] else (allnames if f["complete"] else []))
 values={**f["content"],"release-source.json":f.get("record_bytes",json.dumps(f["record"],sort_keys=True))}
 print(json.dumps({"assets":[{"id":i,"name":n,"digest":"sha256:"+hashlib.sha256(values[n].encode()).hexdigest()} for i,n in enumerate(allnames) if n in names]}));sys.exit(0)
i=int(endpoint.rsplit("/",1)[1]);values=[*f["content"].values(),f.get("record_bytes",json.dumps(f["record"],sort_keys=True))]
sys.stdout.write(values[i])
''')
        for executable in binary.iterdir(): executable.chmod(0o700)
        env = {**os.environ, "PATH": str(binary)+os.pathsep+os.environ["PATH"],
               "RELEASE_FIXTURE": str(state), "GITHUB_REPOSITORY": "tangle-network/blueprint", "GITHUB_SERVER_URL": "https://github.com", "GITHUB_API_URL": "https://api.github.com"}
        def invoke(mode="check", expected=0):
            state.write_text(json.dumps(fixture))
            args = [sys.executable, str(SCRIPT), "--artifacts", mode]
            args += ["release.json"] if mode=="dispatch" else ["--plan", "plan.json", "--tag", TAG]
            result = subprocess.run(args, cwd=root, env=env, capture_output=True, text=True)
            assert result.returncode==expected, (mode, expected, result.returncode, result.stdout, result.stderr)
            assert "credential-reflection-canary" not in result.stdout+result.stderr
            return result
        invoke(expected=3)
        fixture["complete"] = True
        invoke()
        fixture["record"]["source_sha"] = "0"*40
        invoke(expected=2)
        fixture["partial"] = True
        invoke(expected=2)
        fixture["partial"] = False
        fixture["record"]["source_sha"] = SHA
        fixture["head"] = "0"*40
        invoke(expected=2)
        fixture["head"] = SHA
        good = fixture["content"]["cli.tar.xz.sha256"]
        fixture["content"]["cli.tar.xz.sha256"] = "0"*64+"  cli.tar.xz\n"
        invoke(expected=3)
        fixture["content"]["cli.tar.xz.sha256"] = good
        fixture["fail"] = True
        invoke(expected=2)
        fixture["fail"] = False
        result = invoke("dispatch")
        assert '99' in result.stdout, result.stdout
        requests = [json.loads(line) for line in (root / "argv.jsonl").read_text().splitlines()]
        assert ["api", "--method", "POST", "repos/tangle-network/blueprint/actions/workflows/release.yml/dispatches", "-H", "X-GitHub-Api-Version: 2026-03-10", "-f", "ref=main", "-f", f"inputs[tag]={TAG}"] in requests
        (root / "release.json").write_text(json.dumps([{"package_name": "cargo-tangle", "version": "0.5.0-alpha.28", "tag": "main"}]))
        invoke("dispatch", expected=2)
        (root / "release.json").write_text(json.dumps([{"package_name": "blueprint-core", "version": "1.0.0", "tag": "blueprint-core-v1.0.0"}]))
        assert "No cargo-tangle" in invoke("dispatch").stdout
        for name, value in content.items(): (root / "target/distrib" / name).write_text(value)
        invoke("record")
        written = json.loads((root / "target/distrib/release-source.json").read_text())
        assert written==record, written
        # Execute the actual workflow's host/publication shell, using external tool peers.
        (root / ".github").mkdir()
        (root / ".github/scripts").symlink_to(SCRIPT.parent, target_is_directory=True)
        (root / "artifacts").mkdir()
        for name, value in content.items(): (root / "artifacts" / name).write_text(value)
        (binary / "dist").write_text('#!/usr/bin/env python3\nprint("{}")\n')
        (binary / "jq").write_text('#!/usr/bin/env python3\nfrom pathlib import Path\nimport sys\nprint(Path(sys.argv[-1]).read_text().strip())\n')
        for executable in binary.iterdir(): executable.chmod(0o700)
        workflow = (SCRIPT.parents[1] / "workflows/release.yml").read_text().splitlines()
        def step(marker):
            start = next(i for i, line in enumerate(workflow) if line.strip()==marker)
            start = next(i for i in range(start,len(workflow)) if workflow[i].strip()=="run: |")
            lines = []
            for line in workflow[start+1:]:
                if line.strip() and not line.startswith("          "): break
                lines.append(line[10:])
            return "\n".join(lines).replace("${{ needs.plan.outputs.tag-flag }}", f"--tag={TAG}")
        shell = step("- id: host")+"\n"+step("- name: Create GitHub Release")
        assert "${{" not in shell, "unresolved workflow process input"
        fixture["published"] = []
        state.write_text(json.dumps(fixture))
        env.update({"RELEASE_TAG":TAG,"RELEASE_COMMIT":SHA,"PRERELEASE_FLAG":"--prerelease","GITHUB_OUTPUT":str(root/"outputs.txt")})
        result = subprocess.run(["bash","-euo","pipefail","-c",shell],cwd=root,env=env,text=True,capture_output=True)
        assert result.returncode==0, (result.stdout,result.stderr)
        live = json.loads(state.read_text())
        assert set(live["published"])==set(content)|{"release-source.json"}, live
        assert "source-bound artifacts uploaded and verified" in result.stdout
        print(json.dumps({"cases":12,"missing_assets":"incomplete","wrong_source":"error",
                          "bad_checksum":"incomplete","unknown_github":"error","dispatch_receipt":99,"workflow_publication":"complete_after_upload"}))


if __name__ == "__main__":
    proof()
