import io
import os
import shutil
import sys

binPath = "./dist"


def build_orange_sidecar(cwd):
    os.chdir("sidecar")
#     os.system("mvn clean")
#     os.system("mvn package")
    lib_path = "../ui/src-tauri/lib"
    if os.path.exists(lib_path):
        shutil.rmtree(lib_path, True)
    mkdir(lib_path)
    conf_path = "../ui/src-tauri/conf"
    if os.path.exists(conf_path):
        shutil.rmtree(conf_path, True)
    mkdir(conf_path)

    log_path = "../ui/src-tauri/log"
    if os.path.exists(log_path):
        shutil.rmtree(log_path, True)
    mkdir(log_path)
    shutil.copyfile("./target/classes/orange.log", log_path + "/orange.log")
    shutil.copyfile("./target/orange_sidecar", lib_path + "/orange_sidecar")
    shutil.copytree("./src/main/resources/ik", conf_path + "/ik")


def mkdir(lib_path):
    try:
        os.mkdir(lib_path)
    except:
        pass


def clear_dist():
    try:
        if os.path.exists(binPath):
            shutil.rmtree(binPath, True)
            os.mkdir(binPath)
            os.mkdir(binPath + "/lib")
            os.mkdir(binPath + "/log")
            os.mkdir(binPath + "/conf")
    except:
        pass


def build_orange_ui(cwd):
    os.chdir(cwd)
    os.chdir("ui")
    os.system("yarn run build-js")
    os.system("yarn run build-binary")


def build_fsevent(cwd):
    os.chdir(cwd)
    os.chdir("fsevent/notify")
    os.system("cargo build --release")
    lib_path = "../../ui/src-tauri/lib"
    shutil.copyfile("target/release/fsevent", lib_path + "/fsevent.exe")



if __name__ == '__main__':
    cwd = os.getcwd()
    # clear_dist()

    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    build_orange_sidecar(cwd)
    build_fsevent(cwd)
    build_orange_ui(cwd)
