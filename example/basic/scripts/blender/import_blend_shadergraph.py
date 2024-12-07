class BlenderDataImport():
    def load(self, file):
        import bpy
        
        with bpy.data.libraries.load(file, link=True, relative=True, assets_only=True) as (data_from, data_to):
            print(dir(data_from))
            data_to.node_groups = data_from.node_groups

        return data_to.node_groups[0]

    def should_apply(self, this_element, other_element):
        return other_element['metadata']['file_type'] in ['.mesh.blend']

    def apply(self, this_element, other_element):
        import json
        graph = this_element['data']
        collection = other_element['data']

        def apply_to_collection(collection):
            collection = collection.override_create(remap_local_usages=True)
            for obj in collection.objects:
                obj = obj.override_create(remap_local_usages=True)
                mod = obj.modifiers.new(name="Apply Shadergraph", type='NODES')
                mod.node_group = graph
                print("Applied " + str(graph) + " to: " + str(obj))
            
            for child in collection.children:
                if len(child.objects) > 0:
                    apply_to_collection(child)

            return collection

        other_element['data'] = apply_to_collection(collection)