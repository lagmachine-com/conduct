
import bpy
from bpy.types import Operator
from . import utils
import inspect

class OT_AddObjectToPublish(Operator):

    bl_idname = "conduct.add_object_to_export"
    bl_label = "Add Object"

    def execute(self, context):
        data = utils.get_conduct_data()
        
        selected_index = data.export_index
        export = data.exports[selected_index]
        export.items.add()
        return {'FINISHED'}

class OT_RemoveObjectFromPublish(Operator):

    bl_idname = "conduct.remove_object_from_export"
    bl_label = "Remove Object"

    def execute(self, context):
        data = utils.get_conduct_data()
        selected_index = data.export_index
        export = data.exports[selected_index]
        object_index = data.export_collection_index
        export_col = export.items[object_index]
        export.items.remove(object_index)
        return {'FINISHED'}


class OT_RemovePublish(Operator):

    bl_idname = "conduct.remove_export"
    bl_label = "Remove Publish"

    def execute(self, context):
        data = utils.get_conduct_data()
        publish = data.exports.remove(data.export_index)
        return {'FINISHED'}

class OT_CreatePublish(Operator):

    bl_idname = "conduct.create_export"
    bl_label = "Create Export"

    def execute(self, context):
        data = utils.get_conduct_data()
        publish = data.exports.add()
        publish.name = "New Export"
        return {'FINISHED'}

class OT_RunSelectedExport(Operator):

    bl_idname = "conduct.run_selected_export"
    bl_label = "Run Export"

    def execute(self, context):
        data = utils.get_conduct_data()
        export = data.exports[data.export_index]
        name = export.name
        items = [c.value for c in export.items]

        conduct = utils.get_conduct_object()
        result = conduct.export(data.department, export.format, data.asset, name, data.shot)

        locals = {}
        globals = {}
        script = result['script']
        exec(script, locals, globals)

        for item in globals:
            instance = globals[item]

            if not inspect.isclass(instance):
                continue

            exporter_instance = instance()

            exporter_instance.export(
                directory=result['directory'], 
                file_name=result['recommended_file_name'], 
                extension=result['file_format'],
                items = items
                )
        
        self.report({'INFO'}, "Exported %s!" % name)

        return {'FINISHED'}

def register():
    bpy.utils.register_class(OT_CreatePublish)
    bpy.utils.register_class(OT_RemovePublish)
    bpy.utils.register_class(OT_AddObjectToPublish)
    bpy.utils.register_class(OT_RemoveObjectFromPublish)
    bpy.utils.register_class(OT_RunSelectedExport)

def unregister():
    bpy.utils.unregister_class(OT_RunSelectedExport)
    bpy.utils.unregister_class(OT_RemoveObjectFromPublish)
    bpy.utils.unregister_class(OT_AddObjectToPublish)
    bpy.utils.unregister_class(OT_RemovePublish)
    bpy.utils.unregister_class(OT_CreatePublish)