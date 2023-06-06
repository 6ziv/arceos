import os
import os.path
from sys import argv
import subprocess
import json
import itertools
import random
def feature_map(path):
    saved_cwd = os.getcwd()
    os.chdir(path)
    result1 = subprocess.run(["cargo", "read-manifest"], check=True, capture_output=True)
    os.chdir(saved_cwd)
    return list(json.loads(result1.stdout)['features'].keys())

def chk_feature_set(exec, name, path, arceos_path, features, default_features):
    args = [exec, name, "--path", path ,"--root", arceos_path]
    if len(features) != 0:
        args += ["--features", ','.join(features)]
    if not default_features:
        args += ["--no-default-features"]
    result1 = subprocess.run(args, check=True, capture_output=True)
    result2 = subprocess.run(args + ["--from-cargo-tree"], check=True, capture_output=True)
    if result1.stdout != result2.stdout:
        with open(f"{name}_1.log","wb") as fo:
            fo.write(result1.stdout)
        with open(f"{name}_2.log","wb") as fo:
            fo.write(result2.stdout)
        raise Exception(f"mismatch when testing {name}, features={features}, default_features={default_features}\n"
                        f"output saved as {name}_1.log and {name}_2.log")
    

def chk(exec, name, path, arceos_path):
    all_features = feature_map(path)
    cnt_features = len(all_features)
    feature_sets = []
    for l in range(cnt_features+1):
        for subset in itertools.combinations(all_features, l):
            feature_sets += [[list(subset)]]
    if len(feature_sets) > 100:
        random.shuffle(feature_sets)
        feature_sets = feature_sets[0:100]
    total_comb = len(feature_sets)
    i = 0
    for feature_set in feature_sets:
        print(f"Testing {name} : {i} / {total_comb}", end="\r")
        i = i + 1
        features = list(subset)
        has_default = "default" in features
        if has_default:
            features.remove("default")
        chk_feature_set(exec,name,path,arceos_path,features,has_default)
    print(f"Testing {name} -- Ok")

if __name__ == '__main__':
    random.seed(2023)
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
    print("All done!")
