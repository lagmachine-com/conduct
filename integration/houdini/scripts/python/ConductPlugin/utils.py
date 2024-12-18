import os
import hou
from . import conduct


def get_conduct_data_node() -> hou.Node:
    return hou.node("/obj/setup/conduct")


def get_conduct_object() -> conduct.Conduct:
    data = get_conduct_data_node()
    projectParm: hou.Parm = data.parm("project")
    dir = os.path.dirname(projectParm.evalAsString())
    exe = os.path.join(dir, "conduct.exe")

    return conduct.Conduct(exe, "houdini")


def select_project():
    file_path = hou.ui.selectFile(
        start_directory=hou.hipFile.path(),
        title="Select a project",
        pattern="*.yml,*.yaml",
        chooser_mode=hou.fileChooserMode.Read
    )

    return file_path
