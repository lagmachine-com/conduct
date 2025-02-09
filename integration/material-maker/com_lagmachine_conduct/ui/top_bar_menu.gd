extends MenuButton

static var configureProjectId: int = 10000001
static var exportId: int = 10000002

var original_serialize_func = null
var plugin = null
@onready var fileDialog = $FileDialog
@onready var mm = get_node("/root/mm_plugins/com_lagmachine_conduct")



# Called when the node enters the scene tree for the first time.
func _ready():
	get_popup().add_item("Configure Project", configureProjectId)
	get_popup().add_item("Export", exportId)
	get_popup().id_pressed.connect(on_item_pressed)
	fileDialog.file_selected.connect(onFileSelected)
	
	if plugin == null:
		plugin = get_parent()

func on_item_pressed(id):

	if id == configureProjectId:
		configureProject()
		
	if id == exportId:
		export_all()

func export_all():
	var globals = get_node("/root/mm_globals")
	var project = globals.main_window.get_current_project()
	var path = project.save_path
	var result = plugin.vessel_api.find_vessel_exe(path)


	var nodes = project.get_children()
	var export_nodes = []
	
	for node in nodes:
		if not "title" in node:
			continue
			
		if node.title == "Conduct Export":
			export_nodes.append(node)
	
	for node in export_nodes:
		if node.selected:
			await export(node, project)
		else:
			print("Skipping node because not selected: ", node)

func export(export_node, project):
	
	var data = project.top_generator.plugin_data["com.lagmachine.vessel"]
	var dept = data["department"]
	var asset = data["asset"]
	var seq = data["sequence"]
	var shot = data["shot"]
	var element = export_node.controls.element.text
	var nodes = project.get_children()
	
	
	for n in nodes:
		print(n)
		if not n is MMGraphNodeRemote:
			continue

		for key in n.controls:
			var control = n.controls[key]
			if not control is OptionButton:
				continue
			for i in range(control.item_count):
				if control.get_item_text(i) == element:
					n.generator.set_parameter(key, i)
	
	var export = plugin.vessel_api.export(".png", dept, asset, element, seq, shot)
	var script_file = FileAccess.open(export["script"], FileAccess.READ)
	var code = script_file.get_as_text()

	var script = GDScript.new()
	script.source_code = code
	script.reload()
	await script.export(plugin, export_node, export["file_path"], export["recommended_file_name"])
	
func configureProject():
	fileDialog.popup_centered()
	
func onFileSelected(file):
	print(file)
	plugin.conduct_api.set_manifest(file)

	var result = plugin.conduct_api.setup(".ptex")
	
	if result['result'] != 'ok':
		print("Something went wrong!")
		return
		
	var data = result['data']
		

	var dept = data['department']
	var asset = data['asset']
	var shot = data['shot']
	
	var folder = data['folder']
	var file_name = data['file_name']


	
	var globals = get_node("/root/mm_globals")
	var project = globals.main_window.get_current_project()

	project.top_generator.plugin_data["com.lagmachine.conduct"] = {
		"department": dept,
		"asset": asset,
		"shot": shot
	}
	
	project.save_file(folder + "/" + file_name + ".ptex")
