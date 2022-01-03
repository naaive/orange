import os
import shutil

if __name__ == '__main__':
    os.system("yarn run build")
    binPath = "../../../dist"
    if os.path.exists(binPath):
        shutil.rmtree(binPath, True)
    shutil.copytree("./build", binPath )