from sys import stdout
import sys
import os
import subprocess
import json

def log(info):
    print(info)

class Conduct:
    conduct_exe = ""
    current_program = ""

    def __init__(self, conduct_exe, current_program):
        self.conduct_exe = conduct_exe
        self.current_program = current_program

    def run_process(self, args):

        exe = self.conduct_exe
        if "CONDUCT_EXE" in os.environ:
            exe = os.environ["CONDUCT_EXE"]

        args = [exe] + args

        #Hide the cmd window on windows
        startupinfo = None

        creation_flags = 0
        if os.name == 'nt':
            startupinfo = subprocess.STARTUPINFO()
            startupinfo.dwFlags = subprocess.CREATE_NO_WINDOW
            creation_flags = subprocess.CREATE_NO_WINDOW
 
        env = os.environ.copy()

        # Fix errors where conduct tries to inherit env from the host app (occurred in inkscape appimage)
        for name in [
            "LD_LIBRARY_PATH",
            "LD_PRELOAD",
            "GDK_PIXBUF_MODULE_FILE",
            "GDK_PIXBUF_MODULEDIR",
            "GTK_PATH",
            "GTK_DATA_PREFIX",
            "GTK_EXE_PREFIX",
            "GIO_EXTRA_MODULES",
            "PYTHONPATH",
            "PYTHONHOME",
        ]:
            env.pop(name, None)


        cwd = None
        if "CONDUCT_ROOT" in os.environ:
            cwd = os.environ["CONDUCT_ROOT"]
        else:
            os.path.dirname(self.conduct_exe)

        log("Running process: " + str(args))
        process=subprocess.Popen(args, cwd=cwd, startupinfo=startupinfo, stdout=subprocess.PIPE, stderr=subprocess.PIPE, encoding='utf-8', creationflags=creation_flags, env=env)
        res = process.communicate()
        code = process.returncode
        data = res[0]
        err = res[1]
        log(data)

        return json.loads(data)

    def get_summary(self):
        summary = self.run_process(["summary"])
        return summary

    def setup(self, file_format):
        log("Running process")
        args = ["dialog", "create_setup", "--", "--file-format", file_format]
        return self.run_process(args)

    def dialog_load_asset(self, department, shot=None, asset=None ):
        args = ["dialog", "load_asset", "--", "--program", self.current_program, "--department", department]
        if shot != None and shot != "":
            args.append("--shot")
            args.append(shot)
        
        if asset != None and asset != "":
            args.append("--asset")
            args.append(asset)

        return self.run_process(args)
    
    def load_asset(self, asset, department, shot=None, ):
        args = ["load-assets", "--program", self.current_program, asset, "--department", department]
        if shot != None and shot != "":
            args.append("--shot")
            args.append(shot)

        return self.run_process(args)

    def dialog_export(self, department, asset, items, prev_state=None, shot=None,  ):
        args = ["dialog", "export", "--", "--program", self.current_program, "--department", department, "--asset", asset, "--items", items ]
        if shot != None and shot != "":
            args.append("--shot")
            args.append(shot)

        if prev_state != None and prev_state != "":
            args.append("--prev-state")
            args.append(prev_state)
        
        return self.run_process(args)
    
    def list_export_formats(self, department):
        return self.run_process(["list-export-formats", "--from", self.current_program, "--department", department])
    
    def export(self, department, format, asset, element, shot=None):
        args = ["export", "--department", department, "--file-format", format, "--from", self.current_program, "--asset", asset, "--element", element]
        if shot != None and shot != "":
            args.append("--shot")
            args.append(shot)

        return self.run_process(args)

def can_load_from_env():
    return "CONDUCT_MANIFEST" in os.environ

def load_from_env(current_program):
    return Conduct("conduct", current_program)

def get_from_manifest_path(manifest_path, current_program):
    log("Getting exe from manifest path: " + manifest_path)

    dir_path = os.path.dirname(manifest_path)
    exe = "conduct"
    if os.name == "nt":
        exe += ".exe"

    path = os.path.join(dir_path, exe)
    return Conduct(path, current_program)

def find_from_current_path(current_file, current_program):
    log("Looking for conduct path for file: " + current_file)
    path = os.path.dirname(current_file)
    while path != "":
        checks = [
            os.path.join(path, "manifest.yaml"),
            os.path.join(path, "manifest.yml")
        ]

        for check in checks:
            if os.path.isfile(check):
                log("found:" + check)
                return get_from_manifest_path(check, current_program)
            
        path = os.path.dirname(path)

            

    
    