extends MenuButton

static var configureProjectId: int = 10000001
static var exportId: int = 10000002

var original_serialize_func = null
var plugin = null
@onready var fileDialog = $FileDialog
var configureSetupDialog = preload("res://mm_plugin/conduct/ui/configure_setup/configure_setup.tscn")
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
			
		if node.title == "Vessel Export":
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
	plugin.vessel_api.set_manifest(file)
	var summary = plugin.vessel_api.get_summary()
	print(summary)
	var popup = configureSetupDialog.instantiate() as Popup
	popup.summary = summary
	add_child(popup)
	popup.popup_centered()
	var result = await popup.done
	
	print(result)
	
	var dept = result[0]
	var asset = result[1]
	var sequence = result[2]
	var shot = result[3]
	
	var setup_result = plugin.vessel_api.setup(dept, asset, sequence, shot)
	print(setup_result)
	
	var globals = get_node("/root/mm_globals")
	var project = globals.main_window.get_current_project()

	project.top_generator.plugin_data["com.lagmachine.vessel"] = {
		"department": dept,
		"asset": asset,
		"sequence": sequence,
		"shot": shot
	}
	
	project.save_file(setup_result["file_path"] + "/" + asset + ".ptex")
