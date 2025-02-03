import inkex
from inkex import command
import os 
from conduct import conduct
import subprocess

def log_stub(info):
    pass

def log(info):
    inkex.utils.debug(info)

class SvgReader(inkex.extensions.InputExtension):
    pass

class ConductCreateSetup(inkex.EffectExtension):

    def effect(self):
        conduct.log = log_stub

        file_path = os.path.join(self.svg_path(), self.svg.name)
        department = self.svg.get("com.lagmachine.conduct.department")

        c = conduct.find_from_current_path(file_path, "inkscape")
        formats = c.list_export_formats(department)

        log(file_path)
        log(formats)
        return

        
if __name__ == '__main__':
    ConductCreateSetup().run()