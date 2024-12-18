import hou
from . import utils
import os


# Thinking about splitting this into multiple functions for better readability
def create_setup():
    path = utils.select_project()
    path = hou.expandString(path)

    setup_network: hou.Node = hou.node("/obj").createNode("subnet")
    setup_network.setName("setup")
    setup_network.setPosition((0, 0))

    import_network: hou.Node = hou.node("/obj").createNode("subnet")
    import_network.setName("import")
    setup_network.setPosition((0, -1))

    # Might using subnet instead of null, cause a null adds an extra object to the scene
    data_node: hou.Node = setup_network.createNode("null")
    data_node.setName("conduct")

    data_node.addSpareParmTuple(
        hou.StringParmTemplate("project", "Project", 1))
    data_node.addSpareParmTuple(
        hou.StringParmTemplate("department", "Department", 1))
    data_node.addSpareParmTuple(hou.StringParmTemplate("asset", "Asset", 1))
    data_node.addSpareParmTuple(hou.StringParmTemplate("shot", "Shot", 1))

    projectParm: hou.Parm = data_node.parm("project")
    projectParm.set(path)

    conduct = utils.get_conduct_object()

    result = conduct.setup()
    if result['result'] != 'ok':
        return {'FINISHED'}

    dialog_data = result['data']

    deptParm: hou.Parm = data_node.parm("department")
    deptParm.set(dialog_data['department'])

    assetParm: hou.Parm = data_node.parm("asset")
    assetParm.set(dialog_data['asset'])

    shot = dialog_data['shot']
    if shot is not None:
        shotParm: hou.Parm = data_node.parm("shot")
        shotParm.set(shot)

    # We have to discuss a properway of doing this. Houdini does not support the \\?\ prefix. Only tested on Windows
    file_name = os.path.join(dialog_data['path'][4:] if dialog_data['path'].startswith(
        r"\\?") else dialog_data['path'], dialog_data['file_name'] + ".hip")

    hou.hipFile.save(file_name)
