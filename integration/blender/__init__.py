# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTIBILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

bl_info = {
    "name" : "Conduct",
    "author" : "LAGMACHINE",
    "description" : "",
    "blender" : (2, 80, 0),
    "version" : (0, 0, 1),
    "location" : "",
    "warning" : "",
    "category" : "Generic"
}
from . import properties, menu, project_browser, select_project, export, export_menu, load_asset


def register():
    properties.register()
    export.register()
    select_project.register()
    menu.register()
    export_menu.register()
    project_browser.register()
    load_asset.register()


def unregister():
    load_asset.unregister()
    project_browser.unregister()
    export_menu.unregister()
    menu.unregister()
    select_project.unregister()
    export.unregister()
    properties.unregister()