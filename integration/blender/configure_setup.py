from bpy.types import Operator
from bpy.props import StringProperty
import bpy
import json
from . import utils
import os

class OT_ConfigureSetup(Operator):

    bl_idname = "conduct.configure_setup"
    bl_label = "Configure Setup"

    def execute(self, context):

        data = utils.get_conduct_data()
        if data == None:
            data = bpy.data.scenes[0].conduct

        conduct = utils.get_conduct_object()
        
        result = conduct.setup(".blend")
        if result['result'] != 'ok':
            return {'FINISHED'}
        
        dialog_data = result['data']

        data.asset = dialog_data['asset']
        data.department = dialog_data['department']
        
        if dialog_data['shot'] is not None:
            data.shot = dialog_data['shot']

        bpy.ops.wm.save_as_mainfile(filepath=dialog_data['path'])
        self.report({'INFO'}, "Saved Setup!")

        return {'FINISHED'}

def register():
    bpy.utils.register_class(OT_ConfigureSetup)

def unregister():
    bpy.utils.unregister_class(OT_ConfigureSetup)
