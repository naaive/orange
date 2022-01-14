import io
import os
import shutil
import sys

binPath = "./dist"


def build_orange_sidecar():
    os.system("mvn clean")
    os.system("mvn package")
    lib_path = "./src/main/webapp/src-tauri/lib"
    if os.path.exists(lib_path):
        shutil.rmtree(lib_path, True)
    mkdir(lib_path)
    conf_path = "./src/main/webapp/src-tauri/conf"
    if os.path.exists(conf_path):
        shutil.rmtree(conf_path, True)
    mkdir(conf_path)

    log_path = "./src/main/webapp/src-tauri/log"
    if os.path.exists(log_path):
        shutil.rmtree(log_path, True)
    mkdir(log_path)
    shutil.copyfile("./target/classes/orange.log", log_path + "/orange.log")
    shutil.copyfile("./target/orange_sidecar.exe", lib_path + "/orange_sidecar.exe")
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
    os.chdir("./src/main/webapp")
    os.system("yarn run build-js")
    os.system("yarn run build-binary")


def build_fsevent(cwd):
    os.chdir(cwd)
    os.chdir("src/main/py")
    os.system("python -m nuitka --standalone main.py")
    lib_path = "../../../src/main/webapp/src-tauri/lib"
    shutil.copyfile("./main.dist/main.exe", lib_path + "/fsevent.exe")
    shutil.copyfile("./main.dist/libffi-7.dll", lib_path + "/libffi-7.dll")
    shutil.copyfile("./main.dist/python38.dll", lib_path + "/python38.dll")
    shutil.copyfile("./main.dist/_ctypes.pyd", lib_path + "/_ctypes.pyd")
    shutil.rmtree("./main.build", True)
    shutil.rmtree("./main.dist", True)


if __name__ == '__main__':
    cwd = os.getcwd()
    clear_dist()

    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    build_orange_sidecar()
    build_fsevent(cwd)
    build_orange_ui(cwd)
