import bpy
from . import utils

class ObjectItemSlot(bpy.types.UIList):
    def draw_item(self, context, layout, data, item, icon, active_data, active_propname):
        item.draw(layout, context)


def draw_publish(layout, context):
    data = utils.get_conduct_data()

    if data == None:
        return

    index = data.export_index

    row = layout.row()

    row.template_list(
        "TaskItemSlot",
        "conduct_select_export_list",
        data,
        "exports",
        data,
        "export_index"
    )

    col = row.column(align=True)
    col.operator("conduct.create_export", icon='ADD', text="")
    col.operator("conduct.remove_export", icon='REMOVE', text="")


    if index >= len(data.exports):
        return

    publish = data.exports[index]

    layout.label(text=publish.name)

    layout.operator("conduct.run_selected_export", icon='FILE_TICK', text="Run Selected")
    
    layout.prop(publish, 'format')
    
    layout.label(text="Objects:")

    row = layout.row()

    row.template_list(
        "ObjectItemSlot",
        "conduct_select_export_collection_list",
        publish,
        "items",
        data,
        "export_collection_index"
    )

    col = row.column(align=True)
    col.operator("conduct.add_object_to_export", icon='ADD', text="")
    col.operator("conduct.remove_object_from_export", icon='REMOVE', text="")

class PublishPanel(bpy.types.Panel):
    bl_label = "Export (Conduct)"
    bl_idname = "SCENE_PT_Publish"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "scene"

    def draw(self, context):
        draw_publish(self.layout, context)

def register():
    bpy.utils.register_class(ObjectItemSlot)
    bpy.utils.register_class(PublishPanel)

def unregister():
    bpy.utils.unregister_class(ObjectItemSlot)
    bpy.utils.unregister_class(PublishPanel)