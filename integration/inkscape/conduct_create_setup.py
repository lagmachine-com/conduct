import inkex
from inkex import command
import os 
from conduct import conduct
import subprocess

class ConductCreateSetup(inkex.EffectExtension):

    def add_arguments(self, pars):
        pars.add_argument("-m", "--manifest", default="", help="Manifest File Path")

    def effect(self):
        path = self.options.manifest
        c = conduct.get_from_manifest_path(path, "inkscape")
        result = c.setup()

        if result['result'] != 'ok':
            return
        
        dialog_data = result['data']
        path = os.path.join(dialog_data['path'], dialog_data['file_name'] + ".svg")

        self.svg.set("com.lagmachine.conduct.asset", dialog_data['asset'])
        self.svg.set("com.lagmachine.conduct.department", dialog_data['department'])
        if dialog_data['shot'] is not None:
            self.svg.set("com.lagmachine.conduct.shot", dialog_data['shot'])

        # write to new file, and open it in a new instance of inkscape
        # this is the best i can do, there is no function for changing the current file to a different location
        data = self.svg.tostring().decode('utf-8')

        with open(path, mode='w') as f:
            f.write(data)

        exe = inkex.command.which('inkscape')

        #if we could find a good way to kill the original inkscape instance after starting the new one, that would be ideal
        if os.name == 'nt':
            creation_flags = subprocess.CREATE_NEW_PROCESS_GROUP | subprocess.DETACHED_PROCESS
            proc = subprocess.Popen([exe, path], creationflags=creation_flags, start_new_session=True)
        else:
            proc = subprocess.Popen([exe, path], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, stdin=subprocess.DEVNULL)
        
if __name__ == '__main__':
    ConductCreateSetup().run()