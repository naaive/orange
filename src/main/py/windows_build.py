import os
import shutil

if __name__ == '__main__':
    os.system("python -m nuitka --standalone main.py")
    binPath = "../../../bin"
    if os.path.exists(binPath):
        shutil.rmtree(binPath, True)
    os.makedirs(binPath)

    shutil.copyfile("./main.dist/main.exe", binPath + "/fsevent.exe")
    shutil.copyfile("./main.dist/libffi-7.dll", binPath + "/libffi-7.dll")
    shutil.copyfile("./main.dist/python38.dll", binPath + "/python38.dll")
    shutil.copyfile("./main.dist/_ctypes.pyd", binPath + "/_ctypes.pyd")
    shutil.rmtree("./main.build", True)
    shutil.rmtree("./main.dist", True)
