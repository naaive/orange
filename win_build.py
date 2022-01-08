import io
import os
import shutil
import sys

binPath = "./dist"


def build_orange_core():
    os.system("mvn clean")
    os.system("mvn package")
    binPath = "./dist"
    shutil.copyfile("./target/orange_core.exe", binPath + "/lib/orange_core.exe")
    shutil.copytree("./src/main/resources/ik", binPath + "/.orange/conf/ik")


def clear_dist():
    if os.path.exists(binPath):
        shutil.rmtree(binPath, True)
    os.mkdir(binPath)
    os.mkdir(binPath + "/lib")


def build_orange_ui():
    os.chdir("./src/main/webapp")
    os.system("yarn run build-js")
    os.system("yarn run build-binary")
    shutil.copyfile("./src-tauri/target/release/orange_ui.exe", "./../../../dist/orange.exe")


def build_fsevent(cwd):
    os.chdir(cwd)
    os.chdir("src/main/py")
    os.system("python -m nuitka --standalone main.py")
    binPath = "../../../dist/lib"
    shutil.copyfile("./main.dist/main.exe", binPath + "/fsevent.exe")
    shutil.copyfile("./main.dist/libffi-7.dll", binPath + "/libffi-7.dll")
    shutil.copyfile("./main.dist/python38.dll", binPath + "/python38.dll")
    shutil.copyfile("./main.dist/_ctypes.pyd", binPath + "/_ctypes.pyd")
    shutil.rmtree("./main.build", True)
    shutil.rmtree("./main.dist", True)


if __name__ == '__main__':
    cwd = os.getcwd()
    clear_dist()
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    build_orange_core()
    build_orange_ui()
    build_fsevent(cwd)
