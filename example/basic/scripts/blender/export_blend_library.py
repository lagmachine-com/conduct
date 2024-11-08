class BlenderDataExport():

    def export(self, directory=None, file_name=None, extension=None, items=None):
        import bpy
        import os

        # have to make paths absolute here, otherwise libraries.write gets confused with path_remap="RELATIVE_ALL"
        bpy.ops.file.make_paths_absolute()

        print("Exporting blend data!")
        for item in items:
            print(item)

        file = os.path.join(directory, file_name + extension)

        data = set(items)

        bpy.data.libraries.write(file, data, fake_user=True, path_remap="RELATIVE_ALL")

        bpy.ops.file.make_paths_relative()

        print("Exported: " + file)
