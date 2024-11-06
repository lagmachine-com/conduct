from sys import stdout
import sys
import os
import subprocess
import json

class Conduct:
    conduct_exe = ""

    def __init__(self, conduct_exe):
        self.conduct_exe = conduct_exe

    def run_process(self, args):
        args = [self.conduct_exe] + args

        #Hide the cmd window on windows
        startupinfo = None

        creation_flags = 0
        if os.name == 'nt':
            startupinfo = subprocess.STARTUPINFO()
            startupinfo.dwFlags = subprocess.CREATE_NO_WINDOW
            creation_flags = subprocess.CREATE_NO_WINDOW
        
        print("Executing: " + str(args))

        process=subprocess.Popen(args, cwd=os.path.dirname(self.conduct_exe), startupinfo=startupinfo, stdout=subprocess.PIPE, encoding='utf-8', creationflags=creation_flags)

        data = process.communicate()[0]
        print(data)

        return json.loads(data)

    def get_summary(self):
        summary = self.run_process(["summary"])
        return summary

    def setup(self):
        args = ["dialog", "create_setup"]

        return self.run_process(args)


def get_from_manifest_path(manifest_path):
    print("Getting exe from manifest path: " + manifest_path)

    dir_path = os.path.dirname(manifest_path)
    exe = "conduct"
    if os.name == "nt":
        exe += ".exe"

    path = os.path.join(dir_path, exe)
    return Conduct(path)

def find_from_current_path(current_file):
    print("Looking for conduct path for file: " + current_file)
    path = os.path.dirname(current_file)
    while path != "":
        checks = [
            os.path.join(path, "manifest.yaml"),
            os.path.join(path, "manifest.yml")
        ]

        for check in checks:
            if os.path.isfile(check):
                print("found:" + check)
                return get_from_manifest_path(check)
            
        path = os.path.dirname(path)

            

    
    
