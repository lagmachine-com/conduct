import bpy
import os
from .conduct import conduct
from . import properties

def get_conduct_data() -> properties.ConductProperties:
    for scene in bpy.data.scenes:
        if scene.conduct != None and scene.conduct.project != None and scene.conduct.project != "":
            return scene.conduct

    return None

def get_conduct_object(manifest_path = None) -> conduct.Conduct:
    if manifest_path == None:
        data = get_conduct_data()
        if data == None:
            return None

        manifest_path = data.project
    
    if os.name == "nt":
        manifest_path = manifest_path.replace("/", "\\")
    else:
        manifest_path = manifest_path.replace("\\", "/")

    manifest_path = os.path.join(bpy.data.filepath, manifest_path)
    manifest_path = os.path.realpath(manifest_path)
    dir = os.path.dirname(manifest_path)

    exe = os.path.join(dir, "conduct")
    if os.name == 'nt':
        exe += ".exe"

    return conduct.Conduct(exe)