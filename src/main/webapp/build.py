import os
import shutil

if __name__ == '__main__':
    os.system("yarn run build-js")
    os.system("yarn run build-binary")
    binPath = "../../../dist"
    if os.path.exists(binPath):
        shutil.rmtree(binPath, True)
    shutil.copytree("./src-tauri/target/release", binPath)
