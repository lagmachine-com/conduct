import inkex
from inkex import command
import os 
from conduct import conduct
import subprocess
import json
import inspect

def log_stub(info):
    pass

def log(info):
    inkex.utils.debug(info)

class SvgReader(inkex.extensions.InputExtension):
    pass

class ConductExportEffect(inkex.EffectExtension):

    def effect(self):
        conduct.log = log_stub

        file_path = os.path.join(self.svg_path(), self.svg.name)
        department = self.svg.get("com.lagmachine.conduct.department")
        asset = self.svg.get("com.lagmachine.conduct.asset")

        prev_state = self.svg.get("com.lagmachine.conduct.export_save_state")
        if prev_state != None:
            prev_state = json.loads(prev_state)
        
        c = conduct.find_from_current_path(file_path, "inkscape")

        # We arent using get_pages because this is more reliable
        # get_pages result doesnt contain the page label if there is only one page
        children = self.svg.namedview.getchildren()
        pages = [child for child in children if child.tag_name == "inkscape:page"]

        if len(pages) == 0:
            # this function replaces the default document with a page, so we create a temp page then remove it
            self.svg.namedview.new_page("0", "0", "10", "10", "temp_page")

            children = self.svg.namedview.getchildren()
            pages = [child for child in children if child.tag_name == "inkscape:page"]

            self.svg.namedview.remove(pages[1])
            pages[0].label = "Page 1"

            children = self.svg.namedview.getchildren()
            pages = [child for child in children if child.tag_name == "inkscape:page"]


        # only allow export of named pages
        pages = [page for page in pages if page.label != None]
        items = ','.join([page.label for page in pages])
        
        result = c.dialog_export(department, asset, items, prev_state=prev_state)

        if result['result'] != 'ok':
            return

        save_state = result['data']['save_state']
        self.svg.set("com.lagmachine.conduct.export_save_state", save_state)
        
        for export in result['data']['exports']:
            items = export['items']
            export_data = export['result']
            if 'error' in export_data:
                log(export_data['error'])
                return
            export_pages = [page for page in pages if page.label in items]
            
            locals = {}
            globals = {}
            script = export_data['script']
            exec(script, locals, globals)

            for item in globals:
                instance = globals[item]

                if not inspect.isclass(instance):
                    continue

                exporter_instance = instance()

                exporter_instance.export(
                    self,
                    directory=export_data['directory'], 
                    file_name=export_data['recommended_file_name'], 
                    extension=export_data['file_format'],
                    items = export_pages
                    )

        return

        
if __name__ == '__main__':
    ConductExportEffect().run()
