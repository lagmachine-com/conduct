class BlenderDataExport():

    def export(self, directory=None, file_name=None, extension=None, items=None):
        import bpy
        import os

        print("Exporting alembic cache")
        for item in items:
            print(item)

        bpy.ops.object.select_all(action='DESELECT')
    
        collections = [item for item in items if isinstance(item, bpy.types.Collection)]
        objects = [item for item in items if isinstance(item, bpy.types.Object)]

        for collection in collections:
            for obj in collection.objects:
                obj.select_set(True)

        for obj in objects:
            obj.select_set(True)

        file = os.path.join(directory, file_name + extension)
        bpy.ops.wm.alembic_export(
            filepath=file, 
            check_existing=False,
            vcolors=True,
            selected=True,
            visible_objects_only=False,
        )
