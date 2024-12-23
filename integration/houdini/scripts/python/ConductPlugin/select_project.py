import hou
from . import utils
import os


def create_setup():
    path = utils.select_project()
    if path == "":
        return {'FINISHED'}
    path = hou.expandString(path)

    if hou.licenseCategory() != hou.licenseCategoryType.Commercial:
        fileExtension = ".hiplc" if hou.licenseCategory(
        ) == hou.licenseCategoryType.Indie else ".hipnc"
    else:
        fileExtension = ".hip"

    conduct = utils.get_conduct_object(path)

    result = conduct.setup(fileExtension)
    if result['result'] != 'ok':
        return {'FINISHED'}

    data_node = utils.get_conduct_data_node()

    pSelectedParm: hou.Parm = data_node.parm("project_selected")
    pSelectedParm.set(True)

    projectParm: hou.Parm = data_node.parm("project")
    projectParm.set(path)

    dialog_data = result['data']

    deptParm: hou.Parm = data_node.parm("department")
    deptParm.set(dialog_data['department'])

    assetParm: hou.Parm = data_node.parm("asset")
    assetParm.set(dialog_data['asset'])

    shot = dialog_data['shot']
    if shot is not None:
        shotParm: hou.Parm = data_node.parm("shot")
        shotParm.set(shot)

    setattr(hou.session, "projectImported", True)
    hou.hipFile.save(file_name=os.path.join(dialog_data['path']))
