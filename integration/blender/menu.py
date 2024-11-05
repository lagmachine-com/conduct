import bpy
from . import utils

class ConductMenu(bpy.types.Menu):
    bl_label = "Conduct"
    bl_idname = "OBJECT_MT_conduct_menu"

    def draw(self, context):
        layout = self.layout
        data = utils.get_conduct_data()

        if data == None or data.project == None or data.project == "":
            layout.operator("conduct.select_project", icon='ADD', text="Select Project")

def draw_item(self, context):
    layout = self.layout
    layout.menu(ConductMenu.bl_idname)


def register():
    bpy.utils.register_class(ConductMenu)

    # lets add ourselves to the main header
    bpy.types.TOPBAR_MT_editor_menus.append(draw_item)


def unregister():
    bpy.utils.unregister_class(ConductMenu)

    bpy.types.TOPBAR_MT_editor_menus.remove(draw_item)