
class BlenderDataImport():
    asset_library = True

    def load(self, file):
        import bpy
        
        with bpy.data.libraries.load(file, link=True, relative=True, assets_only=True) as (data_from, data_to):
            data_to.collections = data_from.collections

        return data_to.collections[0]