from bpy_extras.io_utils import ImportHelper
from bpy.types import Operator
from bpy.props import StringProperty
import bpy
import json
from . import utils
import os

class OT_SelectProject(Operator, ImportHelper):

    bl_idname = "conduct.select_project"
    bl_label = "Select a project"

    filter_glob: StringProperty(
        default='*.yml;*.yaml',
        options={'HIDDEN'}
    )

    def execute(self, context):
        """Do something with the selected file(s)."""

        data = utils.get_conduct_data()
        if data == None:
            data = bpy.data.scenes[0].conduct

        conduct = utils.get_conduct_object(self.filepath)
        
        result = conduct.setup()
        if result['result'] != 'ok':
            return {'FINISHED'}
        
        dialog_data = result['data']

        data.asset = dialog_data['asset']
        data.department = dialog_data['department']
        
        if dialog_data['shot'] is not None:
            data.shot = dialog_data['shot']

        path = os.path.join(dialog_data['path'], dialog_data['file_name'] + ".blend")
        bpy.ops.wm.save_as_mainfile(filepath=path)
        self.report({'INFO'}, "Saved Setup!")

        return {'FINISHED'}

def register():
    bpy.utils.register_class(OT_SelectProject)

def unregister():
    bpy.utils.unregister_class(OT_SelectProject)
