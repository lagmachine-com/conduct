import hou
from . import conduct


# Not sure about the efficiency
def get_conduct_data_node() -> hou.Node:
    all_nodes = hou.node("/obj").allNodes()
    data_node = None
    for node in all_nodes:
        if node.type().name() == "LAGMACHINE::Conduct" and node.name() == "Conduct":
            data_node = node
            break

    if data_node is None:
        data_node: hou.Node = hou.node(
            "/obj").createNode("LAGMACHINE::Conduct")
        data_node.setName("Conduct")
        data_node.setPosition((0, 0))

    return data_node


def get_conduct_object(manifest_path=None) -> conduct.Conduct:
    if manifest_path != None:
        return conduct.get_from_manifest_path(manifest_path, "houdini")
    else:
        return conduct.find_from_current_path(hou.hipFile.path(), "houdini")


def select_project():
    file_path = hou.ui.selectFile(
        start_directory=hou.hipFile.path(),
        title="Select a project",
        pattern="*.yml,*.yaml",
        chooser_mode=hou.fileChooserMode.Read
    )

    return file_path
