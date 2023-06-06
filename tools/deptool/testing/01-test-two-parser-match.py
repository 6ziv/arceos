import os
import os.path
from sys import argv
import subprocess
def chk(exec, name, path, arceos_path):
    result1 = subprocess.run([exec, name, "--path", path ,"--root", arceos_path], check=True, capture_output=True)
    result2 = subprocess.run([exec, name, "--path", path ,"--root", arceos_path, "--from-cargo-tree"], check=True, capture_output=True)
    if result1.stdout == result2.stdout:
        print(f"Test {name} -- Ok")
    else:
        with open(f"{name}_1.log","wb") as fo:
            fo.write(result1.stdout)
        with open(f"{name}_2.log","wb") as fo:
            fo.write(result2.stdout)
        print(f"Test {name} -- Failed")
    pass

if __name__ == '__main__':
    print(f"deptool executable is {argv[1]}")
    print(f"arceos root is {argv[2]}")
    deptool = argv[1]
    arceos_path = argv[2]
    for path, _, files in os.walk(os.path.join(arceos_path,"crates")):
        for file in files:
            if file == 'Cargo.toml':
                _, name = os.path.split(path)
                chk(deptool, name, path, arceos_path)
    for path, _, files in os.walk(os.path.join(arceos_path,"modules")):
        for file in files:
            if file == 'Cargo.toml':
                _, name = os.path.split(path)
                chk(deptool, name, path, arceos_path)
    for path, _, files in os.walk(os.path.join(arceos_path,"apps")):
        for file in files:
            if file == 'Cargo.toml':
                _, name = os.path.split(path)
                chk(deptool, name, path, arceos_path)
