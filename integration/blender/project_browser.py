import bpy

from . import utils

class TaskItemSlot(bpy.types.UIList):
    def draw_item(self, context, layout, data, item, icon, active_data, active_propname):

        if item.icon == None or item.icon == "":
            layout.prop(item, "name", text="", emboss=False)
        else:
            layout.prop(item, "name", text="", icon=item.icon, emboss=False)

class ProjectBrowser(bpy.types.Panel):
    bl_label = "Conduct"
    bl_category = "Conduct"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"

    def draw(self, context):
        layout = self.layout

        layout = self.layout
        scene = context.scene
        
        layout.use_property_split = True
        layout.use_property_decorate = False

        data = utils.get_conduct_data()

        if data == None or data.asset == None or data.asset == "":
            layout.operator("conduct.select_project", icon='ADD', text="Select Project")
            return

        if data.department != "":
            layout.label(text="Department: " + data.department)
        
        if data.asset != "":
            layout.label(text="Asset: " + data.asset)

def register():
    bpy.utils.register_class(TaskItemSlot)
    
    bpy.utils.register_class(ProjectBrowser)

def unregister():
    bpy.utils.unregister_class(ProjectBrowser)
    
    bpy.utils.unregister_class(TaskItemSlot)