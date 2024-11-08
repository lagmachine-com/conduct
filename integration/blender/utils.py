import bpy
import os
from .conduct import conduct
from . import properties

def get_conduct_data() -> properties.ConductProperties:
    for scene in bpy.data.scenes:
        if scene.conduct != None and scene.conduct.asset != None and scene.conduct.asset != "":
            return scene.conduct

    return None

def get_conduct_object(manifest_path = None) -> conduct.Conduct:
    if manifest_path != None:
        return conduct.get_from_manifest_path(manifest_path, "blender")
    else:
        return conduct.find_from_current_path(bpy.data.filepath, "blender")
    
