#!/usr/bin/env python3
import os, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
LEAN = ROOT / ".lake" / "build" / "bin" / "stage_minus_one"
RUST = ROOT / "rust" / "target" / "debug" / "stage-minus-one-rust"
FIXTURES = ["nested", "inherit"]
FAULTS = ["drop_text", "off_by_one_width", "wrong_sibling_y", "wrong_paint_order"]

def run(cmd, env=None):
    return subprocess.check_output([str(x) for x in cmd], text=True, env=env).strip()

def compare(fixture, fault=None):
    expected = run([LEAN, fixture])
    env = os.environ.copy()
    if fault: env["STAGE_MINUS_ONE_FAULT"] = fault
    actual = run([RUST, fixture], env=env)
    return expected == actual

def main():
    bad = []
    for f in FIXTURES:
        if not compare(f): bad.append(f"baseline mismatch: {f}")
    for fault in FAULTS:
        detected = any(not compare(f, fault) for f in FIXTURES)
        if not detected: bad.append(f"fault escaped: {fault}")
    if bad:
        print("FAIL")
        print("\n".join(bad))
        return 1
    print(f"PASS: {len(FIXTURES)} fixtures agree; {len(FAULTS)} deliberate faults detected")
    return 0
if __name__ == "__main__": sys.exit(main())
