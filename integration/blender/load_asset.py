import bpy
import json
from . import utils
import os
import inspect

def can_add_to_collection(item):
    if isinstance(item, bpy.types.Collection):
        return True
    
    if isinstance(item, bpy.types.Object):
        return True

def make_collection(item, asset, element):
    name = "IN_" + asset + "_" + element
    collection = bpy.data.collections.new(name)

    if collection.name != name:
        old_collection = bpy.data.collections[name]
        old_collection.name = name + "_temp"
        collection.name = name
        
        old_collection.user_remap(collection)
        bpy.data.collections.remove(old_collection)
    
    if isinstance(item, bpy.types.Collection):
        collection.children.link(item)
    if isinstance(item, bpy.types.Object):
        collection.objects.link(item)

    return collection

def load(results):
    scripts = {}
        
    for entry in results:
        script_file = entry["script"]
        script_data = open(script_file).read()

        locals = {}
        globals = {}
        exec(script_data, locals, globals)

        for item in globals:
            instance = globals[item]

            if not inspect.isclass(instance):
                continue

            importer_instance = instance()
            scripts[script_file] = importer_instance

    file_to_element = {}
    asset_to_files = {}
    for entry in results:
        file = entry['file']
        asset = entry['asset']
        element = entry['element']
        if not asset in asset_to_files:
            asset_to_files[asset] = []
            
        asset_to_files[asset].append(file)
        importer = scripts[entry['script']]
        
        importer.asset = asset
        importer.element = element

        result = importer.load(file)

        result = {
            "data": result,
            "metadata": entry
        }

        if result != None:
            file_to_element[file] = result

    print("Loaded elements:")
    print(file_to_element)

    print("Asset to file:")
    print(asset_to_files)

    print ("Apply Stage")
    for entry in results:
        print("--------")

        file = entry['file']
        asset = entry['asset']
        element = entry['element']
        importer = scripts[entry['script']]

        print("Asset: " + asset)
        print("Element: " +  element)

        if not hasattr(importer, "apply"):
            continue

        elements = [file_to_element[x] for x in asset_to_files[asset] if x in file_to_element]

        this_element = None
        if file in file_to_element:
            this_element = file_to_element[file]
            
        if this_element != None:
            elements.remove(this_element)

        for element in elements:
            info = "  Applying to: " + element['metadata']['asset'] + "/" + element['metadata']['element']
            if hasattr(importer, "should_apply"):
                if importer.should_apply(this_element, element):
                    print(info)
                    importer.apply(this_element, element)

            else:
                print(info)
                importer.apply(this_element, element)
        
    print("Finished Applications")
    
    # Workaround for https://github.com/lagmachinery/conduct/issues/18
    bpy.ops.file.make_paths_absolute()

    for entry in file_to_element.values():
        data = entry['data']
        metadata = entry['metadata']
            
        asset = metadata['asset']
        element = metadata['element']
        importer = scripts[metadata['script']]

        if can_add_to_collection(data):
            data = make_collection(data, asset, element)
            
        if hasattr(importer, 'asset_library') and importer.asset_library:
            # Note, this fails if we are trying to mark linked asset data
            data.asset_mark()
            data.asset_generate_preview()

            if data.asset_data != None:
                data.asset_data.tags.new('asset:' + asset)
                data.asset_data.tags.new('element:' + element)


class OT_ImportAsset(bpy.types.Operator):

    bl_idname = "conduct.load_asset"
    bl_label = "Import Asset(s)"

    def execute(self, context):
        """Do something with the selected file(s)."""

        data = utils.get_conduct_data()
        conduct = utils.get_conduct_object()

        result = conduct.dialog_load_asset(data.department, data.shot, data.asset)

        if result['result'] != 'ok':
            return {'FINISHED'}
        
        results = result['data']['results']
        load(results)
        
        print("Finished Importing Assets")
        return {'FINISHED'}


def register():
    bpy.utils.register_class(OT_ImportAsset)

def unregister():
    bpy.utils.unregister_class(OT_ImportAsset)
